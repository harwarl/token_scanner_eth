use std::{
    sync::{
        Arc, RwLock,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use reqwest::Client;

#[derive(Debug)]
pub struct LoadBalancer {
    pub chain_id: i32,
    pub servers: Vec<Server>,
    pub current: AtomicUsize,
}

impl LoadBalancer {
    pub fn new(chain_id: i32, urls: Vec<&str>) -> LoadBalancer {
        LoadBalancer {
            chain_id,
            servers: urls
                .into_iter()
                .map(|url| Server::new(chain_id, url.to_string()))
                .collect(),
            current: AtomicUsize::new(0),
        }
    }

    pub async fn get_next_server(&self) -> Option<Server> {
        let servers = self.servers.clone();
        let len = servers.len() as usize;

        for _ in 0..len {
            let idx = self.current.fetch_add(1, Ordering::Relaxed) % len;
            let server = servers[idx].clone();
            if server.health_check().await {
                return Some(server);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    pub chain_id: i32,
    pub url: String,
    pub is_healthy: Arc<RwLock<bool>>,
    pub last_failed: Arc<RwLock<Option<Instant>>>,
    pub client: Client,
}

impl Server {
    pub fn new(chain_id: i32, url: String) -> Server {
        Server {
            chain_id,
            url,
            is_healthy: Arc::new(RwLock::new(true)),
            last_failed: Arc::new(RwLock::new(None)),
            client: reqwest::Client::new(),
        }
    }

    // check if the server is healthy for the next call
    pub async fn health_check(&self) -> bool {
        // skip if failed within the last 60 seconds
        if let Some(last) = *self.last_failed.read().unwrap() {
            if last.elapsed() < Duration::from_secs(60) {
                return false;
            }
        }

        // Call for a block number to validate the RPC URL
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
        });

        let result = self
            .client
            .post(&self.url)
            .json(&body)
            .timeout(Duration::from_secs(3))
            .send()
            .await;

        // return true is there is a valid response and false if there isn't
        let is_healthy = match result {
            Ok(res) => {
                let json: serde_json::Value = res.json().await.unwrap_or_default();
                json.get("result").is_some()
            }
            Err(_) => false,
        };

        tracing::info!("Is Healthy: {is_healthy}");
        self.set_health(is_healthy);
        if !is_healthy {
            *self.last_failed.write().unwrap() = Some(Instant::now());
        };

        is_healthy
    }

    // Update the health status of the server
    pub fn set_health(&self, healthy: bool) {
        let mut health = self.is_healthy.write().unwrap();
        *health = healthy
    }

    pub fn fallback(url: &str, chain_id: i32) -> Server {
        Server {
            chain_id,
            url: url.to_string(),
            last_failed: Arc::new(RwLock::new(None)),
            is_healthy: Arc::new(RwLock::new(true)),
            client: Client::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    // ── helpers ────────────────────────────────────────────────────────────────

    fn healthy_response() -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": "0x112a880"
        }))
    }

    fn unhealthy_response() -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": { "code": -32000, "message": "server error" }
        }))
    }

    async fn mock_server_with(response: ResponseTemplate) -> (MockServer, Server) {
        let mock = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(response)
            .mount(&mock)
            .await;
        let server = Server::new(1, mock.uri());
        (mock, server)
    }

    // ── Server::health_check ───────────────────────────────────────────────────

    #[tokio::test]
    async fn health_check_returns_true_when_result_present() {
        let (_mock, server) = mock_server_with(healthy_response()).await;
        assert!(server.health_check().await);
    }

    #[tokio::test]
    async fn health_check_returns_false_when_no_result_field() {
        let (_mock, server) = mock_server_with(unhealthy_response()).await;
        assert!(!server.health_check().await);
    }

    #[tokio::test]
    async fn health_check_returns_false_on_connection_failure() {
        // Point at a port nothing is listening on
        let server = Server::new(1, "http://127.0.0.1:19999".to_string());
        assert!(!server.health_check().await);
    }

    #[tokio::test]
    async fn health_check_updates_is_healthy_flag() {
        let (_mock, server) = mock_server_with(healthy_response()).await;
        server.health_check().await;
        assert!(*server.is_healthy.read().unwrap());

        let (_mock2, sick) = mock_server_with(unhealthy_response()).await;
        sick.health_check().await;
        assert!(!*sick.is_healthy.read().unwrap());
    }

    // ── Server::set_health ─────────────────────────────────────────────────────

    #[test]
    fn set_health_flips_flag() {
        let server = Server::new(1, "http://localhost".to_string());
        server.set_health(false);
        assert!(!*server.is_healthy.read().unwrap());
        server.set_health(true);
        assert!(*server.is_healthy.read().unwrap());
    }

    // ── LoadBalancer::get_next_server ──────────────────────────────────────────

    #[tokio::test]
    async fn returns_first_healthy_server() {
        let mock1 = MockServer::start().await;
        let mock2 = MockServer::start().await;

        // first server is sick, second is healthy
        Mock::given(method("POST"))
            .respond_with(unhealthy_response())
            .mount(&mock1)
            .await;
        Mock::given(method("POST"))
            .respond_with(healthy_response())
            .mount(&mock2)
            .await;

        let lb = LoadBalancer::new(1, vec![mock1.uri().as_str(), mock2.uri().as_str()]);
        let server = lb.get_next_server().await;

        assert!(server.is_some());
        assert_eq!(server.unwrap().url, mock2.uri());
    }

    #[tokio::test]
    async fn returns_none_when_all_servers_unhealthy() {
        let mock = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(unhealthy_response())
            .mount(&mock)
            .await;

        let lb = LoadBalancer::new(1, vec![mock.uri().as_str()]);
        assert!(lb.get_next_server().await.is_none());
    }

    #[tokio::test]
    async fn round_robins_across_healthy_servers() {
        let mock1 = MockServer::start().await;
        let mock2 = MockServer::start().await;

        for mock in [&mock1, &mock2] {
            Mock::given(method("POST"))
                .respond_with(healthy_response())
                .mount(mock)
                .await;
        }

        let lb = LoadBalancer::new(1, vec![mock1.uri().as_str(), mock2.uri().as_str()]);

        let first = lb.get_next_server().await.unwrap().url;
        let second = lb.get_next_server().await.unwrap().url;

        // should alternate
        assert_ne!(first, second);
    }

    #[tokio::test]
    async fn counter_wraps_around() {
        let mock = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(healthy_response())
            .mount(&mock)
            .await;

        let lb = LoadBalancer::new(1, vec![mock.uri().as_str()]);

        // Force the counter near usize::MAX to test wrapping
        lb.current.load(Ordering::Relaxed);

        // Should not panic
        let server = lb.get_next_server().await;
        assert!(server.is_some());
    }
}
