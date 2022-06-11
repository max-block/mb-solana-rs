use std::str::FromStr;

use rand::prelude::SliceRandom;
use reqwest::blocking::Client;
use rust_decimal::Decimal;
use serde_json::{json, Value};
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

#[derive(TypedBuilder)]
pub struct SplTokenBalanceParams {
    pub owner: String,
    pub mint: String,

    #[builder(default = vec!["https://api.mainnet-beta.solana.com".to_string()])]
    pub nodes: Vec<String>,

    #[builder(default = 5)]
    pub attempts: u8,

    #[builder(default = 10)]
    pub timeout: u8,
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

pub fn spl_token_balance(params: SplTokenBalanceParams) -> Option<Decimal> {
    let mut rng = rand::thread_rng();
    let request_data = json!({"jsonrpc": "2.0", "id": 1, "method": "getTokenAccountsByOwner", "params": [params.owner, {"mint": params.mint}, {"encoding": "jsonParsed"}]});
    for _ in 0..params.attempts {
        let node = params.nodes.choose(&mut rng).unwrap();
        let res = Client::new().post(node).json(&request_data).send().unwrap();
        let res: Value = res.json().unwrap();
        if let Some(amount) = res.pointer("/result/value/0/account/data/parsed/info/tokenAmount/uiAmountString") {
            if let Ok(amount) = Decimal::from_str(amount.as_str().unwrap()) {
                return Some(amount);
            }
        }
    }
    None
}
