use std::sync::Arc;

use alloy::{providers::Provider, rpc::types::Log, sol_types::SolEvent};

use crate::{
    token::info::get_token_info,
    types::PartialTokenInfo,
    utils::{constant::Contracts, contracts::IUniswapPoolManager},
};

pub async fn decode_pool<P>(
    log: &Log,
    provider: &P,
    chain_contracts: Arc<Contracts>,
) -> Option<PartialTokenInfo>
where
    P: Provider,
{
    if let Ok(pool_event) = IUniswapPoolManager::Initialize::decode_log(log.inner.as_ref()) {
        let currency0 = pool_event.currency0;
        let currency1 = pool_event.currency1;

        let base_tokens = [chain_contracts.usdt, chain_contracts.usdc, chain_contracts.weth];

        // Filter - Only WETH, USDC and USDT
        let token_address = if base_tokens.contains(&currency0) {
            currency1
        } else if base_tokens.contains(&currency1) {
            currency0
        } else {
            return None;
        };

        let token_info = get_token_info(provider, token_address).await;

        println!("Token Info: {:?}", token_info);

        return Some(PartialTokenInfo {
            token_address,
            pair_address: pool_event.address,
            token0: currency0,
            token1: currency1,
            name: token_info.name,
            symbol: token_info.symbol,
            total_supply: token_info.total_supply_formatted,
            renounced: token_info.renounced,
            deployer: token_info.owner,
        });
    }

    None
}
