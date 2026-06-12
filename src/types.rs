use alloy::primitives::{Address, U256};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub name: String,
    pub address: Address,
    pub total_supply: f64,
    pub verified: bool,
    pub liquidity_usd: f64,
    pub lp_locked: bool,
    pub renounced: bool,
    pub buy_tax: f64,
    pub sell_tax: f64,
    pub market_cap_usd: f64,
    pub honeypot: bool,
    pub deployer: Address,
    pub deployer_age_days: u64,
    pub is_fresh_wallet: bool,
    pub bad_reputation: bool,
    pub contract_name: String,
    pub mcap_to_liq_ratio: f64,
    pub holder_count: u64,
    // Volume
    pub volume_5m: f64,
    pub volume_1h: f64,
    pub volume_6h: f64,
    pub volume_24h: f64,
    // Transactions
    pub buys_5m: u32,
    pub sells_5m: u32,
    pub buys_1h: u32,
    pub sells_1h: u32,
    pub buys_6h: u32,
    pub sells_6h: u32,
    pub buys_24h: u32,
    pub sells_24h: u32,
    // Price
    pub price_usd: f64,
    pub price_change_5m: f64,
    pub price_change_1h: f64,

    // Volatility
    pub volatility: f64,

    // socials
    pub socials: String,
}

#[derive(Debug)]
pub struct PartialTokenInfo {
    pub token_address: Address,
    pub pair_address: Address,
    pub token0: Address,
    pub token1: Address,

    // From token contract calls
    pub name: String,
    pub symbol: String,
    pub total_supply: f64,
    pub renounced: bool,
    pub deployer: Address,
}

#[derive(Debug, Clone)]
pub struct TokenMetaInfo {
    pub name: String,
    pub address: Address,
    pub symbol: String,
    pub total_supply: U256,
    pub total_supply_formatted: f64,
    pub decimals: u8,
    pub owner: Address,
    pub renounced: bool,
}

#[derive(Debug, Clone)]
pub struct SwapInfo {
    pub pair: Address,
    pub token0: Address,
    pub token1: Address,
    pub buy_count: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationResult {
    pub buy_tax: f64,
    pub sell_tax: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoneyPotResult {
    pub is_honeypot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub creation_tx_hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoneyPotResponse {
    pub simulation_success: bool,
    pub simulation_result: SimulationResult,
    pub honeypot_result: HoneyPotResult,
    pub pair: Pair,
}

#[derive(Debug, Deserialize)]
pub struct ContractCreation {
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(rename = "contractCreator")]
    pub contract_creator: String,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct EtherscanResponse<T> {
    pub status: String,
    pub result: T,
}

#[derive(Debug, Deserialize)]
pub struct SourceCode {
    #[serde(rename = "SourceCode")]
    pub source_code: String,
    #[serde(rename = "ContractName")]
    pub contract_name: String,
    #[serde(rename = "CompilerVersion")]
    pub compiler_version: String,
}

#[derive(Debug)]
pub struct ContractInfo {
    pub verified: bool,
    pub contract_name: String,
    pub deployer: Address,
    pub tx_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
}

#[derive(Debug)]
pub struct WalletInfo {
    pub age_days: u64,
    pub deployed_contracts: Vec<Address>,
    pub is_fresh_wallet: bool,
}

#[derive(Debug, Deserialize)]
pub struct DexscreenerVolume {
    pub price_usd: f64,
    pub market_cap: f64,
    pub liquidity_usd: f64,
    pub vol_5m: f64,
    pub vol_1h: f64,
    pub vol_6h: f64,
    pub vol_24h: f64,
    pub buys_5m: u32,
    pub sells_5m: u32,
    pub buys_1h: u32,
    pub sells_1h: u32,
    pub buys_6h: u32,
    pub sells_6h: u32,
    pub buys_24h: u32,
    pub sells_24h: u32,
    pub price_change_5m: f64,
    pub price_change_1h: f64,
    pub price_change_6h: f64,
    pub price_change_24h: f64,
    pub holder_count: u64,
    pub website: Option<String>,
    pub x: Option<String>,
    pub telegram: Option<String>,
}
