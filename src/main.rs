use crate::config::Config;

pub mod config;

#[tokio::main]
async fn main() {
    // Load the env values
    dotenv::dotenv().ok();
    let config = Config::from_env().unwrap_or_else(|e| {
        tracing::error!("Configuration Error {e}");
        std::process::exit(1);
    });

    tracing_subscriber::fmt().init();

    tracing::info!("Starting Token Scanner for ETH");

    tracing::info!("Listening for new blocks");
}
