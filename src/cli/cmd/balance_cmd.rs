use std::fs::File;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use solana_sdk::native_token::lamports_to_sol;

use crate::{
    balance::{sol_balance, SolBalanceParams},
    cli::util::replace_str_with_vec,
};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    accounts: Vec<String>,
    nodes: Vec<String>,
}

pub fn run(config_path: String) {
    let config = get_config(config_path);
    let mut sum = 0.0;
    for acc in config.accounts {
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

fn get_config(config_path: String) -> Config {
    let mut config: Value = serde_yaml::from_reader(File::open(config_path).unwrap()).unwrap();
    replace_str_with_vec(&mut config, "accounts", false, true, true);
    replace_str_with_vec(&mut config, "nodes", true, true, true);
    let config: Config = serde_yaml::from_value(config).unwrap();
    config
}
