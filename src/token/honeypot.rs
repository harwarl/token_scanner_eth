use alloy::primitives::Address;

use crate::types::HoneyPotResponse;

pub async fn get_honey_pot(
    token: &Address,
    pair_address: &Address,
    chain_id: u64,
) -> Option<HoneyPotResponse> {
    let honeypot_url = format!(
        "https://api.honeypot.is/v2/IsHoneypot?address={token}&pair={pair_address}&chainID={chain_id}"
    );

    let res = match reqwest::get(&honeypot_url).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to call honeypot API: {e}");
            return None;
        }
    };

    let data = match res.text().await {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("Failed to read response body: {e}");
            return None;
        }
    };

    match serde_json::from_str::<HoneyPotResponse>(&data) {
        Ok(v) => Some(v),
        Err(e) => {
            println!("Failed to deserialize: {e}  — body was: {data}");
            tracing::error!("Failed to deserialize: {e} — body was: {data}");
            None
        }
    }
}
