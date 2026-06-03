use std::collections::HashSet;

use crate::{
    etherscan::client::EtherscanClient,
    lp::lp_lock::is_lp_locked,
    telegram,
    token::{
        honeypot::get_honey_pot,
        info::{get_deployer, get_token_info},
        liquidity::get_liquidity,
        market::get_market_cap,
    },
    types::TokenInfo,
    utils::{constant::WETH, contracts::IUniswapV2Pair},
};
use alloy::{
    primitives::{Address, U256, b256},
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
};
use teloxide::Bot;

pub async fn decode_swap<P: Provider>(
    log: &Log,
    pair_address: Address,
    checked_pairs: &mut HashSet<Address>,
    block_number: u64,
    provider: &P,
    eth_price: f64,
    bot: &Bot,
    etherscan_client: &EtherscanClient,
) {
    // Check if the hashed set contains the pair address, if it does, skip processing this log to avoid duplicate processing of the same pair in the same block
    if checked_pairs.contains(&pair_address) {
        return;
    };

    if let Ok(_swap_event) = IUniswapV2Pair::Swap::decode_log(log.inner.as_ref()) {
        checked_pairs.insert(pair_address);

        let swap_topic =
            b256!("0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822");

        let pair = IUniswapV2Pair::new(pair_address, &provider);
        let token0 = match pair.token0().call().await {
            Ok(t) => t,
            Err(_) => {
                return;
            }
        };

        let token1 = match pair.token1().call().await {
            Ok(t) => t,
            Err(_) => {
                return;
            }
        };

        let filter = Filter::new()
            .address(pair_address)
            .event_signature(swap_topic)
            .from_block(block_number - 2)
            .to_block(block_number);

        let logs = match provider.get_logs(&filter).await {
            Ok(logs) => logs,
            Err(_) => {
                return;
            }
        };

        let mut buy_counts = 0u32;
        let mut total_swaps = 0u32;
        let mut unique_buyers: HashSet<Address> = HashSet::new();
        let mut volume_usd = 0.0f64;

        for past_log in logs {
            if let Ok(past_swap_event) = IUniswapV2Pair::Swap::decode_log(past_log.inner.as_ref()) {
                total_swaps += 1;

                // Track Unique buyers via sender field
                unique_buyers.insert(past_swap_event.sender);

                // Sum WETH volume
                let weth_in = if token0 == WETH {
                    past_swap_event.amount0In.to::<u128>() as f64 / 1e18
                } else {
                    past_swap_event.amount1In.to::<u128>() as f64 / 1e18
                };
                volume_usd += weth_in * eth_price;

                let swap_direction = if past_swap_event.amount1Out == U256::ZERO {
                    if token0 == WETH { 1 } else { 0 }
                } else {
                    if token0 == WETH { 0 } else { 1 }
                };

                if swap_direction == 0 {
                    buy_counts += 1;
                }
            }
        }

        tracing::info!("Buy Counts: {buy_counts}");
        if buy_counts < 20 {
            return;
        }

        // Buy Pressure Ratio
        let buy_ratio = if total_swaps > 0 {
            buy_counts as f64 / total_swaps as f64
        } else {
            0.0
        };

        // Get Token Info
        let token_meta = get_token_info(provider, token0, token1).await;
        let liquidity = get_liquidity(provider, &pair_address, token0).await;
        let market_cap = get_market_cap(
            provider,
            token0,
            token_meta.total_supply,
            token_meta.decimals,
            pair_address,
        )
        .await;
        let honeypoy_res = get_honey_pot(&token_meta.address, &pair_address)
            .await
            .unwrap();
        let deployer = get_deployer(provider, &honeypoy_res.pair.creation_tx_hash).await;
        let is_lp_locked = is_lp_locked(&pair_address, provider).await;

        // Etherscan calls
        let contract_info = etherscan_client
            .get_contract_info(&token_meta.address)
            .await;
        let wallet_info = etherscan_client.get_wallet_info(&deployer).await;
        let bad_reputation = etherscan_client.check_deployer_reputation(&deployer).await;

        let liquidity_usd = liquidity * eth_price;
        let marketcap_usd = market_cap * eth_price;

        // More Filters
        if unique_buyers.len() < 2 {
            return;
        } // at least 2 different wallets
        if volume_usd < 500.0 {
            return;
        } // at least $500 volume
        if liquidity_usd < 5000.0 {
            return;
        } // at least $5k liquidity

        let token_info = TokenInfo {
            name: token_meta.name,
            address: token_meta.address,
            total_supply: token_meta.total_supply_formatted,
            verified: contract_info.verified,
            contract_name: contract_info.contract_name,
            lp_locked: is_lp_locked,
            renounced: token_meta.renounced,
            buy_tax: honeypoy_res.simulation_result.buy_tax,
            sell_tax: honeypoy_res.simulation_result.sell_tax,
            market_cap_usd: marketcap_usd,
            honeypot: honeypoy_res.honeypot_result.is_honeypot,
            deployer: deployer,
            liquidity_usd: liquidity * eth_price,
            buy_ratio,
            deployer_age_days: wallet_info.age_days,
            is_fresh_wallet: wallet_info.is_fresh_wallet,
            bad_reputation,
            buy_count: buy_counts,
            total_swaps,
            mcap_to_liq_ratio: market_cap / liquidity,
            unique_buyers_count: unique_buyers.len() as u32,
            volume_usd: volume_usd,
        };

        // Send the Message to TELEGRAM
        telegram::bot::send_tg_message(bot, token_info).await;
    }
}
