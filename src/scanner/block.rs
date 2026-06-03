use std::collections::HashSet;

use alloy::{
    network::TransactionResponse, primitives::Address, providers::Provider,
    rpc::types::TransactionReceipt,
};
use teloxide::Bot;

use crate::{
    etherscan::client::EtherscanClient,
    scanner,
    utils::{constant::WETH, contracts::IUniswapV2Pair, helpers},
};

pub async fn analyze_block<P: Provider>(
    provider: P,
    block_number: u64,
    eth_price: f64,
    bot: &Bot,
    etherscan_client: &EtherscanClient,
) {
    tracing::info!("Analyzing block: {}", block_number);
    let mut checked_pairs: HashSet<Address> = HashSet::new();

    let block = provider
        .get_block_by_number(block_number.into())
        .full()
        .await
        .expect("Failed to get block")
        .expect("Block not found");

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

            // compair the log address with the computed pair address to filter out irrelevant logs
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
                &mut checked_pairs,
                block_number,
                &provider,
                eth_price,
                bot,
                etherscan_client,
            )
            .await;
        }
    }
}
