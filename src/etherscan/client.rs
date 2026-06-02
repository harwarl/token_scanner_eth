use reqwest::Client;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct EtherscanClient {
    pub client: Client,
    pub api_key: String,
    pub base_url: String,
}

impl EtherscanClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            base_url: "https://api.etherscan.io/api".to_string(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, params: &[(&str, &str)]) -> Option<T> {
        let mut query = params.to_vec();
        query.push(("apiKey", &self.api_key.as_str()));

        let url = reqwest::Url::parse_with_params(&self.base_url, query.iter()).unwrap();

        match self.client.get(url).send().await {
            Ok(r) => r.json::<T>().await.ok(),
            Err(e) => {
                tracing::error!("Etherscan request failed: {e}");
                None
            }
        }
    }
}
