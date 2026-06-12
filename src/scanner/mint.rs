use alloy::{primitives::Address, providers::Provider, rpc::types::Log, sol_types::SolEvent};

use crate::{
    token::info::get_token_info,
    types::PartialTokenInfo,
    utils::{
        constant::{MIN_ETH_LIQUIDITY, WETH},
        contracts::IUniswapV2Pair,
    },
};

pub async fn decode_mint<P>(
    provider: &P,
    log: &Log,
    token0: Address,
    token1: Address,
) -> Option<PartialTokenInfo>
where
    P: Provider,
{
    if let Ok(mint_event) = IUniswapV2Pair::Mint::decode_log(&log.inner.as_ref()) {
        
        let pair_address = log.address();

        // Check liquidity directly from the Mint event — no getReserves needed
        let eth_amount = if token0 == WETH {
            mint_event.amount0.to::<u128>()
        } else {
            mint_event.amount1.to::<u128>()
        };

        if eth_amount < MIN_ETH_LIQUIDITY {
            tracing::info!("Skipping low liquidity mint: {} wei", eth_amount);
            return None;
        }

        let token_address = if token0 == WETH { token1 } else { token0 };
        let token_meta = get_token_info(provider, token_address).await;

        return Some(PartialTokenInfo {
            token_address,
            pair_address,
            token0,
            token1,
            name: token_meta.name,
            symbol: token_meta.symbol,
            total_supply: token_meta.total_supply_formatted,
            renounced: token_meta.renounced,
            deployer: token_meta.owner,
        });
    }
    None
}
