use alloy::primitives::U256;
use ethers::abi::Address;

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub name: String, 
    pub address: Address,
    pub total_supply: U256,
    pub verified: bool, 
    pub liquidity_usd: f64,
    pub lp_lock: bool,
    pub renounced: bool,
    pub buy_tax: f64,
    pub sell_tax: f64,
    pub market_cap_usd: f64,
    pub honeypot: bool,
    pub deployer: String
}

#[derive(Debug, Clone)]
pub struct SwapInfo {
    pub pair: Address,
    pub token0: Address,
    pub token1: Address,
    pub buy_count: u32
}