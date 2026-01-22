use chrono::{TimeZone, Utc};

use crate::{
    commands::Args,
    handlers::convert::{ConvertArg, ConvertArgs},
};

#[test]
fn empty_args_parsing() {
    let args = Args("".into());
    let ret: ConvertArgs = args.try_into().unwrap();
    
    match ret {
        ConvertArgs::Empty => (),
        _ => panic!("Expected Empty variant"),
    }
}

#[test]
fn valid_convert_with_integer_amount() {
    let args = Args("USD 1000 ; IDR".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("USD", ret.from_currency);
    assert_eq!("1000", ret.from_amount);
    assert_eq!("IDR", ret.to_currency);
    assert_eq!(None, ret.date);
}

#[test]
fn valid_convert_with_comma_separated_amount() {
    let args = Args("USD 50,000 ; IDR".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("USD", ret.from_currency);
    assert_eq!("50,000", ret.from_amount);
    assert_eq!("IDR", ret.to_currency);
}

#[test]
fn valid_convert_with_decimal_amount() {
    let args = Args("BTC 0.5 ; USD".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("BTC", ret.from_currency);
    assert_eq!("0.5", ret.from_amount);
    assert_eq!("USD", ret.to_currency);
}

#[test]
fn valid_convert_with_comma_and_decimal() {
    let args = Args("USD 1,234,567.89 ; EUR".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("USD", ret.from_currency);
    assert_eq!("1,234,567.89", ret.from_amount);
    assert_eq!("EUR", ret.to_currency);
}

#[test]
fn lowercase_currency_codes() {
    let args = Args("usd 100 ; idr".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("USD", ret.from_currency);
    assert_eq!("100", ret.from_amount);
    assert_eq!("IDR", ret.to_currency);
}

#[test]
fn mixed_case_currency_codes() {
    let args = Args("UsD 500 ; IdR".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("USD", ret.from_currency);
    assert_eq!("500", ret.from_amount);
    assert_eq!("IDR", ret.to_currency);
}

#[test]
fn extra_whitespace_handling() {
    let args = Args("  USD   1000   ;   IDR  ".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("USD", ret.from_currency);
    assert_eq!("1000", ret.from_amount);
    assert_eq!("IDR", ret.to_currency);
}

#[test]
fn invalid_missing_semicolon() {
    let args = Args("USD 1000 IDR".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn valid_convert_with_date() {
    let args = Args("USD 1000 ; IDR ; 2022-02-02".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("USD", ret.from_currency);
    assert_eq!("1000", ret.from_amount);
    assert_eq!("IDR", ret.to_currency);
    assert_eq!(
        Utc.with_ymd_and_hms(2022, 2, 2, 0, 0, 0).unwrap(),
        ret.date.unwrap()
    );
}

#[test]
fn invalid_missing_amount() {
    let args = Args("USD ; IDR".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_too_many_tokens_in_from() {
    let args = Args("USD 1000 extra ; IDR".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_from_currency_too_short() {
    let args = Args("US 1000 ; IDR".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_from_currency_too_long() {
    let args = Args("USDD 1000 ; IDR".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_to_currency_too_short() {
    let args = Args("USD 1000 ; ID".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_to_currency_too_long() {
    let args = Args("USD 1000 ; IDRR".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_amount_with_letters() {
    let args = Args("USD 100abc ; IDR".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_amount_with_special_chars() {
    let args = Args("USD 1000$ ; IDR".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_negative_amount() {
    let args = Args("USD -1000 ; IDR".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn valid_convert_with_date_and_comma_amount() {
    let args = Args("USD 50,000 ; IDR ; 2023-12-25".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("USD", ret.from_currency);
    assert_eq!("50,000", ret.from_amount);
    assert_eq!("IDR", ret.to_currency);
    assert_eq!(
        Utc.with_ymd_and_hms(2023, 12, 25, 0, 0, 0).unwrap(),
        ret.date.unwrap()
    );
}

#[test]
fn valid_convert_with_date_lowercase() {
    let args = Args("btc 0.5 ; usd ; 2024-01-15".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("BTC", ret.from_currency);
    assert_eq!("0.5", ret.from_amount);
    assert_eq!("USD", ret.to_currency);
    assert_eq!(
        Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap(),
        ret.date.unwrap()
    );
}

#[test]
fn valid_convert_with_date_extra_whitespace() {
    let args = Args("  EUR   1000.50   ;   GBP   ;   2025-06-30  ".into());
    let ret: ConvertArg = args.try_into().unwrap();
    
    assert_eq!("EUR", ret.from_currency);
    assert_eq!("1000.50", ret.from_amount);
    assert_eq!("GBP", ret.to_currency);
    assert_eq!(
        Utc.with_ymd_and_hms(2025, 6, 30, 0, 0, 0).unwrap(),
        ret.date.unwrap()
    );
}

#[test]
fn invalid_date_format() {
    let args = Args("USD 1000 ; IDR ; 02-02-2022".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_date_format_wrong_separator() {
    let args = Args("USD 1000 ; IDR ; 2022/02/02".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_date_text() {
    let args = Args("USD 1000 ; IDR ; yesterday".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_too_many_parts() {
    let args = Args("USD 1000 ; IDR ; 2022-02-02 ; extra".into());
    let ret: Result<ConvertArg, _> = args.try_into();
    assert!(ret.is_err());
}
