# token_scanner_eth

A high-performance Ethereum token scanner built in Rust. Monitors every block in real time, detects new Uniswap V2 token launches, analyzes on-chain data, and delivers alpha signals directly to Telegram — filtering out honeypots, rugs, and low-quality launches before you ever see them.

---

## How It Works

```
WebSocket Block Stream
        ↓
  Fetch Full Block
        ↓
  Loop Transactions
        ↓
  Decode Swap Logs (Uniswap V2)
        ↓
  Validate Pair (token0/token1, WETH check, CREATE2 verification)
        ↓
  Count Buys / Unique Buyers / Volume (last 2 blocks)
        ↓
  Filter: buy_count >= 20
        ↓
  Gather Token Intelligence
  ├── Token metadata (name, supply, decimals, owner)
  ├── Liquidity & market cap
  ├── Honeypot check (honeypot.is API)
  ├── LP lock status
  ├── Etherscan: contract verified, deployer, contract age
  └── Etherscan: deployer wallet age, reputation, deployed contracts
        ↓
  Build TokenInfo
        ↓
  Send Telegram Alert
```

---

## Features

- Real-time block scanning via WebSocket (`wss://`)
- Uniswap V2 pair detection and CREATE2 address verification
- Buy count, unique buyer count, buy/sell ratio, and USD volume analysis
- Honeypot detection via [honeypot.is](https://honeypot.is) API
- LP lock detection (TeamFinance, Pinklock, Unicrypt, DEAD addresses)
- Contract verification check via Etherscan
- Deployer wallet age and reputation analysis
- Market cap / liquidity ratio calculation
- Telegram bot alerts with full token intelligence

---

## Tech Stack

- **Language:** Rust (Edition 2024)
- **Async Runtime:** Tokio
- **Ethereum:** Alloy (WebSocket provider, ABI decoding, typed bindings)
- **HTTP Client:** Reqwest
- **Telegram:** Teloxide
- **Logging:** Tracing + tracing-subscriber
- **External APIs:** Etherscan, honeypot.is

---

## Project Structure

```
token_scanner_eth/
├── Cargo.toml
├── .env
└── src/
    ├── main.rs                   # Entry point — block stream loop
    ├── config.rs                 # Config struct, env var validation, URL checks
    ├── types.rs                  # TokenInfo, HoneypotResponse, shared types
    │
    ├── provider/
    │   └── mod.rs                # WebSocket and HTTP provider setup
    │
    ├── scanner/
    │   ├── mod.rs
    │   └── block.rs              # analyze_block — tx loop, log decoding
    │
    ├── swap/
    │   ├── mod.rs
    │   └── decode.rs             # decode_swap — buy count, unique buyers, volume
    │
    ├── token/
    │   ├── mod.rs
    │   ├── info.rs               # name, supply, decimals, owner, renounced
    │   ├── honeypot.rs           # honeypot.is API client
    │   ├── liquidity.rs          # reserves, liquidity USD
    │   └── market.rs             # market cap, ETH price
    │
    ├── lp/
    │   ├── mod.rs
    │   └── lp_lock.rs            # LP lock detection
    │
    ├── etherscan/
    │   ├── mod.rs
    │   ├── client.rs             # Base Etherscan HTTP client
    │   ├── contract.rs           # Contract verified, deployer, creation tx
    │   └── wallet.rs             # Wallet age, deployed contracts, reputation
    │
    ├── telegram/
    │   ├── mod.rs
    │   └── bot.rs                # Message formatting, send_tg_message
    │
    └── utils/
        ├── mod.rs
        ├── constant.rs           # WETH, DEAD addresses, known lock contracts
        └── contracts.rs          # sol! ABI bindings (IERC20, IUniswapV2Pair)
```

---

## TokenInfo Fields

| Field                 | Source        | Description                            |
| --------------------- | ------------- | -------------------------------------- |
| `name`                | On-chain      | ERC20 token name                       |
| `address`             | On-chain      | Token contract address                 |
| `total_supply`        | On-chain      | Formatted total supply                 |
| `verified`            | Etherscan     | Contract source code verified          |
| `contract_name`       | Etherscan     | Contract name from source              |
| `liquidity_usd`       | On-chain      | Total liquidity in USD                 |
| `lp_locked`           | On-chain      | LP tokens locked in known lockers      |
| `renounced`           | On-chain      | Owner is zero/dead address             |
| `buy_tax`             | honeypot.is   | Simulated buy tax %                    |
| `sell_tax`            | honeypot.is   | Simulated sell tax %                   |
| `market_cap_usd`      | On-chain      | Market cap in USD                      |
| `mcap_to_liq_ratio`   | Computed      | Market cap / liquidity ratio           |
| `honeypot`            | honeypot.is   | Is token a honeypot                    |
| `deployer`            | honeypot.is   | Deployer wallet address                |
| `buy_count`           | On-chain logs | Buy transactions in last 2 blocks      |
| `total_swaps`         | On-chain logs | Total swaps in last 2 blocks           |
| `buy_ratio`           | Computed      | buy_count / total_swaps                |
| `unique_buyers_count` | On-chain logs | Unique buyer wallets                   |
| `volume_usd`          | On-chain logs | USD volume in last 2 blocks            |
| `deployer_age_days`   | Etherscan     | Days since deployer's first tx         |
| `is_fresh_wallet`     | Etherscan     | Deployer wallet < 30 days old          |
| `bad_reputation`      | Etherscan     | Deployer has unverified past contracts |

---

## Getting Started

### Prerequisites

- Rust (latest stable)
- A WebSocket Ethereum RPC endpoint (Alchemy, Infura, or public)
- Etherscan API key
- Telegram bot token (via [@BotFather](https://t.me/BotFather))

### Installation

```bash
git clone https://github.com/Harwarl/token_scanner_eth
cd token_scanner_eth
```

### Environment Variables

Create a `.env` file in the project root:

```env
RPC_URL=https://your-eth-rpc-endpoint
RPC_URL_WSS=wss://your-eth-wss-endpoint
ETHERSCAN_API_KEY=your_etherscan_api_key
TELOXIDE_TOKEN=your_telegram_bot_token
```

| Variable            | Description                                                   |
| ------------------- | ------------------------------------------------------------- |
| `RPC_URL`           | HTTP Ethereum RPC endpoint (must be `http://` or `https://`)  |
| `RPC_URL_WSS`       | WebSocket Ethereum RPC endpoint (must be `ws://` or `wss://`) |
| `ETHERSCAN_API_KEY` | Etherscan API key for contract and wallet lookups             |
| `TELOXIDE_TOKEN`    | Telegram bot token from BotFather                             |

### Running

```bash
cargo run
```

### Running with Auto-Reload

```bash
cargo install cargo-watch
cargo watch -c -x run
```

---

## Telegram Alert Format

```
🌞 TokenName
📜 CA: 0x...

🎎 Supply: 1.00B | 🏆 MCap: $120,000.00
💧 Liquidity: $45,000.00 | 📊 MCap/Liq: 2.67x
📈 Volume: $32,000.00 | 👥 Unique Buyers: 67
🔄 Buys: 48 / 60 | 💹 Buy Ratio: 80%

🏁 Verified: ✅ | 🏠 Renounced: ✅
🔒 LP Lock: ✅ | 🍯 Honeypot: ✅ No
💰 Tax: Buy 0% | Sell 0%

👩‍🍳 Deployer: 0x...
📅 Deployer Age: 180 days | 🆕 Fresh Wallet: ✅ No
⚠️ Bad Reputation: ✅ No

📈 Dextools | DexScreener | DexSpy | DexView
```

---

## Alpha Signal Parameters

The scanner is tuned to identify early high-conviction token launches:

| Parameter            | Target     | Signal                               |
| -------------------- | ---------- | ------------------------------------ |
| `buy_count`          | >= 20      | Momentum in last 2 blocks            |
| `buy_ratio`          | >= 60%     | More buys than sells                 |
| `unique_buyers`      | >= 50      | Real community, not bots             |
| `liquidity_usd`      | $10K–$500K | Serious but still early              |
| `mcap_to_liq_ratio`  | <= 5x      | Price not inflated on thin liquidity |
| `buy_tax / sell_tax` | 0%         | Quality projects launch frictionless |
| `honeypot`           | false      | Non-negotiable safety check          |
| `lp_locked`          | true       | Reduces rug risk                     |
| `deployer_age_days`  | >= 30      | Not a throwaway wallet               |
| `bad_reputation`     | false      | No prior rugged contracts            |
| `volume_usd`         | >= $50K    | Real money moving                    |

---

## Dependencies

| Crate          | Version | Purpose                              |
| -------------- | ------- | ------------------------------------ |
| `alloy`        | 2.0.5   | Ethereum provider, ABI, primitives   |
| `tokio`        | 1.52.3  | Async runtime                        |
| `teloxide`     | 0.17.0  | Telegram bot                         |
| `reqwest`      | 0.13.4  | HTTP client (honeypot.is, Etherscan) |
| `serde`        | 1.0.228 | Serialization/deserialization        |
| `tracing`      | 0.1     | Structured logging                   |
| `thiserror`    | 2.0.18  | Error types                          |
| `futures-util` | 0.3.32  | Stream utilities                     |
| `dotenv`       | 0.15.0  | Environment variable loading         |
| `url`          | 2.5.8   | URL validation                       |

---

## License

MIT

---

## Author

**Oduwale Awwal** — [GitHub](https://github.com/Harwarl) · [Twitter](https://twitter.com/_Harwarl)
