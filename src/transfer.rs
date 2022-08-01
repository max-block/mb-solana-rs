use std::{str::FromStr, time::Duration};

use rust_decimal::{prelude::ToPrimitive, Decimal};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{native_token::sol_to_lamports, pubkey::Pubkey, signature::Keypair, system_transaction};
use tokio::runtime::Builder;

pub async fn transfer_sol(rpc_url: &str, private_key_base58: &str, recipient: &str, amount: Decimal) -> anyhow::Result<String> {
    let client = RpcClient::new_with_timeout(rpc_url.to_string(), Duration::from_secs(10));
    let keypair = Keypair::from_base58_string(private_key_base58);
    let recent_blockhash = client.get_latest_blockhash().await?;
    let lamports = sol_to_lamports(amount.to_f64().unwrap());
    let tx = system_transaction::transfer(&keypair, &Pubkey::from_str(recipient)?, lamports, recent_blockhash);
    let res = client.send_transaction(&tx).await?;
    Ok(res.to_string())
}

pub fn transfer_sol_sync(
    rpc_url: &str,
    private_key_base58: &str,
    recipient_address: &str,
    amount: Decimal,
) -> anyhow::Result<String> {
    Builder::new_multi_thread().enable_all().build().unwrap().block_on(transfer_sol(
        rpc_url,
        private_key_base58,
        recipient_address,
        amount,
    ))
}
