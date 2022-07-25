use rand::{prelude::{SliceRandom, StdRng}, SeedableRng};
use rust_decimal::Decimal;
use tokio::runtime::Builder;
use typed_builder::TypedBuilder;

use crate::rpc;

#[derive(TypedBuilder)]
pub struct SolBalanceParams {
    pub account: String,

    #[builder(default = vec!["https://api.mainnet-beta.solana.com".to_string()])]
    pub nodes: Vec<String>,

    #[builder(default = vec![])]
    pub proxies: Vec<String>,

    #[builder(default = 5)]
    pub attempts: u8,

    #[builder(default = 10)]
    pub timeout: u8,
}

#[derive(TypedBuilder)]
pub struct SplTokenBalanceParams {
    pub owner: String,
    pub mint: String,

    #[builder(default = vec!["https://api.mainnet-beta.solana.com".to_string()])]
    pub nodes: Vec<String>,

    #[builder(default = vec![])]
    pub proxies: Vec<String>,

    #[builder(default = 5)]
    pub attempts: u8,

    #[builder(default = 10)]
    pub timeout: u8,
}

pub async fn sol_balance(params: SolBalanceParams) -> Option<u64> {
    let mut rng = StdRng::from_entropy();
    for _ in 0..params.attempts {
        let node = params.nodes.choose(&mut rng).unwrap();
        let proxy = params.proxies.choose(&mut rng).map(|p| p.as_str());
        if let Ok(balance) = rpc::sol_balance(node, &params.account, params.timeout, proxy).await {
            return Some(balance);
        }
    }
    None
}

pub fn sol_balance_sync(params: SolBalanceParams) -> Option<u64> {
    Builder::new_multi_thread().enable_all().build().unwrap().block_on(sol_balance(params))
}

pub async fn spl_token_balance(params: SplTokenBalanceParams) -> Option<Decimal> {
    let mut rng = StdRng::from_entropy();
    for _ in 0..params.attempts {
        let node = params.nodes.choose(&mut rng).unwrap();
        let proxy = params.proxies.choose(&mut rng).map(|p| p.as_str());
        if let Ok(balance) = rpc::spl_token_balance(node, &params.owner, &params.mint, params.timeout, proxy).await {
            return Some(balance);
        }
    }
    None
}

pub fn spl_token_balance_sync(params: SplTokenBalanceParams) -> Option<Decimal> {
    Builder::new_multi_thread().enable_all().build().unwrap().block_on(spl_token_balance(params))
}
