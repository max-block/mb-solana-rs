use rstest::fixture;

#[fixture]
pub fn rpc_url() -> String {
    dotenv::dotenv().ok();
    std::env::var("RPC_URL").expect("can't get env RPC_URL")
}

#[fixture]
pub fn account() -> String {
    dotenv::dotenv().ok();
    std::env::var("ACCOUNT").expect("can't get env ACCOUNT")
}

#[fixture]
pub fn token() -> String {
    dotenv::dotenv().ok();
    std::env::var("TOKEN").expect("can't get env TOKEN")
}
