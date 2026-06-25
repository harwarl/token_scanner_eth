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
}

impl EtherscanClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            base_url: "https://api.etherscan.io/v2/api".to_string(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, chain: Chain, params: &[(&str, &str)]) -> Option<T> {
        let mut query = params.to_vec();
        let chain_id = chain.chain_id().to_string();
        
        query.push(("chainid", &chain_id));
        query.push(("apiKey", &self.api_key.as_str()));

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
