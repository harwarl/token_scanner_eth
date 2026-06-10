use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use crate::{
    config::Config, etherscan::client::EtherscanClient, lib::server_balancer::LoadBalancer,
};
use alloy::{primitives::Address, providers::Provider};
use axum::{Router, routing::get};
use futures_util::StreamExt;

pub mod config;
pub mod decscreener;
pub mod etherscan;
pub mod lib;
pub mod lplock;
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
        vec!["https://eth.blockrazor.xyz", "https://ethereum-rpc.publicnode.com", "https://ethereum.public.blockpi.network/v1/rpc/public"],
    ));
    let etherscan_client = Arc::new(EtherscanClient::new(config.etherscan_api_key.clone()));
    let bot = config.get_bot();

    // Add logic to process new blocks and scan for token transfers
    while let Some(block) = stream.next().await {
        let block_number = block.number;

        if block_number % 100 == 0 {
            checked_pairs.write().unwrap().clear();
        }

        // let provider = Arc::clone(&provider);
        let etherscan_client = Arc::clone(&etherscan_client);
        let bot = bot.clone();
        let checked_pairs = Arc::clone(&checked_pairs);
        let provider_balancer = Arc::clone(&provider_balancer);
        let server = provider_balancer.get_next_server().await.unwrap();
        let url = server.url.clone();
        let provider = provider::connect(url).await;

        tokio::spawn(async move {
            scanner::block::analyze_block(
                &provider,
                block_number,
                checked_pairs,
                &bot,
                &etherscan_client,
            )
            .await;
        });
    }
}
