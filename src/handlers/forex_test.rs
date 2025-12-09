
use chrono::{TimeZone, Utc};

use crate::{
    commands::Args,
    handlers::forex::{BaseRatesArg, SinglePairArg},
};

#[test]
fn argument_parsing_test() {
    let args = Args("USD/IDR".into());

    let ret: SinglePairArg = args.try_into().unwrap();
    dbg!(&ret);
    assert_eq!("USD".to_string(), ret.left);
    assert_eq!("IDR".to_string(), ret.right);
    assert_eq!(None, ret.date);

    let args = Args("BTC/IDR 2022-02-02".into());

    let ret: SinglePairArg = args.try_into().unwrap();
    dbg!(&ret);
    assert_eq!("BTC".to_string(), ret.left);
    assert_eq!("IDR".to_string(), ret.right);
    assert_eq!(
        Utc.with_ymd_and_hms(2022, 2, 2, 0, 0, 0).unwrap(),
        ret.date.unwrap()
    );
}

#[test]
fn lowercase_parsing_test() {
    let args = Args("usd/idr".into());

    let ret: SinglePairArg = args.try_into().unwrap();
    assert_eq!("USD".to_string(), ret.left);
    assert_eq!("IDR".to_string(), ret.right);
    assert_eq!(None, ret.date);
}

#[test]
fn mixed_case_parsing_test() {
    let args = Args("IdR/Usd".into());

    let ret: SinglePairArg = args.try_into().unwrap();
    assert_eq!("IDR".to_string(), ret.left);
    assert_eq!("USD".to_string(), ret.right);
    assert_eq!(None, ret.date);

    let args = Args("btc/IDR 2023-12-25".into());

    let ret: SinglePairArg = args.try_into().unwrap();
    assert_eq!("BTC".to_string(), ret.left);
    assert_eq!("IDR".to_string(), ret.right);
    assert_eq!(
        Utc.with_ymd_and_hms(2023, 12, 25, 0, 0, 0).unwrap(),
        ret.date.unwrap()
    );
}

#[test]
fn invalid_format_single_currency() {
    let args = Args("USD".into());
    let ret: Result<SinglePairArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_format_wrong_length() {
    let args = Args("US/IDR".into());
    let ret: Result<SinglePairArg, _> = args.try_into();
    assert!(ret.is_err());

    let args = Args("USD/IDRR".into());
    let ret: Result<SinglePairArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_format_no_separator() {
    let args = Args("USDIDR".into());
    let ret: Result<SinglePairArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn invalid_format_invalid_separator() {
    let args = Args("USD&IDR".into());
    let ret: Result<SinglePairArg, _> = args.try_into();
    assert!(ret.is_err());
}

#[test]
fn valid_currency_no_date() {
    let args = Args("usd".into());
    let parsed = BaseRatesArg::try_from(args).expect("should parse lowercase currency");
    assert_eq!(parsed.base, "USD"); // always uppercase
    assert!(parsed.date.is_none());
}

#[test]
fn valid_currency_uppercase_with_date() {
    let args = Args("IDR 2022-02-02".into());
    let parsed = BaseRatesArg::try_from(args).expect("should parse uppercase currency + date");
    assert_eq!(parsed.base, "IDR");

    let expected = Utc.ymd(2022, 2, 2).and_hms(0, 0, 0);
    assert_eq!(parsed.date.unwrap(), expected);
}

#[test]
fn valid_currency_mixed_case_with_date() {
    let args = Args("uSd 2025-12-08".into());
    let parsed = BaseRatesArg::try_from(args).expect("should parse mixed-case currency + date");
    assert_eq!(parsed.base, "USD"); // normalized to uppercase

    let expected = Utc.ymd(2025, 12, 8).and_hms(0, 0, 0);
    assert_eq!(parsed.date.unwrap(), expected);
}

#[test]
fn invalid_currency_format() {
    // too short
    assert!(BaseRatesArg::try_from(Args("US".into())).is_err());

    // contains slash -> invalid
    assert!(BaseRatesArg::try_from(Args("USD/IDR".into())).is_err());

    // contains digit
    assert!(BaseRatesArg::try_from(Args("U2D".into())).is_err());
}

#[test]
fn invalid_date_format() {
    let res = BaseRatesArg::try_from(Args("USD wrongdate".into()));
    assert!(res.is_err());
}
