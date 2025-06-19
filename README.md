# 🪙 Rugs.fun Trading Rust Bot

A Solana-based automated trading bot for **Rugs.fun** — detects new game rounds in real-time via WebSocket, and automatically buys and sells based on configurable parameters.

> ⚠️ **High-risk degen project for entertainment purposes. Use at your own risk.**

---

## 📬 Contact

Feel free to reach out for collaborations, feature requests, or questions: [Telegram](https://t.me/shiny0103) | [Twitter](https://x.com/0xTan1319)

---

## 📌 Features

- 🔍 Detects new Rugs.fun game rounds live via on-chain WebSocket updates.
- ⚡ Automatically places buy orders (bets) when a new round starts.
- 📤 Attempts to sell after a configurable delay or multiplier threshold.
- 📝 Easy-to-edit config file (`config.toml`)
- 🦀 Fully written in Rust using Solana SDK.

---

## 📂 Project Structure

rugs_bot/
├── Cargo.toml
├── README.md
├── config.toml
└── src/
├── main.rs
├── config.rs
├── buy_sell.rs
└── listener.rs


---

## 🛠️ Installation & Setup

### 1️⃣ Install Dependencies

Ensure you have Rust and Solana CLI installed:

```bash
rustup install stable
solana --version

cargo build
```

### 2️⃣ Configure the Bot

Create a config.toml in the project root:

```bash
rpc_url = "https://api.mainnet-beta.solana.com"
keypair_path = "/home/youruser/.config/solana/id.json"
program_id = "REPLACE_WITH_RUGS_FUN_PROGRAM_ID"
market_account = "REPLACE_WITH_MARKET_ACCOUNT"
buy_amount_lamports = 10000000  # 0.01 SOL
target_multiplier = 1.2
```

### 3️⃣ Run the Bot

```bash

cargo run --release

```

Watch logs for transaction signatures and status updates.

### 📈 How It Works

- Subscribes to the Rugs.fun market account via WebSocket.

- Detects new round triggers by monitoring account data changes.

- Automatically buys in at the start of each round.

- Waits a set duration or multiplier.

- Attempts to sell before the rug happens.

- Logs all actions with transaction hashes.

### ⚙️ Notes

- Replace placeholder Program ID and Market Account with actual Rugs.fun program addresses.

- Default buy/sell instructions currently use system_instruction::transfer for demonstration — update these to actual Rugs.fun CPI instructions.

- Test on Solana devnet before using on mainnet with real SOL.

- Add error handling, retries, and multipliers logic as needed for production use.

### 🙏 Credits

Developed by [0xTan1319](https://github.com/0xTan1319)  
Inspired by the degen Solana meme casino at [Rugs.fun](https://rugs.fun)