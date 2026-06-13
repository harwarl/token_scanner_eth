use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use alloy::{
    network::TransactionResponse, primitives::Address, providers::Provider,
    rpc::types::TransactionReceipt,
};
use teloxide::Bot;

use crate::{
    etherscan::client::EtherscanClient,
    scanner::{self, pipeline},
    utils::{
        constant::{UNISWAP_V4_POOL_MANAGER, WETH},
        contracts::IUniswapV2Pair,
        helpers::{self, get_block},
    },
};

pub async fn analyze_block<P>(
    provider: P,
    fallback: P,
    block_number: u64,
    checked_pairs: Arc<RwLock<HashSet<Address>>>,
    bot: &Bot,
    etherscan_client: &EtherscanClient,
) where
    P: Provider,
{
    let block = get_block(&provider, &fallback, block_number)
        .await
        .expect("Failed to get block");

    let txns = block.transactions.as_transactions().unwrap_or_default();

    for txn in txns {
        // Get the transaction receipt to analyze logs and events
        let txn_receipts: TransactionReceipt =
            match provider.get_transaction_receipt(txn.tx_hash()).await {
                Ok(Some(receipt)) => receipt,
                Ok(None) => continue,
                Err(_) => continue,
            };

        if !txn_receipts.status() {
            continue;
        }

        for log in txn_receipts.logs() {
            let log_address: Address = log.address();

            // Move to the next log if the pair has already been processed
            {
                let pairs = checked_pairs.read().unwrap();
                if pairs.contains(&log_address) {
                    continue;
                }
            }

            // ========== Uniswap V4 ==========
            if log_address == UNISWAP_V4_POOL_MANAGER {
                if let Some(partial) = scanner::pool::decode_pool(log, &provider).await {
                    pipeline::run_pipeline(
                        &provider,
                        log_address,
                        Arc::clone(&checked_pairs),
                        partial,
                        bot,
                        etherscan_client,
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
            if token0 != WETH && token1 != WETH {
                continue;
            }

            // Validate it's actually a uniswap v2 pair
            let computed_pair_address = helpers::get_univ2_pair_address(&token0, &token1);
            if computed_pair_address != log_address {
                continue;
            }

            // DECODE MINT
            if let Some(partial) = scanner::mint::decode_mint(&provider, log, token0, token1).await
            {
                pipeline::run_pipeline(
                    &provider,
                    log_address,
                    Arc::clone(&checked_pairs),
                    partial,
                    bot,
                    etherscan_client,
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
                )
                .await;
                continue;
            }
            // Decode as a Swap Event from Uniswap V2 Pair
        }
    }
}
