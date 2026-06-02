use alloy::{
    primitives::{Address, U256},
    providers::Provider,
};

use crate::utils::{
    constant::{DEAD1, DEAD2, PINKLOCK, TEAMFINANCE, UNICRYPT},
    contracts::IUniswapV2Pair,
};

const LOCK_THRESHOLD: u8 = 80;

pub async fn is_lp_locked<P: Provider>(pair_address: &Address, provider: P) -> bool {
    let pair = IUniswapV2Pair::new(*pair_address, provider);

    let total_supply = match pair.totalSupply().call().await {
        Ok(ts) => ts,
        Err(_) => return false,
    };

    if total_supply.is_zero() {
        return false;
    }

    let dead_lock1 = match pair.balanceOf(DEAD1).call().await {
        Ok(b) => b,
        Err(_) => U256::ZERO,
    };

    let dead_lock2 = match pair.balanceOf(DEAD2).call().await {
        Ok(b) => b,
        Err(_) => U256::ZERO,
    };

    let team_finance_lock = match pair.balanceOf(TEAMFINANCE).call().await {
        Ok(b) => b,
        Err(_) => U256::ZERO,
    };

    let unicrypt_lock = match pair.balanceOf(UNICRYPT).call().await {
        Ok(b) => b,
        Err(_) => U256::ZERO,
    };

    let pink_lock = match pair.balanceOf(PINKLOCK).call().await {
        Ok(b) => b,
        Err(_) => U256::ZERO,
    };

    let total_locked = team_finance_lock + pink_lock + unicrypt_lock + dead_lock1 + dead_lock2;

    let locked_pct = total_locked * U256::from(100) / total_supply;

    return locked_pct > U256::from(LOCK_THRESHOLD);
}
