use std::str::FromStr;

use alloy::{
    network::TransactionResponse,
    primitives::{Address, TxHash, U256},
    providers::Provider,
};

use crate::{
    types::TokenMetaInfo,
    utils::{
        constant::{DEAD1, DEAD2, WETH},
        contracts::IERC20,
    },
};

pub async fn get_token_info<P: Provider>(
    provider: &P,
    token0: Address,
    token1: Address,
) -> TokenMetaInfo {
    let token = if token0 == WETH { token1 } else { token0 };

    let contract = IERC20::new(token, provider);

    // Token Name
    let token_name = contract
        .name()
        .call()
        .await
        .unwrap_or_else(|_| "Unknown".to_string());

    let symbol = contract
        .symbol()
        .call()
        .await
        .unwrap_or_else(|_| "Ukn".to_string());

    // Total Supply
    let total_supply = contract
        .totalSupply()
        .call()
        .await
        .map(|r| r)
        .unwrap_or(U256::ZERO);

    // Decimals
    let decimals = contract.decimals().call().await.map(|r| r).unwrap_or(18u8);

    // format units
    let divisor = U256::from(10u64).pow(U256::from(decimals));
    let total_supply_formatted = total_supply.to::<u128>() as f64 / divisor.to::<u128>() as f64;

    // IsRenounced
    let owner = match contract.owner().call().await {
        Ok(r) => r,
        Err(_) => match contract.getOwner().call().await {
            Ok(r) => r,
            Err(_) => Address::ZERO,
        },
    };

    let renounced = owner == DEAD1 || owner == DEAD2;

    TokenMetaInfo {
        name: token_name,
        address: token,
        symbol,
        total_supply,
        total_supply_formatted,
        decimals,
        owner,
        renounced,
    }
}

pub async fn get_deployer<P: Provider>(provider: P, creation_tx_hash: &str) -> Address {
    let hash = match TxHash::from_str(creation_tx_hash) {
        Ok(h) => h,
        Err(_) => return Address::ZERO,
    };

    match provider.get_transaction_by_hash(hash).await {
        Ok(Some(txn)) => txn.from(),
        Ok(None) => Address::ZERO,
        Err(_) => Address::ZERO,
    }
}
