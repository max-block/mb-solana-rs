use std::{str::FromStr, time::Duration};

use anyhow::Context;
use reqwest::{Client, Proxy};
use rust_decimal::Decimal;
use serde_json::{json, Value};

pub async fn sol_balance(node: &str, address: &str, timeout: u8, proxy: Option<&str>) -> anyhow::Result<u64> {
    let request_data = json!({"jsonrpc":"2.0", "id":1, "method":"getBalance", "params":[address]});
    let mut client = Client::builder().timeout(Duration::from_secs(timeout as u64));
    if proxy.is_some() {
        client = client.proxy(Proxy::all(proxy.unwrap())?);
    }

    let res = client.build()?.post(node).json(&request_data).send().await?;
    let res: Value = res.json().await.context("invalid json response")?;
    if let Some(amount) = res.pointer("/result/value") {
        if amount.is_i64() {
            return Ok(amount.as_i64().unwrap() as u64);
        }
    }
    Err(anyhow::Error::msg("can't parse response"))
}

pub async fn spl_token_balance(node: &str, owner: &str, mint: &str, timeout: u8, proxy: Option<&str>) -> anyhow::Result<Decimal> {
    let request_data = json!({"jsonrpc": "2.0", "id": 1, "method": "getTokenAccountsByOwner", "params": [owner, {"mint": mint}, {"encoding": "jsonParsed"}]});
    let mut client = Client::builder().timeout(Duration::from_secs(timeout as u64));
    if proxy.is_some() {
        client = client.proxy(Proxy::all(proxy.unwrap())?);
    }

    let res = client.build()?.post(node).json(&request_data).send().await?;
    let res: Value = res.json().await.context("invalid json response")?;
    if let Some(amount) = res.pointer("/result/value/0/account/data/parsed/info/tokenAmount/uiAmountString") {
        if let Ok(amount) = Decimal::from_str(amount.as_str().unwrap()) {
            return Ok(amount);
        }
    }
    Err(anyhow::Error::msg("can't parse response"))
}
