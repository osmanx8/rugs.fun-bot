use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Signer, Keypair},
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction,
};
use crate::config::Config;

pub fn buy(
    client: &RpcClient,
    payer: &Keypair,
    market: &Pubkey,
    amount: u64,
) -> anyhow::Result<String> {
    let ix = system_instruction::transfer(&payer.pubkey(), market, amount);
    let recent = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], recent);
    let sig = client.send_and_confirm_transaction(&tx)?;
    Ok(sig.to_string())
}

pub fn sell(
    client: &RpcClient,
    payer: &Keypair,
    market: &Pubkey,
) -> anyhow::Result<String> {
    // Replace with actual Rugs.fun sell instruction
    let ix = system_instruction::transfer(market, &payer.pubkey(), 1);
    let recent = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], recent);
    let sig = client.send_and_confirm_transaction(&tx)?;
    Ok(sig.to_string())
}
