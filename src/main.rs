use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use crate::{
    config::Config,
    etherscan::client::EtherscanClient,
    library::server_balancer::{LoadBalancer, Server},
};
use alloy::{primitives::Address, providers::Provider};
use axum::{Router, routing::get};
use futures_util::StreamExt;

pub mod config;
pub mod decscreener;
pub mod etherscan;
pub mod library;
pub mod lplock;
pub mod provider;
pub mod scanner;
pub mod telegram;
pub mod token;
pub mod types;
pub mod utils;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() {
    // Load the env values
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let config = Config::from_env().unwrap_or_else(|e| {
        tracing::error!("Configuration Error {e}");
        std::process::exit(1);
    });

    // Create a minimalistic Server for health check
    let app = Router::new().route("/", get(|| async { "ok" }));
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    tracing::info!("Health server listening on port {port}");

    tokio::spawn(async {
        axum::serve(listener, app).await.unwrap();
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

    let checked_pairs = Arc::new(RwLock::new(HashSet::<Address>::new()));
    // let provider = Arc::new(wss_provider);
    let provider_balancer = Arc::new(LoadBalancer::new(
        1,
        vec![
            "https://eth.blockrazor.xyz",
            "https://ethereum-rpc.publicnode.com",
            "https://ethereum.public.blockpi.network/v1/rpc/public",
            "https://0xrpc.io/eth",
            "https://ethereum-json-rpc.stakely.io",
            "https://rpc.fullsend.to",
            "https://api.zan.top/eth-mainnet",
            "https://eth.llamarpc.com",
            "https://rpc.payload.de",
            "https://endpoints.omniatech.io/v1/eth/mainnet/public",
            "https://rpc.public.curie.radiumblock.co/ws/ethereum",
            "https://rpc.polysplit.cloud/v1/chain/1",
            "https://eth.merkle.io",
        ],
    ));
    let fallback_provider = Arc::new(provider::connect(config.rpc_url.clone()).await);
    let etherscan_client = Arc::new(EtherscanClient::new(config.etherscan_api_key.clone()));
    let bot = config.get_bot();

    // Add logic to process new blocks and scan for token transfers
    while let Some(block) = stream.next().await {
        let block_number = block.number;

        if block_number % 1000 == 0 {
            checked_pairs.write().unwrap().clear();
        }

        let etherscan_client = Arc::clone(&etherscan_client);
        let checked_pairs = Arc::clone(&checked_pairs);
        let provider_balancer = Arc::clone(&provider_balancer);
        let fallback = Arc::clone(&fallback_provider);
        let bot = bot.clone();

        let server = provider_balancer
            .get_next_server()
            .await
            .unwrap_or_else(|| Server::fallback(config.rpc_url.as_str(), 1));

        let url = server.url.clone();
        let provider = provider::connect(url).await;

        tokio::spawn(async move {
            scanner::block::analyze_block(
                &provider,
                &fallback,
                block_number,
                checked_pairs,
                &bot,
                &etherscan_client,
            )
            .await;
        });
    }
}
