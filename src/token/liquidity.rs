use alloy::{primitives::Address, providers::Provider};

use crate::utils::{constant::WETH, contracts::IUniswapV2Pair};

pub async fn get_liquidity<P: Provider>(
    provider: &P,
    pair_address: &Address,
    token0: Address,
    eth_price: f64,
) -> f64 {
    let pair = IUniswapV2Pair::new(*pair_address, provider);
    let reserves = match pair.getReserves().call().await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to get reserves for pair {:?}: {e}", pair_address);
            return 0.0;
        }
    };

    let weth_reserve = if token0 == WETH {
        reserves._reserve0.to::<u128>() as f64 / 1e18
    } else {
        reserves._reserve1.to::<u128>() as f64 / 1e18
    };

    let liquidity_usd = weth_reserve * 2.0 * eth_price;

    liquidity_usd
}
