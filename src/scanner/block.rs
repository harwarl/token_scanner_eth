use std::{collections::HashSet, sync::Arc};

use tokio::sync::RwLock;

use alloy::{primitives::Address, providers::Provider};
use teloxide::Bot;

use crate::{
    etherscan::client::EtherscanClient,
    scanner::{self, bad_actor::BadActorDB, pipeline},
    utils::{constant::Contracts, contracts::IUniswapV2Pair, helpers::get_block_receipts},
};

pub async fn analyze_block<P>(
    provider: P,
    fallback: P,
    block_number: u64,
    checked_pairs: Arc<RwLock<HashSet<Address>>>,
    bot: &Bot,
    etherscan_client: &EtherscanClient,
    chain_id: u64,
    chain_contracts: Arc<Contracts>,
    bad_actors: Arc<RwLock<BadActorDB>>,
) where
    P: Provider,
{
    let block_log_receipts = get_block_receipts(&provider, &fallback, block_number)
        .await
        .expect("failed to get block receipts");

    for receipt in block_log_receipts {
        for log in receipt.inner.logs() {
            let log_address: Address = log.address();
            // Move to the next log if the pair has already been processed
            {
                let pairs = checked_pairs.read().await;
                if pairs.contains(&log_address) {
                    continue;
                }
            }

            // ========== Uniswap V4 ==========
            if log_address == chain_contracts.v4_pool_manager {
                if let Some(partial) =
                    scanner::pool::decode_pool(log, &provider, Arc::clone(&chain_contracts)).await
                {
                    pipeline::run_pipeline(
                        &provider,
                        log_address,
                        Arc::clone(&checked_pairs),
                        partial,
                        bot,
                        etherscan_client,
                        chain_id,
                        Arc::clone(&chain_contracts),
                        Arc::clone(&bad_actors),
                    )
                    .await;
                    continue;
                }
            }

            // Decode Pair Obsolete since the goal is to decode fresh Mints
            // if let Some(partial) = scanner::pair::decode_pair(&provider, log).await {
            //     pipeline::run_pipeline(
            //         &provider,
            //         log_address,
            //         Arc::clone(&checked_pairs),
            //         partial,
            //         bot,
            //         etherscan_client,
            //     )
            //     .await;
            //     continue;
            // };

            // For swap — need token0/token1 from contract
            let pair = IUniswapV2Pair::new(log_address, &provider);
            let token0 = match pair.token0().call().await {
                Ok(token) => token,
                Err(_) => continue,
            };
            let token1 = match pair.token1().call().await {
                Ok(token) => token,
                Err(_) => continue,
            };

            // Filter out non WETH pairs early
            if token0 != chain_contracts.weth && token1 != chain_contracts.weth {
                continue;
            }

            // // Validate it's actually a uniswap v2 pair
            // let computed_pair_address =
            //     helpers::get_univ2_pair_address(&token0, &token1, &chain_contracts.v2_factory);
            // if computed_pair_address != log_address {
            //     continue;
            // }

            // DECODE MINT
            if let Some(partial) =
                scanner::mint::decode_mint(&provider, log, token0, token1, chain_contracts.weth)
                    .await
            {
                pipeline::run_pipeline(
                    &provider,
                    log_address,
                    Arc::clone(&checked_pairs),
                    partial,
                    bot,
                    etherscan_client,
                    chain_id,
                    Arc::clone(&chain_contracts),
                    Arc::clone(&bad_actors),
                )
                .await;
                continue;
            }

            // DECODE SWAP
            if let Some(partial) = scanner::swap::decode_swap(
                log,
                log_address,
                block_number,
                &provider,
                token0,
                token1,
                chain_contracts.weth,
            )
            .await
            {
                pipeline::run_pipeline(
                    &provider,
                    log_address,
                    Arc::clone(&checked_pairs),
                    partial,
                    bot,
                    etherscan_client,
                    chain_id,
                    Arc::clone(&chain_contracts),
                    Arc::clone(&bad_actors),
                )
                .await;
                continue;
            }
            // Decode as a Swap Event from Uniswap V2 Pair
        }
    }
}
