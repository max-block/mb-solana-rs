use std::{env, str::FromStr};

use mb_solana::transfer::transfer_sol_sync;
use rust_decimal::Decimal;

fn main() {
    dotenv::dotenv().ok();
    let rpc_url = env::var("RPC_URL").unwrap();
    let private_key = env::var("PRIVATE_KEY").unwrap();
    let recipient = env::var("RECIPIENT").unwrap();
    let amount = Decimal::from_str("0.1234").unwrap();
    let res = transfer_sol_sync(&rpc_url, &private_key, &recipient, amount);
    dbg!(res.unwrap());
}
