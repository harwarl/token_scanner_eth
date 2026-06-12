use alloy::{
    primitives::{Address, B256, keccak256},
    providers::Provider,
    rpc::types::Block,
};

use hex::decode;

use crate::utils::{
    constant::{MIN_ETH_LIQUIDITY, UNISWAP_FACTORY, WETH},
    contracts::IUniswapV2Pair,
};

pub fn get_univ2_pair_address(token_a: &Address, token_b: &Address) -> Address {
    let (token0, token1) = if token_a < token_b {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    };

    let mut salt_data = Vec::new();
    salt_data.extend_from_slice(token0.as_slice());
    salt_data.extend_from_slice(token1.as_slice());
    let salt: B256 = keccak256(&salt_data);

    // Init Code Hash (Standard Ethereum Uniswap V2 value)
    let init_code_hash_hex = "96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f";
    let init_code_hash = B256::from_slice(&decode(init_code_hash_hex).unwrap());

    // ABI Encode Packed the CREATE2 parameters
    let mut encoded = Vec::new();
    encoded.push(0xff); // 0xff prefix
    encoded.extend_from_slice(UNISWAP_FACTORY.as_slice());
    encoded.extend_from_slice(salt.as_slice());
    encoded.extend_from_slice(init_code_hash.as_slice());

    // Compute final pair address
    let hash = keccak256(&encoded);
    let mut pair_address_bytes = [0u8; 20];
    pair_address_bytes.copy_from_slice(&hash[12..32]); // Address is the last 20 bytes of the hash
    let pair_address = Address::from(pair_address_bytes);

    pair_address
}

pub fn address_match(token_a: Address, token_b: Address) -> bool {
    token_a.to_checksum(None).to_lowercase() == token_b.to_checksum(None).to_lowercase()
}

pub fn format_number(n: f64) -> String {
    if n >= 1_000_000_000.0 {
        format!("{:.2}B", n / 1_000_000_000.0)
    } else if n >= 1_000_000.0 {
        format!("{:.2}M", n / 1_000_000.0)
    } else if n >= 1_000.0 {
        format!("{:.2}K", n / 1_000.0)
    } else {
        format!("{:.2}", n)
    }
}

pub fn calculate_volatility(volume_24h: f64, liquidity_usd: f64) -> f64 {
    if liquidity_usd == 0.0 {
        return 0.0;
    }
    (volume_24h / liquidity_usd) * 100.0
}

pub fn volatility_label(v: f64) -> &'static str {
    match v {
        v if v < 100.0 => "🟢 Low",
        v if v < 300.0 => "🟡 Medium",
        v if v < 1000.0 => "🟠 High",
        _ => "🔴 Extreme",
    }
}

pub async fn get_block<P>(primary: &P, fallback: &P, block_number: u64) -> eyre::Result<Block>
where
    P: Provider,
{
    match primary
        .get_block_by_number(block_number.into())
        .full()
        .await
    {
        Ok(Some(block)) => Ok(block),
        _ => {
            tracing::warn!(
                "Primary RPC failed for block {}, trying fallback",
                block_number
            );
            fallback
                .get_block_by_number(block_number.into())
                .hashes()
                .await?
                .ok_or_else(|| eyre::eyre!("Block {} not found on fallback", block_number))
        }
    }
}

pub async fn has_enough_liquidity<P>(provider: &P, pair_address: Address, token0: Address) -> bool
where
    P: Provider,
{
    let pair = IUniswapV2Pair::new(pair_address, provider);

    let (reserve0, reserve1) = match pair.getReserves().call().await {
        Ok(r) => (r._reserve0, r._reserve1),
        Err(e) => {
            tracing::warn!("Failed to get reserves for {:?}: {e}", pair_address);
            return false;
        }
    };

    let eth_reserve = if token0 == WETH {
        reserve0.to::<u128>()
    } else {
        reserve1.to::<u128>()
    };

    if eth_reserve < MIN_ETH_LIQUIDITY {
        tracing::info!(
            "Skipping low liquidity pair {:?}: {} wei ETH",
            pair_address,
            eth_reserve
        );
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_univ2_weth_usdc_pair() {
        let weth: Address = "0xC02aaA39b223FE8D0A0E5C4F27eAD9083C756Cc2"
            .parse()
            .unwrap();

        let usdc: Address = "0xA0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
            .parse()
            .unwrap();

        let expected: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc"
            .parse()
            .unwrap();

        let calculated = get_univ2_pair_address(&weth, &usdc);

        assert_eq!(calculated, expected);
    }
}
