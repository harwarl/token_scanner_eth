use alloy::{
    primitives::Address,
    providers::Provider,
    rpc::types::Log,
    sol_types::{SolCall, SolEvent},
};
use teloxide::Bot;

use crate::{
    etherscan::client::EtherscanClient, scanner::pair, token::info::get_token_info, types::{PartialTokenInfo, TokenMetaInfo}, utils::{
        constant::WETH,
        contracts::{IUniswapV2Factory, IUniswapV2Pair::getReservesCall}, helpers::has_enough_liquidity,
    }
};

pub async fn decode_pair<P>(
    provider: &P,
    log: &Log,
) -> Option<PartialTokenInfo>
where
    P: Provider,
{
    if let Ok(new_pair_event) = IUniswapV2Factory::PairCreated::decode_log(log.inner.as_ref()) {
        let pair_address = new_pair_event.pair;
        let token0 = new_pair_event.token0;
        let token1 = new_pair_event.token1;

        let token_address = if token0 == WETH { token1 } else { token0 };

        // check if the token has enough liquidity
        if !has_enough_liquidity(provider, pair_address, token_address).await {
            return None;
        }

        // Get onchain Token info
        let token_info= get_token_info(provider, token0, token1).await;

        return Some(
            PartialTokenInfo {
                token_address,
                pair_address,
                token0,
                token1,
                name: token_info.name,
                symbol: token_info.symbol,
                total_supply: token_info.total_supply_formatted,
                renounced: token_info.renounced, 
                deployer: token_info.owner
            }
        )
    }
    None
}
