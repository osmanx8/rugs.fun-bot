use tokio::sync::broadcast;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::read_keypair_file;
use std::sync::Arc;
use crate::{config::Config, listener::watch_round_start, buy_sell::{buy, sell}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::load()?;
    let payer = read_keypair_file(&cfg.keypair_path)?;
    let client = RpcClient::new(cfg.rpc_url.clone());
    let market = cfg.market_account;
    let (tx, mut rx) = broadcast::channel::<()>(16);

    // Spawn listener for new round detection
    tokio::spawn(watch_round_start(
        &cfg.rpc_url.replace("https", "ws"),
        market,
        tx.clone(),
    ));

    let client_arc = Arc::new(client);
    let payer_arc = Arc::new(payer);

    println!("Listening for new Rugs.fun rounds...");
    while rx.recv().await.is_ok() {
        println!("📈 New round detected – buying low");
        let sig = buy(&client_arc, &payer_arc, &market, cfg.buy_amount_lamports)?;
        println!("Buy tx: {}", sig);

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        println!("🔁 Attempting to sell");
        match sell(&client_arc, &payer_arc, &market) {
            Ok(sig2) => println!("Sold: {}", sig2),
            Err(e) => println!("Sell failed (rugg pull?): {}", e),
        }
    }

    Ok(())
}
