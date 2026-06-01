use alloy::{providers::Provider, rpc::types::BlockTransactionsKind};

pub async fn analyze_block<P: Provider>(provider: P, block_number: u64) {
    tracing::info!("Analyzing block: {}", block_number);
    let block = provider
        .get_block_by_number(block_number.into())
        .await
        .expect("Failed to get block")
        .expect("Block not found");

    let txns = block
        .transactions
        .as_transactions()
        .unwrap_or_default();

    for txn in txns {
        tracing::info!("Transaction: {:?}", txn.block_hash);
    }
}
