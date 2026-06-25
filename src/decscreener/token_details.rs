use alloy::primitives::Address;

use crate::types::DexscreenerVolume;

pub async fn get_dexscreener_data(
    pair_address: &Address,
    chain_id: u64,
) -> Option<DexscreenerVolume> {
    let chain_name = if chain_id == 1 {
        "ethereum"
    } else if chain_id == 8453 {
        "base"
    } else {
        return None;
    };

    let url = format!(
        "https://api.dexscreener.com/latest/dex/pairs/{}/{}",
        chain_name, pair_address
    );

    let res: serde_json::Value = reqwest::get(&url).await.ok()?.json().await.ok()?;
    
    let pair = &res["pair"];
    let socials = &pair["info"]["socials"];
    let websites = &pair["info"]["websites"];

    let website = websites[0]["url"].as_str().map(|s| s.to_string());

    let x = socials
        .as_array()
        .and_then(|arr| arr.iter().find(|s| s["type"] == "twitter"))
        .and_then(|s| s["url"].as_str())
        .map(|s| s.to_string());

    let telegram = socials
        .as_array()
        .and_then(|arr| arr.iter().find(|s| s["type"] == "telegram"))
        .and_then(|s| s["url"].as_str())
        .map(|s| s.to_string());

    Some(DexscreenerVolume {
        price_usd: pair["priceUsd"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0),
        market_cap: pair["marketCap"].as_f64().unwrap_or(0.0),
        liquidity_usd: pair["liquidity"]["usd"].as_f64().unwrap_or(0.0),
        vol_5m: pair["volume"]["m5"].as_f64().unwrap_or(0.0),
        vol_1h: pair["volume"]["h1"].as_f64().unwrap_or(0.0),
        vol_6h: pair["volume"]["h6"].as_f64().unwrap_or(0.0),
        vol_24h: pair["volume"]["h24"].as_f64().unwrap_or(0.0),
        buys_5m: pair["txns"]["m5"]["buys"].as_u64().unwrap_or(0) as u32,
        sells_5m: pair["txns"]["m5"]["sells"].as_u64().unwrap_or(0) as u32,
        buys_1h: pair["txns"]["h1"]["buys"].as_u64().unwrap_or(0) as u32,
        sells_1h: pair["txns"]["h1"]["sells"].as_u64().unwrap_or(0) as u32,
        buys_6h: pair["txns"]["h6"]["buys"].as_u64().unwrap_or(0) as u32,
        sells_6h: pair["txns"]["h6"]["sells"].as_u64().unwrap_or(0) as u32,
        buys_24h: pair["txns"]["h24"]["buys"].as_u64().unwrap_or(0) as u32,
        sells_24h: pair["txns"]["h24"]["sells"].as_u64().unwrap_or(0) as u32,
        price_change_5m: pair["priceChange"]["m5"].as_f64().unwrap_or(0.0),
        price_change_1h: pair["priceChange"]["h1"].as_f64().unwrap_or(0.0),
        price_change_6h: pair["priceChange"]["h6"].as_f64().unwrap_or(0.0),
        price_change_24h: pair["priceChange"]["h24"].as_f64().unwrap_or(0.0),
        holder_count: pair["info"]["holders"].as_u64().unwrap_or(0),
        website,
        x,
        telegram,
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use alloy::primitives::Address;
//     use std::str::FromStr;

//     // Real WETH/USDC pair on Ethereum
//     const WETH_USDC_PAIR: &str = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc";

//     #[tokio::test]
//     async fn test_get_dexscreener_data_real_pair() {
//         let address = Address::from_str(WETH_USDC_PAIR).unwrap();
//         let chain_id = 1u64;
//         let result = get_dexscreener_data(&address, chain_id).await;

//         assert!(result.is_some(), "Should return data for a known pair");
//         let data = result.unwrap();

//         // Volumes should be positive for an active pair
//         assert!(data.vol_24h > 0.0, "24h volume should be > 0");
//         assert!(data.vol_1h >= 0.0);
//         assert!(data.vol_6h >= 0.0);
//         assert!(data.vol_5m >= 0.0);

//         println!(
//             "Vol => 5m: ${:.2} | 1h: ${:.2} | 6h: ${:.2} | 24h: ${:.2}",
//             data.vol_5m, data.vol_1h, data.vol_6h, data.vol_24h
//         );
//     }

//     #[tokio::test]
//     async fn test_get_dexscreener_data_invalid_pair() {
//         // Random/dead address should return None or zeroed data
//         let address = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();
//         let chain_id = 1u64;
//         let result = get_dexscreener_data(&address, chain_id).await;

//         // Either None or all zeros — both are acceptable
//         if let Some(data) = result {
//             assert_eq!(data.vol_24h, 0.0);
//         }
//     }
// }
