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
    scanner,
    utils::{
        constant::WETH,
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
            let pair_address: Address = log.address();
            let pair = IUniswapV2Pair::new(pair_address, &provider);

            let token0 = match pair.token0().call().await {
                Ok(token) => token,
                Err(_) => continue,
            };

            let token1 = match pair.token1().call().await {
                Ok(token) => token,
                Err(_) => continue,
            };

            // compare the log address with the computed pair address to filter out irrelevant logs
            let computed_pair_address = helpers::get_univ2_pair_address(&token0, &token1);

            if computed_pair_address != pair_address {
                continue;
            }

            if token0 != WETH && token1 != WETH {
                continue;
            }

            // Decode as a Swap Event from Uniswap V2 Pair
            scanner::swap::decode_swap(
                log,
                pair_address,
                Arc::clone(&checked_pairs),
                block_number,
                &provider,
                token0,
                token1,
                bot,
                etherscan_client,
            )
            .await;

            // Decode as a new pair
            scanner::pair::decode_pair(
                &provider,
                log,
                bot,
                etherscan_client,
            )
            .await;
        }
    }
}
