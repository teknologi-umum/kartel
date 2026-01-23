use std::str::FromStr;

use accounting::Accounting;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub fn format_money_str(code: &str, money_str: &str) -> String {
    let mut ac = Accounting::new_from_seperator(code, 2, ",", ".");

    ac.set_format("{s} {v}");

    let amount = Decimal::from_str(money_str).unwrap_or(dec!(0));
    let money_display = ac.format_money(amount);

    money_display
}
