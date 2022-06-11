use std::fs::File;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use solana_sdk::native_token::lamports_to_sol;

use crate::{
    balance::{sol_balance, spl_token_balance, SolBalanceParams, SplTokenBalanceParams},
    cli::util::replace_str_with_vec,
};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    accounts: Vec<String>,
    tokens: Vec<String>,
    nodes: Vec<String>,
}

pub fn run(config_path: String) {
    let config = get_config(config_path);
    process_sol_balance(&config);
    for token in &config.tokens {
        process_spl_balance(&config, token);
    }
}

fn process_sol_balance(config: &Config) {
    let mut sum = 0.0;
    for acc in &config.accounts {
        let res = if let Some(balance) =
            sol_balance(SolBalanceParams::builder().account(acc.clone()).nodes(config.nodes.clone()).build())
        {
            let balance = lamports_to_sol(balance);
            sum += balance;
            format!("{:.2} SOL", balance)
        } else {
            "error".to_string()
        };
        println!("{}\t{}", acc, res);
    }
    println!("sum: {:.2}", sum);
}

fn process_spl_balance(config: &Config, token: &str) {
    println!("\ntoken: {}", token);
    let mut sum = Decimal::new(0, 0);
    for acc in &config.accounts {
        let res = if let Some(balance) = spl_token_balance(
            SplTokenBalanceParams::builder().owner(acc.clone()).mint(token.to_string()).nodes(config.nodes.clone()).build(),
        ) {
            sum += balance;
            format!("{:.2}", balance)
        } else {
            "error".to_string()
        };
        println!("{}\t{}", acc, res);
    }
    println!("sum: {:.2}", sum);
}

fn get_config(config_path: String) -> Config {
    let mut config: Value = serde_yaml::from_reader(File::open(config_path).unwrap()).unwrap();
    replace_str_with_vec(&mut config, "accounts", false, true, true);
    replace_str_with_vec(&mut config, "nodes", true, true, true);
    if config.get("tokens").is_none() {
        config["tokens"] = serde_yaml::Value::Sequence(Vec::new());
    }
    let config: Config = serde_yaml::from_value(config).unwrap();
    config
}
