use rust_decimal::{prelude::FromPrimitive, Decimal};

pub fn lamports_to_sol(lamports: u64, ndigits: u8) -> Decimal {
    let sol = lamports as f64 / 10i32.pow(9) as f64;
    Decimal::from_f64(sol).unwrap().round_dp(ndigits as u32)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_lamports_to_sol() {
        assert_eq!(lamports_to_sol(5046063475073, 4), Decimal::from_str("5046.0635").unwrap())
    }
}
