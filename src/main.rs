use std::sync::Arc;

use crate::{config::Config, etherscan::client::EtherscanClient, token::market::get_eth_price};
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
    let wss_provider = provider::connect_wss(config.rpc_url_wss.clone()).await;

    tracing::info!("Listening for new blocks");
    let mut stream = wss_provider
        .subscribe_blocks()
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Failed to subscribe to blocks: {e}");
            std::process::exit(1);
        })
        .into_stream();
    
    let provider = Arc::new(wss_provider);
    let etherscan_client = Arc::new(EtherscanClient::new(config.etherscan_api_key.clone()));
    let bot = config.get_bot();

    // Add logic to process new blocks and scan for token transfers
    while let Some(block) = stream.next().await {
        let block_number = block.number;
        tracing::info!("Block Number: {block_number}");

        let provider = Arc::clone(&provider);
        let etherscan_client = Arc::clone(&etherscan_client);
        let bot = bot.clone();

        tokio::spawn(async move {
            let eth_price = get_eth_price(&provider).await;
            scanner::block::analyze_block(&provider, block_number, eth_price, &bot, &etherscan_client).await;
        });
    }
}
