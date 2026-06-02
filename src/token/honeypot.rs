use alloy::primitives::Address;
use reqwest::Client;

use crate::types::HoneyPotResponse;

pub async fn get_honey_pot(token: &Address, pair_address: &Address) -> Option<HoneyPotResponse> {
    let client = Client::new();

    let honeypot_url = format!(
        "https://api.honeypot.is/v2/IsHoneypot?address=${token}&pair=${pair_address}&chainID=1"
    );
    let res = match client.get(&honeypot_url).send().await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to call honeypot API: {e}");
            return None;
        }
    };

    match res.json::<HoneyPotResponse>().await {
        Ok(v) => Some(v),
        Err(e) => {
            tracing::error!("Failed to deserialize honeypot response: {e}");
            None
        }
    }
}
