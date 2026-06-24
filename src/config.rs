use std::env;
use teloxide::Bot;
use thiserror::Error;
use url::Url;

use crate::utils::constant::{BASE_FREE_RPCS, ETH_FREE_RPCS};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variables: {0}")]
    MissingVar(String),
    #[error("Invalid URL format for {field}: {message}")]
    InvalidUrl { field: String, message: String },
    #[error("Invalid Chain Id: {0}")]
    InvalidChainId(String),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub rpc_url_wss: String,
    pub bot: Bot,
    pub etherscan_api_key: String,
    pub chat_id: i64,
    pub chain_id: u64,
    pub free_rpcs: &'static [&'static str],
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let eth_rpc_url = env::var("ETH_RPC_URL")
            .map_err(|_| ConfigError::MissingVar("ETH_RPC_URL".to_string()))?;

        let eth_rpc_url_wss = env::var("ETH_RPC_URL_WSS")
            .map_err(|_| ConfigError::MissingVar("ETH_RPC_URL_WSS".to_string()))?;

        let base_rpc_url = env::var("BASE_RPC_URL")
            .map_err(|_| ConfigError::MissingVar("BASE_RPC_URL".to_string()))?;

        let base_rpc_url_wss = env::var("BASE_RPC_URL_WSS")
            .map_err(|_| ConfigError::MissingVar("BASE_RPC_URL_WSS".to_string()))?;

        let bot_token = env::var("TELOXIDE_TOKEN")
            .map_err(|_| ConfigError::MissingVar("TELOXIDE_TOKEN".to_string()))?;

        let etherscan_api_key = env::var("ETHERSCAN_API_KEY")
            .map_err(|_| ConfigError::MissingVar("ETHERSCAN_API_KEY".to_string()))?;

        let chat_id =
            env::var("CHAT_ID").map_err(|_| ConfigError::MissingVar("CHAT_ID".to_string()))?;

        let chain_id =
            env::var("CHAIN_ID").map_err(|_| ConfigError::MissingVar("CHAIN_ID".to_string()))?;

        Self::new(
            eth_rpc_url,
            eth_rpc_url_wss,
            base_rpc_url,
            base_rpc_url_wss,
            bot_token,
            etherscan_api_key,
            chat_id,
            chain_id,
        )
    }

    pub fn new(
        eth_rpc_url: String,
        eth_rpc_url_wss: String,
        base_rpc_url: String,
        base_rpc_url_wss: String,
        bot_token: String,
        etherscan_api_key: String,
        chat_id: String,
        chain_id: String,
    ) -> Result<Self, ConfigError> {
        // Parse the chain_id
        let chain_id: u64 = chain_id
            .parse()
            .map_err(|_| ConfigError::InvalidChainId("failed to parse chainId".to_string()))?;

        let (rpc_url, rpc_url_wss, free_rpcs) = match chain_id {
            1 => (eth_rpc_url, eth_rpc_url_wss, ETH_FREE_RPCS),
            8453 => (base_rpc_url, base_rpc_url_wss, BASE_FREE_RPCS),
            _ => {
                return Err(ConfigError::InvalidChainId(format!(
                    "Invalid Chain Id: {}",
                    chain_id
                )));
            }
        };

        // Validate the Urls
        validate_url(&rpc_url, "RPC_URL")?;
        validate_wss_url(&rpc_url_wss, "RPC_URL_WSS")?;
        let parse_chat_id = chat_id.parse::<i64>().expect("Not a valid number");

        // Create Bot Instance
        let bot = Bot::new(bot_token);

        Ok(Self {
            rpc_url,
            rpc_url_wss,
            bot,
            etherscan_api_key,
            chat_id: parse_chat_id * -1,
            chain_id,
            free_rpcs,
        })
    }

    pub fn get_bot(self: &Self) -> &Bot {
        &self.bot
    }
}

fn validate_url(url: &str, field: &str) -> Result<(), ConfigError> {
    let parsed = Url::parse(url).map_err(|e| ConfigError::InvalidUrl {
        field: field.to_string(),
        message: e.to_string(),
    })?;

    match parsed.scheme() {
        "http" | "https" => Ok(()),
        scheme => Err(ConfigError::InvalidUrl {
            field: field.to_string(),
            message: format!("expected http/https, got '{scheme}'"),
        }),
    }
}

fn validate_wss_url(url: &str, field: &str) -> Result<(), ConfigError> {
    let parsed = Url::parse(url).map_err(|e| ConfigError::InvalidUrl {
        field: field.to_string(),
        message: e.to_string(),
    })?;

    match parsed.scheme() {
        "ws" | "wss" => Ok(()),
        scheme => Err(ConfigError::InvalidUrl {
            field: field.to_string(),
            message: format!("expected ws/wss, got '{scheme}'"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let config = Config::new(
            "https://mainnet.infura.io/v3/key".to_string(),
            "wss://mainnet.infura.io/ws/v3/key".to_string(),
            "https://mainnet.infura.io/v3/key".to_string(),
            "wss://mainnet.infura.io/ws/v3/key".to_string(),
            "some_token".to_string(),
            "some_ether_scan_key".to_string(),
            "-2339089083209803".to_string(),
            "1".to_string(),
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_invalid_rpc_url_scheme() {
        let result = Config::new(
            "wss://mainnet.infura.io".to_string(),
            "wss://mainnet.infura.io/ws".to_string(),
            "wss://mainnet.infura.io".to_string(),
            "wss://mainnet.infura.io/ws".to_string(),
            "some_token".to_string(),
            "some_ether_scan_key".to_string(),
            "-2339089083209803".to_string(),
            "1".to_string(),
        );
        match result {
            Err(ConfigError::InvalidUrl { field, .. }) => assert_eq!(field, "RPC_URL"),
            _ => panic!("Expected InvalidUrl error"),
        }
    }

    #[test]
    fn test_invalid_wss_url_scheme() {
        let result = Config::new(
            "https://mainnet.infura.io".to_string(),
            "https://mainnet.infura.io".to_string(),
            "wss://mainnet.infura.io".to_string(),
            "wss://mainnet.infura.io/ws".to_string(),
            "some_token".to_string(),
            "some_ether_scan_key".to_string(),
            "-2339089083209803".to_string(),
            "1".to_string(),
        );
        match result {
            Err(ConfigError::InvalidUrl { field, .. }) => assert_eq!(field, "RPC_URL_WSS"),
            _ => panic!("Expected InvalidUrl error"),
        }
    }

    #[test]
    fn test_malformed_rpc_url() {
        let result = Config::new(
            "not_a_url".to_string(),
            "wss://mainnet.infura.io/ws".to_string(),
            "wss://mainnet.infura.io".to_string(),
            "wss://mainnet.infura.io/ws".to_string(),
            "some_token".to_string(),
            "some_ether_scan_key".to_string(),
            "-2339089083209803".to_string(),
            "1".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_wss_url() {
        let result = Config::new(
            "https://mainnet.infura.io".to_string(),
            "not_a_url".to_string(),
            "wss://mainnet.infura.io".to_string(),
            "wss://mainnet.infura.io/ws".to_string(),
            "some_token".to_string(),
            "some_ether_scan_key".to_string(),
            "-2339089083209803".to_string(),
            "1".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_http_scheme_is_valid_for_rpc() {
        let result = Config::new(
            "http://localhost:8545".to_string(),
            "ws://localhost:8546".to_string(),
            "wss://mainnet.infura.io".to_string(),
            "wss://mainnet.infura.io/ws".to_string(),
            "some_token".to_string(),
            "some_ether_scan_key".to_string(),
            "2339089083209803".to_string(),
            "1".to_string(),
        );

        assert!(result.is_ok());
        assert!(result.unwrap().chat_id < 0);
    }
}
