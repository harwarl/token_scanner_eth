use alloy::primitives::{Address, address};

// pub const WETHUSDTV2PAIR: Address = address!("0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852");
pub const WETH: Address = address!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
pub const UNISWAPV2_ROUTER: Address = address!("0x7a250d5630b4cf539739df2c5dacb4c659f2488d");
pub const DEAD1: Address = address!("0x0000000000000000000000000000000000000000");
pub const DEAD2: Address = address!("0x000000000000000000000000000000000000dead");
pub const TEAMFINANCE: Address = address!("0xe2fe530c047f2d85298b07d9333c05737f1435fb");
pub const UNICRYPT: Address = address!("0x663A5C229c09b049E36dCc11a9B0d4a8Eb9db214");
pub const PINKLOCK: Address = address!("0x71B5759d73262FBb223956913ecF4ecC51057641");
pub const UNISWAP_FACTORY: Address = address!("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
pub const UNISWAP_V4_POOL_MANAGER: Address = address!("0x000000000004444c5dc75cB358380D2e3dE08A90");
pub const MIN_ETH_LIQUIDITY: u128 = 1_000_000_000_000_000u128;
pub const USDC: Address = address!("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
pub const USDT: Address = address!("0xdAC17F958D2ee523a2206206994597C13D831ec7");
pub const BASE_USDC: Address = address!("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
pub const BASE_USDT: Address = address!("0xdAC17F958D2ee523a2206206994597C13D831ec7");

pub const BASE_TOKENS: [Address; 3] = [WETH, USDC, USDT];

#[derive(Debug, Clone)]
pub struct Contracts {
    pub v2_router: Address,
    pub v2_factory: Address,
    pub v4_pool_manager: Address,
    pub weth: Address,
    pub usdt: Address,
    pub usdc: Address,
}

pub const MAINNET: Contracts = Contracts {
    v2_factory: address!("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f"),
    v2_router: address!("0x7a250d5630b4cf539739df2c5dacb4c659f2488d"),
    v4_pool_manager: address!("0x000000000004444c5dc75cB358380D2e3dE08A90"),
    weth: address!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
    usdt: address!("0xdAC17F958D2ee523a2206206994597C13D831ec7"),
    usdc: address!("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
};

pub const BASE: Contracts = Contracts {
    v2_factory: address!("0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6"),
    v2_router: address!("0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24"),
    v4_pool_manager: address!("0x000000000004444c5dc75cB358380D2e3dE08A90"),
    weth: address!("0x4200000000000000000000000000000000000006"),
    usdt: address!("0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2"),
    usdc: address!("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"),
};

// public free RPC URLs
pub const ETH_FREE_RPCS: &[&str] = &[
    "https://eth.blockrazor.xyz",
    "https://ethereum-rpc.publicnode.com",
    "https://ethereum.public.blockpi.network/v1/rpc/public",
    "https://0xrpc.io/eth",
    "https://ethereum-json-rpc.stakely.io",
    "https://rpc.fullsend.to",
    "https://api.zan.top/eth-mainnet",
    "https://eth.llamarpc.com",
    "https://rpc.payload.de",
    "https://endpoints.omniatech.io/v1/eth/mainnet/public",
    "https://rpc.public.curie.radiumblock.co/ws/ethereum",
    "https://rpc.polysplit.cloud/v1/chain/1",
    "https://eth.merkle.io",
];

pub const BASE_FREE_RPCS: &[&str] = &[
    "https://rpc.baseazul.dev",
    "https://base-public.nodies.app",
    "https://1rpc.io/base",
    "https://base-rpc.publicnode.com",
    "https://base.meowrpc.com",
    "https://base.api.pocket.network",
    "https://base.drpc.org",
    "https://rpc.nodeflare.app/base/public",
];
