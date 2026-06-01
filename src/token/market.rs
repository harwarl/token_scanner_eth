use alloy::providers::Provider;

use crate::{utils::{constant::WETHUSDTV2PAIR, contracts::IUniswapV2Pair}};

pub async fn get_eth_price<P: Provider>(provider: P) -> f64 {
    let pair = IUniswapV2Pair::new(WETHUSDTV2PAIR, provider);
    let result = pair.getReserves().call().await.unwrap();

    // reserve0 = WETH (18 decimals), reserve1 = USDT (6 decimals)
    let reserve_weth = result._reserve0.to::<u128>() as f64 / 1e18;
    let reserve_usdt = result._reserve1.to::<u128>() as f64 / 1e6;

    reserve_usdt / reserve_weth
}
