use alloy::providers::{Provider, ProviderBuilder, WsConnect};

/// Creates a WebSocket provider connection
///
/// # Arguments
/// * `rpc_url_wss` - A WebSocket URL e.g. `wss://mainnet.infura.io/ws/v3/<key>`
///
/// # Panics
/// Panics if the WebSocket connection cannot be established
pub async fn connect_wss(rpc_url_wss: String) -> impl Provider {
    let ws = WsConnect::new(rpc_url_wss);
    ProviderBuilder::new()
        .connect_ws(ws)
        .await
        .expect("Failed to connect to websocket")
}

/// Creates an HTTP provider connection
///
/// # Arguments
/// * `rpc_url` - An HTTP/HTTPS URL e.g. `https://mainnet.infura.io/v3/<key>`
///
/// # Panics
/// Panics if the HTTP connection cannot be established
pub async fn connect(rpc_url: String) -> impl Provider {
    ProviderBuilder::new()
        .connect(rpc_url.as_str())
        .await
        .expect("Failed to connect to provider")
}
