use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use alloy::{primitives::Address, providers::Provider};
use teloxide::Bot;

use crate::{
    decscreener::token_details::get_dexscreener_data,
    etherscan::client::EtherscanClient,
    lplock::lp_lock::is_lp_locked,
    telegram,
    token::{honeypot::get_honey_pot, info::get_deployer},
    types::{PartialTokenInfo, TokenInfo},
    utils::helpers::calculate_volatility,
};

pub async fn run_pipeline<P>(
    provider: &P,
    pair_address: Address,
    checked_pairs: Arc<RwLock<HashSet<Address>>>,
    token_info: PartialTokenInfo,
    bot: &Bot,
    etherscan_client: &EtherscanClient,
) where
    P: Provider,
{
    {
        let mut pairs = checked_pairs.write().unwrap();
        if pairs.contains(&pair_address) {
            return;
        }
        pairs.insert(pair_address);
    }

    // Get Token Info
    let dex_data = get_dexscreener_data(&token_info.pair_address).await;
    let dex_data = match dex_data {
        Some(data) => data,
        None => return,
    };
    let honeypoy_res = get_honey_pot(&token_info.token_address, &token_info.pair_address)
        .await
        .unwrap();
    let deployer = get_deployer(provider, &honeypoy_res.pair.creation_tx_hash).await;
    let is_lp_locked = is_lp_locked(&token_info.pair_address, &provider).await;

    let liquidity_usd = dex_data.liquidity_usd;
    let marketcap_usd = dex_data.market_cap;
    // Filter Based on MC
    if marketcap_usd < 10_000f64 || marketcap_usd > 1_000_000f64 {
        return;
    }

    // Etherscan calls
    let contract_info = etherscan_client
        .get_contract_info(&token_info.token_address)
        .await;
    let wallet_info = etherscan_client.get_wallet_info(&deployer).await;
    let bad_reputation = etherscan_client.check_deployer_reputation(&deployer).await;
    // let holder_count = etherscan_client.get_holder_count(&token_info.address).await;

    // Get Prices

    let mcap_to_liq_ratio = if liquidity_usd > 0.0 {
        marketcap_usd / liquidity_usd
    } else {
        0.0
    };

    // Volatility
    let volatility = calculate_volatility(dex_data.vol_24h, dex_data.liquidity_usd);

    // // More Filters
    // if unique_buyers.len() < 2 {
    //     return;
    // } // at least 2 different wallets
    // if volume_usd < 500.0 {
    //     return;
    // } // at least $500 volume
    // if liquidity_usd < 5000.0 {
    //     return;
    // } // at least $5k liquidity

    // Less than 10k tokens to be abandoned

    // Socials
    let website_link = dex_data
        .website
        .as_ref()
        .map(|u| format!("<a href=\"{}\">🌐 Website</a>", u))
        .unwrap_or_default();

    let x_link = dex_data
        .x
        .as_ref()
        .map(|u| format!("<a href=\"{}\">𝕏 Twitter</a>", u))
        .unwrap_or_default();

    let telegram_link = dex_data
        .telegram
        .as_ref()
        .map(|u| format!("<a href=\"{}\">✈️ Telegram</a>", u))
        .unwrap_or_default();

    let socials = [website_link, x_link, telegram_link]
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join(" • ");

    let token_info = TokenInfo {
        name: token_info.name,
        address: token_info.token_address,
        total_supply: token_info.total_supply,
        verified: contract_info.verified,
        contract_name: contract_info.contract_name,
        lp_locked: is_lp_locked,
        renounced: token_info.renounced,
        buy_tax: honeypoy_res.simulation_result.buy_tax,
        sell_tax: honeypoy_res.simulation_result.sell_tax,
        market_cap_usd: marketcap_usd,
        honeypot: honeypoy_res.honeypot_result.is_honeypot,
        deployer: deployer,
        liquidity_usd,
        deployer_age_days: wallet_info.age_days,
        is_fresh_wallet: wallet_info.is_fresh_wallet,
        bad_reputation,
        mcap_to_liq_ratio,
        holder_count: dex_data.holder_count,
        volume_5m: dex_data.vol_5m,
        volume_1h: dex_data.vol_1h,
        volume_6h: dex_data.vol_6h,
        volume_24h: dex_data.vol_24h,
        buys_5m: dex_data.buys_5m,
        sells_5m: dex_data.sells_5m,
        buys_1h: dex_data.buys_1h,
        sells_1h: dex_data.sells_1h,
        buys_6h: dex_data.buys_6h,
        sells_6h: dex_data.sells_6h,
        buys_24h: dex_data.buys_24h,
        sells_24h: dex_data.sells_24h,
        price_usd: dex_data.price_usd,
        price_change_5m: dex_data.price_change_5m,
        price_change_1h: dex_data.price_change_1h,
        volatility,
        socials,
    };

    // Send the Message to TELEGRAM
    telegram::bot::send_tg_message(bot, token_info).await;
}
