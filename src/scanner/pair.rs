use alloy::{
    primitives::Address,
    providers::Provider,
    rpc::types::Log,
    sol_types::{SolCall, SolEvent},
};
use teloxide::Bot;

use crate::{
    etherscan::client::EtherscanClient,
    scanner::pair,
    utils::{
        constant::WETH,
        contracts::{IUniswapV2Factory, IUniswapV2Pair::getReservesCall}, helpers::has_enough_liquidity,
    },
};

pub async fn decode_pair<P>(
    provider: &P,
    log: &Log,
    bot: &Bot,
    etherscan_client: &EtherscanClient,
) where
    P: Provider,
{
    if let Ok(new_pair_event) = IUniswapV2Factory::PairCreated::decode_log(log.inner.as_ref()) {
        let pair_address = new_pair_event.pair;
        let token0 = new_pair_event.token0;
        let token1 = new_pair_event.token1;

        let new_token = if token0 == WETH { token1 } else { token0 };

        // check if the token has enough liquidity
        if !has_enough_liquidity(provider, pair_address, new_token).await {
            return
        };

        




        
    }
}
