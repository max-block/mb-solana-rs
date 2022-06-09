use std::str::FromStr;

use rand::prelude::SliceRandom;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct SolBalanceParams {
    pub account: String,

    #[builder(default = vec!["https://api.mainnet-beta.solana.com".to_string()])]
    pub nodes: Vec<String>,

    #[builder(default = 5)]
    pub attempts: u8,
}

pub fn sol_balance(params: SolBalanceParams) -> Option<u64> {
    let pubkey = Pubkey::from_str(&params.account).ok()?;
    let mut rng = rand::thread_rng();
    for _ in 0..params.attempts {
        let client = RpcClient::new(params.nodes.choose(&mut rng).unwrap());
        if let Ok(balance) = client.get_balance(&pubkey) {
            return Some(balance);
        }
    }
    None
}
