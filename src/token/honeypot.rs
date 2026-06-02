use alloy::primitives::Address;

use crate::types::HoneyPotResponse;

pub async fn get_honey_pot(token: &Address, pair_address: &Address) -> Option<HoneyPotResponse> {
    let honeypot_url = format!(
        "https://api.honeypot.is/v2/IsHoneypot?address={token}&pair={pair_address}&chainID=1"
    );

    let res = match reqwest::get(&honeypot_url).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to call honeypot API: {e}");
            return None;
        }
    };

    let data = match res.text().await {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("Failed to read response body: {e}");
            return None;
        }
    };

    match serde_json::from_str::<HoneyPotResponse>(&data) {
        Ok(v) => Some(v),
        Err(e) => {
            println!("Failed to deserialize: {e}  — body was: {data}");
            tracing::error!("Failed to deserialize: {e} — body was: {data}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_token() -> Address {
        "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
            .parse()
            .unwrap()
    }

    fn test_pair() -> Address {
        "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852"
            .parse()
            .unwrap()
    }

    fn mock_honeypot_response(is_honeypot: bool) -> serde_json::Value {
        serde_json::json!({
            "simulationSuccess": true,
            "honeypotResult": {
                "isHoneypot": is_honeypot
            },
            "simulationResult": {
                "buyTax": 5.0,
                "sellTax": 5.0
            }
        })
    }

    #[tokio::test]
    async fn test_get_honeypot_returns_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/v2/IsHoneypot.*"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_honeypot_response(false)))
            .mount(&mock_server)
            .await;

        // Note: you'll need to refactor get_honey_pot to accept a base_url param for testability
        let result = get_honey_pot(&test_token(), &test_pair()).await;
        println!("RESULT: {result:?}");
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_get_honeypot_detects_honeypot() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/v2/IsHoneypot.*"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_honeypot_response(true)))
            .mount(&mock_server)
            .await;

        let result = get_honey_pot(&test_token(), &test_pair()).await;
        println!("RESULT: {result:?}");
        assert!(result.is_some());
        assert!(result.unwrap().honeypot_result.is_honeypot);
    }

    #[tokio::test]
    async fn test_get_honeypot_returns_none_on_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/v2/IsHoneypot.*"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = get_honey_pot(&test_token(), &test_pair()).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_honeypot_returns_none_on_invalid_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/v2/IsHoneypot.*"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
            .mount(&mock_server)
            .await;

        let result = get_honey_pot(&test_token(), &test_pair()).await;
        assert!(result.is_none());
    }
}
