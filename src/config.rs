use solana_sdk::pubkey::Pubkey;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub keypair_path: String,
    pub program_id: Pubkey,
    pub market_account: Pubkey,
    pub buy_amount_lamports: u64,
    pub target_multiplier: f64,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let cfg_str = std::fs::read_to_string("config.toml")?;
        let cfg: Config = toml::from_str(&cfg_str)?;
        Ok(cfg)
    }
}
