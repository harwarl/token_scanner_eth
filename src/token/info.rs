use alloy::{
    primitives::{Address, U256, Uint, b256},
    providers::Provider,
};

use crate::utils::{constant::WETH, contracts::IERC20};

pub async fn get_token_info<P: Provider>(
    provider: &P,
    pair_address: &Address,
    token0: Address,
    token1: Address,
) -> (String, Uint<256, 4>, Uint<256, 4>, f64, u8) {
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
    
    ( token_name, contract_balance, total_supply, total_supply_formatted, decimals)
}
