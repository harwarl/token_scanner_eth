use crate::{config::Config, token::market::get_eth_price};
use alloy::providers::Provider;
use futures_util::StreamExt;

pub mod config;
pub mod etherscan;
pub mod lp;
pub mod provider;
pub mod scanner;
pub mod telegram;
pub mod token;
pub mod types;
pub mod utils;

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
    let mut stream = wss_provider
        .subscribe_blocks()
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Failed to subscribe to blocks: {e}");
            std::process::exit(1);
        })
        .into_stream();

    // TODO: Add logic to process new blocks and scan for token transfers
    while let Some(block) = stream.next().await {
        let block_number = block.number;

        // Get ETh Price
        let eth_price = get_eth_price(&wss_provider).await;
        tracing::info!("Current ETH Price: ${eth_price:.2}");

        // Analyze the block for token transfers and logs
        scanner::block::analyze_block(&wss_provider, block_number, eth_price).await;
    }
}
