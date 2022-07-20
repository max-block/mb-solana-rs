use std::env;

use mb_solana::balance::{spl_token_balance_sync, SplTokenBalanceParams};

fn main() {
    dotenv::dotenv().ok();
    let owner = env::var("ACCOUNT").unwrap();
    let mint = env::var("TOKEN").unwrap();
    let res = spl_token_balance_sync(SplTokenBalanceParams::builder().owner(owner).mint(mint).attempts(1).build());
    dbg!(res);
}
