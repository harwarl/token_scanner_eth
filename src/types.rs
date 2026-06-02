use alloy::primitives::U256;
use ethers::abi::Address;
use serde::{Deserialize, Serialize};

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
    pub deployer: String,
}

#[derive(Debug, Clone)]
pub struct SwapInfo {
    pub pair: Address,
    pub token0: Address,
    pub token1: Address,
    pub buy_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct SimulationResult {
    pub buy_tax: f64,
    pub sell_tax: f64,
}

#[derive(Debug, Deserialize)]
pub struct HoneyPotResult {
    pub is_honey_pot: bool,
}

#[derive(Debug, Deserialize)]
pub struct HoneyPotResponse {
    pub simulation_success: bool,
    pub simulation_result: SimulationResult,
    pub honeypot_result: HoneyPotResult,
}
