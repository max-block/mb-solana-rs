mod helper;

use helper::*;
use rstest::rstest;
use rust_decimal::Decimal;

#[rstest]
#[tokio::test]
async fn sol_balance(rpc_url: String, account: String) {
    let res = mb_solana::rpc::sol_balance(&rpc_url, &account, 5, None).await;
    assert!(res.unwrap() > 0);
}


#[rstest]
#[tokio::test]
async fn spl_token_balance(rpc_url: String, account: String, token: String) {
    let res = mb_solana::rpc::spl_token_balance(&rpc_url, &account, &token, 5, None).await;
    assert!(res.unwrap() > Decimal::from(0));
}
