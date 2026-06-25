use std::time::UNIX_EPOCH;

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

    // Liquidity Weth
    let liquidity_weth: Option<f64> = if pair["quoteToken"]["symbol"] == "WETH" {
        Some(pair["liquidity"]["quote"].as_f64().unwrap_or(0.0))
    } else if pair["baseToken"]["symbol"] == "WETH" {
        Some(pair["liquidity"]["base"].as_f64().unwrap_or(0.0))
    } else {
        None
    };

    // Token Pair Age
    let pair_created_at = pair["pairCreatedAt"].as_u64().unwrap_or(0);
    let age_hours = if pair_created_at > 0 {
        let now = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        (now - pair_created_at) / (1000 * 60 * 60)
    } else {
        0
    };

    Some(DexscreenerVolume {
        price_usd: pair["priceUsd"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0),
        market_cap: pair["marketCap"].as_f64().unwrap_or(0.0),
        liquidity_usd: pair["liquidity"]["usd"].as_f64().unwrap_or(0.0),
        liquidity_weth,
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
        age_hours,
        website,
        x,
        telegram,
    })
}
