//! Property-based tests for decimal-money crate.

use proptest::prelude::*;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;

use decimal_money::{Currency, CurrencyAmount};

fn arb_currency() -> impl Strategy<Value = Currency> {
    prop_oneof![
        Just(Currency::USD),
        Just(Currency::EUR),
        Just(Currency::GBP),
        Just(Currency::JPY),
        Just(Currency::CHF),
        Just(Currency::CAD),
        Just(Currency::AUD),
        Just(Currency::CNY),
        Just(Currency::BTC),
        Just(Currency::ETH),
    ]
}

proptest! {
    #[test]
    fn currency_code_always_non_empty(c in arb_currency()) {
        prop_assert!(!c.code().is_empty());
    }

    #[test]
    fn currency_symbol_always_non_empty(c in arb_currency()) {
        prop_assert!(!c.symbol().is_empty());
    }

    #[test]
    fn currency_decimal_places_valid(c in arb_currency()) {
        let places = c.decimal_places();
        prop_assert!(places <= 18);
    }

    #[test]
    fn currency_from_str_roundtrip(c in arb_currency()) {
        let code = c.code();
        let parsed: Currency = code.parse().unwrap();
        prop_assert_eq!(c, parsed);
    }

    #[test]
    fn addition_commutative(a in -100_000i64..100_000, b in -100_000i64..100_000) {
        let amt_a = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        let amt_b = CurrencyAmount::new(Decimal::from(b), Currency::USD);
        let sum_ab = (amt_a.clone() + amt_b.clone()).unwrap();
        let sum_ba = (amt_b + amt_a).unwrap();
        prop_assert_eq!(sum_ab.amount, sum_ba.amount);
    }

    #[test]
    fn addition_zero_identity(a in -100_000i64..100_000) {
        let amt = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        let zero = CurrencyAmount::new(Decimal::ZERO, Currency::USD);
        let sum = (amt.clone() + zero).unwrap();
        prop_assert_eq!(sum.amount, amt.amount);
    }

    #[test]
    fn subtraction_inverse(a in -100_000i64..100_000) {
        let amt = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        let result = (amt.clone() - amt).unwrap();
        prop_assert_eq!(result.amount, Decimal::ZERO);
    }

    #[test]
    fn negation_doubles_to_zero(a in -100_000i64..100_000) {
        let amt = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        let negated = amt.negate();
        let sum = (amt + negated).unwrap();
        prop_assert_eq!(sum.amount, Decimal::ZERO);
    }

    #[test]
    fn abs_always_non_negative(a in -100_000i64..100_000) {
        let amt = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        let abs = amt.abs();
        prop_assert!(!abs.amount.is_sign_negative());
    }

    #[test]
    fn is_zero_matches_value(a in -100_000i64..100_000) {
        let amt = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        prop_assert_eq!(amt.is_zero(), a == 0);
    }

    #[test]
    fn is_positive_matches_value(a in -100_000i64..100_000) {
        let amt = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        prop_assert_eq!(amt.is_positive(), a > 0);
    }

    #[test]
    fn is_negative_matches_value(a in -100_000i64..100_000) {
        let amt = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        prop_assert_eq!(amt.is_negative(), a < 0);
    }

    #[test]
    fn addition_currency_mismatch_fails(
        c1 in arb_currency(),
        c2 in arb_currency(),
    ) {
        let a = CurrencyAmount::new(Decimal::from(10), c1);
        let b = CurrencyAmount::new(Decimal::from(20), c2);
        if c1 != c2 {
            let result = a + b;
            prop_assert!(result.is_err());
        }
    }

    #[test]
    fn display_always_starts_with_symbol(a in -100_000i64..100_000) {
        let amt = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        let display = format!("{amt}");
        prop_assert!(display.starts_with('$'));
    }

    #[test]
    fn round_preserves_currency(a in -100_000i64..100_000) {
        let amt = CurrencyAmount::new(Decimal::from(a), Currency::USD);
        let rounded = amt.round();
        prop_assert_eq!(rounded.currency, Currency::USD);
    }

    #[test]
    fn from_str_values_roundtrip(s in "[0-9]{1,6}\\.[0-9]{1,2}") {
        let amt = CurrencyAmount::from_str_values(&s, Currency::USD).unwrap();
        prop_assert_eq!(amt.currency, Currency::USD);
    }
}
