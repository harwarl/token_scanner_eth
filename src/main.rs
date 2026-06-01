use alloy::providers::Provider;

use crate::config::Config;

pub mod config;
pub mod provider;
pub mod types;

#[tokio::main]
async fn main() {
    // Load the env values
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();
    let config = Config::from_env().unwrap_or_else(|e| {
        tracing::error!("Configuration Error {e}");
        std::process::exit(1);
    });

    tracing::info!("Starting Token Scanner for ETH");
    let wss_provider = provider::connect_wss(&config.rpc_url_wss).await;
 
    tracing::info!("Listening for new blocks");
    let mut stream = wss_provider.subscribe_blocks().await.unwrap_or_else(|e| {
        tracing::error!("Failed to subscribe to blocks: {e}");
        std::process::exit(1);
    });


    // TODO: Add logic to process new blocks and scan for token transfers
}
