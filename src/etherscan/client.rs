use reqwest::Client;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chain {
    Ethereum,
    Base,
    Arbitrum,
    Optimism,
}

impl Chain {
    pub fn chain_id(&self) -> u64 {
        match self {
            Chain::Ethereum => 1,
            Chain::Base => 8453,
            Chain::Arbitrum => 42161,
            Chain::Optimism => 10,
        }
    }

    pub fn from_chain_id(chain_id: u64) -> Option<Self> {
        match chain_id {
            1 => Some(Chain::Ethereum),
            8453 => Some(Chain::Base),
            42161 => Some(Chain::Arbitrum),
            10 => Some(Chain::Optimism),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EtherscanClient {
    pub client: Client,
    pub api_key: String,
    pub base_url: String,
    pub chain_id: u64,
}

impl EtherscanClient {
    pub fn new(api_key: String, chain_id: u64) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            base_url: match chain_id {
                8453 => "https://base.blockscout.com/api".to_string(),
                _ => "https://api.etherscan.io/v2/api".to_string(),
            },
            chain_id,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, params: &[(&str, &str)]) -> Option<T> {
        let mut query = params.to_vec();

        if self.chain_id == 1 {
            query.push(("apiKey", &self.api_key.as_str()));
        }

        let chain_id = format!("{}", self.chain_id);
        query.push(("chainid", &chain_id));

        let url = reqwest::Url::parse_with_params(&self.base_url, query.iter()).unwrap();
        match self.client.get(url).send().await {
            Ok(r) => {
                let text = r.text().await.ok()?;
                match serde_json::from_str::<T>(&text) {
                    Ok(parsed) => Some(parsed),
                    Err(e) => {
                        tracing::error!("Deserialization error: {e}");
                        println!("Deserialize error: {e}");
                        None
                    }
                }
            }
            Err(e) => {
                tracing::error!("Etherscan request failed: {e}");
                None
            }
        }
    }
}
