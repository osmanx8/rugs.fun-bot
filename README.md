# 🎰 Rugs.fun Trading Bot
A high-speed, automated Solana trading bot tailored for Rugs.fun — the degen meme casino. This bot listens to live game rounds via WebSocket and executes auto-buy/sell strategies based on your config.


---

## 📬 Contact

 [Telegram](https://t.me/ShadowRusii)

---

🚀 Features
🔄 Live Game Detection: Subscribes to Rugs.fun rounds via on-chain WebSocket updates.

⚡ Auto Buy & Sell: Executes buys at round start, sells based on multiplier or delay.

🛠️ Easy Config: Tune behavior via config.toml.

🧠 Custom Logic Ready: Extend for strategy changes or CPI integration.

🦀 Blazingly Fast: Written in Rust using Solana SDK for performance.

---


---

## 🛠️ Installation & Setup

### 1️⃣ Install Dependencies

Ensure you have Rust and Solana CLI installed:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.11/install)"

# Confirm install
solana --version
cargo --version
```

### 2️⃣ Configure the Bot

Create a config.toml in the project root:

```bash
rpc_url = "https://api.mainnet-beta.solana.com"
keypair_path = "/home/youruser/.config/solana/id.json"
program_id = "REPLACE_WITH_RUGS_FUN_PROGRAM_ID"
market_account = "REPLACE_WITH_MARKET_ACCOUNT"
buy_amount_lamports = 10000000        # e.g. 0.01 SOL
target_multiplier = 1.2               # Auto-sell multiplier
```

### 3️⃣ Run the Bot

```bash
cargo build --release
cargo run --release
```

Watch logs for transaction signatures and status updates.

### 🧠 How It Works
🛰️ Connects to the Rugs.fun market account over WebSocket.

🔔 Detects a new round trigger in real-time.

🛒 Sends a buy transaction for configured amount.

⏳ Waits until price hits multiplier or timeout.

💸 Sends a sell transaction (currently mock/transfer — should be updated to CPI).

📊 Logs every action clearly.

### ⚠️ Important Notes
🔐 Always test on Solana Devnet before mainnet deployment.

📍 Replace placeholder Program IDs & Market Account addresses with actual ones.

🧪 Current sell logic uses system_instruction::transfer — you should replace it with actual CPI logic from Rugs.fun when ready.

🔁 Add retries, error handling, and safety checks for production.
