use std::collections::HashSet;

use crate::{
    token::info::get_token_info,
    types::PartialTokenInfo,
    utils::{constant::WETH, contracts::IUniswapV2Pair},
};
use alloy::{
    primitives::{Address, U256, b256},
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
};

pub async fn decode_swap<P: Provider>(
    log: &Log,
    pair_address: Address,
    block_number: u64,
    provider: &P,
    token0: Address,
    token1: Address,
) -> Option<PartialTokenInfo> {
    // Check if the hashed set contains the pair address, if it does, skip processing this log to avoid duplicate processing of the same pair in the same block

    if let Ok(_) = IUniswapV2Pair::Swap::decode_log(log.inner.as_ref()) {
        let swap_topic = b256!("d78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822");

        let filter = Filter::new()
            .address(pair_address)
            .event_signature(swap_topic)
            .from_block(block_number - 5)
            .to_block(block_number);

        let logs = match provider.get_logs(&filter).await {
            Ok(logs) => logs,
            Err(_) => {
                return None;
            }
        };

        let mut buy_counts = 0u32;
        // let mut total_swaps = 0u32;
        let mut unique_buyers: HashSet<Address> = HashSet::new();

        for past_log in logs {
            if let Ok(past_swap_event) = IUniswapV2Pair::Swap::decode_log(past_log.inner.as_ref()) {
                // total_swaps += 1;

                // Track Unique buyers via sender field
                unique_buyers.insert(past_swap_event.sender);

                // Sum WETH volume
                // let weth_in = if token0 == WETH {
                //     past_swap_event.amount0In.to::<u128>() as f64 / 1e18
                // } else {
                //     past_swap_event.amount1In.to::<u128>() as f64 / 1e18
                // };
                // volume_usd += weth_in * eth_price;

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

        println!("Buy count: {:?}", buy_counts);
        if buy_counts < 5 {
            return None;
        }

        let token = if token0 == WETH { token1 } else { token0 };
        let token_info = get_token_info(provider, token).await;

        return Some(PartialTokenInfo {
            token_address: token,
            pair_address,
            token0,
            token1,
            name: token_info.name,
            symbol: token_info.symbol,
            total_supply: token_info.total_supply_formatted,
            renounced: token_info.renounced,
            deployer: token_info.owner,
        });
    }
    None
}
