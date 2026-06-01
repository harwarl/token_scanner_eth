use alloy::{
    primitives::{Address, U256, Uint},
    providers::Provider,
};

use crate::utils::{
    constant::{WETH, WETHUSDTV2PAIR},
    contracts::IUniswapV2Pair,
};

pub async fn get_eth_price<P: Provider>(provider: P) -> f64 {
    let pair = IUniswapV2Pair::new(WETHUSDTV2PAIR, provider);
    let reserves = pair.getReserves().call().await.unwrap();

    // reserve0 = WETH (18 decimals), reserve1 = USDT (6 decimals)
    let reserve_weth = reserves._reserve0.to::<u128>() as f64 / 1e18;
    let reserve_usdt = reserves._reserve1.to::<u128>() as f64 / 1e6;

    reserve_usdt / reserve_weth
}

pub async fn get_market_cap<P: Provider>(
    provider: &P,
    token0: Address,
    token1: Address,
    total_supply: Uint<256, 4>,
    decimals: u8,
    pair_address: Address,
    eth_price: f64
) -> f64 {
    let pair = IUniswapV2Pair::new(pair_address, provider);

    let reserves = match pair.getReserves().call().await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to get reserves: {e}");
            return 0.0;
        }
    };

    let token = if token0 == WETH { token1 } else { token0 };

    let (weth_reserve, token_reserve) = if token0 == WETH {
        (reserves._reserve0, reserves._reserve1)
    } else {
        (reserves._reserve1, reserves._reserve0)
    };

    let divisor = U256::from(10u64).pow(U256::from(decimals));
    let total_supply_f64 = total_supply.to::<u128>() as f64 / divisor.to::<u128>() as f64;
    let token_reserve_f64 = token_reserve.to::<u128>() as f64 / divisor.to::<u128>() as f64;
    let weth_reserve_f64 = weth_reserve.to::<u128>() as f64 / 1e18;


    let market_cap = total_supply_f64 * weth_reserve_f64 / token_reserve_f64;

    // Market price in $
    market_cap * eth_price
}
