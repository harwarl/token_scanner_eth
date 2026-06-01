use std::env;
use ethers::core::k256::sha2::digest::crypto_common;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variables: {0}")]
    MissingVar(String),
    #[error("Invalid URL format for {field}: {message}")]
    InvalidUrl { field: String, message: String },
}

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub rpc_url_wss: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let rpc_url =
            env::var("RPC_URL").map_err(|_| ConfigError::MissingVar("RPC_URL".to_string()))?;

        let rpc_url_wss = env::var("RPC_URL_WSS")
            .map_err(|_| ConfigError::MissingVar("RPC_URL_WSS".to_string()))?;

        Self::new(rpc_url, rpc_url_wss)
    }

    pub fn new(rpc_url: String, rpc_url_wss: String) -> Result<Self, ConfigError> {
        // Validate the Urls
        validate_url(&rpc_url, "RPC_URL")?;
        validate_wss_url(&rpc_url_wss, "RPC_URL_WSS")?;

        Ok(Self {
            rpc_url,
            rpc_url_wss,
        })
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
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_invalid_rpc_url_scheme() {
        let result = Config::new(
            "wss://mainnet.infura.io".to_string(),
            "wss://mainnet.infura.io/ws".to_string(),
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
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_wss_url() {
        let result = Config::new(
            "https://mainnet.infura.io".to_string(),
            "not_a_url".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_http_scheme_is_valid_for_rpc() {
        let result = Config::new(
            "http://localhost:8545".to_string(),
            "ws://localhost:8546".to_string(),
        );
        assert!(result.is_ok());
    }
}