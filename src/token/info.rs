use std::str::FromStr;

use alloy::{
    network::TransactionResponse, primitives::{Address, TxHash, U256, Uint}, providers::Provider
};

use crate::utils::{
    constant::{DEAD1, DEAD2, WETH},
    contracts::IERC20,
};

pub async fn get_token_info<P: Provider>(
    provider: &P,
    token0: Address,
    token1: Address,
) -> (String, Uint<256, 4>, Uint<256, 4>, f64, u8, Address, bool) {
    let token = if token0 == WETH { token1 } else { token0 };

    // Token Name
    let token_name = IERC20::new(token, provider)
        .name()
        .call()
        .await
        .unwrap_or_else(|_| "Unknown".to_string());

    // Total Balance
    let contract_balance = IERC20::new(token, provider)
        .balanceOf(token)
        .call()
        .await
        .map(|r| r)
        .unwrap_or(U256::ZERO);

    // Total Supply
    let total_supply = IERC20::new(token, provider)
        .totalSupply()
        .call()
        .await
        .map(|r| r)
        .unwrap_or(U256::ZERO);

    // Decimals
    let decimals = IERC20::new(token, provider)
        .decimals()
        .call()
        .await
        .map(|r| r)
        .unwrap_or(18u8);

    // format units
    let divisor = U256::from(10u64).pow(U256::from(decimals));
    let total_supply_formatted = total_supply.to::<u128>() as f64 / divisor.to::<u128>() as f64;

    // IsRenounced
    let owner = match IERC20::new(token, provider).owner().call().await {
        Ok(r) => r,
        Err(_) => match IERC20::new(token, provider).getOwner().call().await {
            Ok(r) => r,
            Err(_) => Address::ZERO,
        },
    };

    let renounced = owner == DEAD1 || owner == DEAD2;

    (
        token_name,
        contract_balance,
        total_supply,
        total_supply_formatted,
        decimals,
        owner,
        renounced,
    )
}


async fn get_deployer<P: Provider>(provider: P, creation_tx_hash: &str) -> Address {
    let hash = match TxHash::from_str(creation_tx_hash) {
        Ok(h) => h,
        Err(_) => return Address::ZERO
    };

    match provider.get_transaction_by_hash(hash).await {
        Ok(Some(txn)) => txn.from(),
        Ok(None) => Address::ZERO,
        Err(_) => Address::ZERO
    }
}
