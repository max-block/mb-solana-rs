use crate::account::keypair_bytes_to_string;
use colored_json::ToColoredJson;
use itertools::{Either, Itertools};
use serde_yaml::Value;
use solana_sdk::signature::{Keypair, Signer};
use std::process;

pub fn print_colored_json(data: &str) {
    println!("{}", data.to_colored_json_auto().unwrap());
}

pub fn print_keypair(keypair: Keypair) {
    println!("public: {}", keypair.pubkey());
    println!("private base58: {}", keypair.to_base58_string());
    println!("private arr: {}", keypair_bytes_to_string(keypair.to_bytes()));
}

pub fn print_fatal(message: impl AsRef<str>) -> ! {
    eprintln!("fatal error: {}", message.as_ref());
    process::exit(1);
}

pub fn str_to_vec(data: &str, lower: bool, remove_comments: bool, unique: bool) -> Vec<String> {
    let data = if lower { data.to_lowercase() } else { data.into() };
    let result = data.split('\n').into_iter().map(|x| x.trim().to_string()).filter(|x| !x.is_empty());
    let result = if unique { Either::Right(result.unique()) } else { Either::Left(result) };
    let result = if remove_comments {
        Either::Right(result.map(|x| x.split('#').next().unwrap().trim().to_string()))
    } else {
        Either::Left(result)
    };

    result.collect()
}

pub fn replace_str_with_vec(data: &mut Value, field: &str, lower: bool, remove_comments: bool, unique: bool) -> Option<()> {
    let field_value = data.get(field)?.as_str()?;
    let new_data: Vec<Value> =
        str_to_vec(field_value, lower, remove_comments, unique).iter().map(|x| Value::String(x.to_string())).collect_vec();
    data.as_mapping_mut()?.insert(serde_yaml::Value::String(field.to_string()), serde_yaml::Value::Sequence(new_data));
    None
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::str_to_vec;

    #[test]
    fn f1() {
        let data = "\
        A # 1
        b

        c
        b
        ";
        assert_eq!(str_to_vec(&data, false, false, false), vec!["A # 1", "b", "c", "b"]);
        assert_eq!(str_to_vec(&data, true, true, true), vec!["a", "b", "c"]);
    }
}
