use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::pubkey::Pubkey;
use tokio::sync::broadcast;

pub async fn watch_round_start(
    ws_url: &str,
    market: Pubkey,
    tx: broadcast::Sender<()>,
) -> anyhow::Result<()> {
    let (mut client, receiver) = PubsubClient::account_subscribe(
        ws_url,
        &market,
        Some(RpcAccountInfoConfig::default()),
    ).await?;

    tokio::spawn(async move {
        let mut prev_data: Vec<u8> = Vec::new();
        while let Some(update) = receiver.recv().await {
            if update.value.data != prev_data {
                prev_data = update.value.data.clone();
                // TODO: Decode if this signals round start
                let _ = tx.send(());
            }
        }
    });

    Ok(())
}
