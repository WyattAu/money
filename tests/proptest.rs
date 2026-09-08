//! Property-based tests for decimal-money crate.

// Property tests exercise hostile inputs directly; unwrap/expect, slicing,
// and panicking asserts are the test signal here.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use proptest::prelude::*;
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

// Allocation properties (largest-remainder) ----------------------------

fn arb_amount() -> impl Strategy<Value = Decimal> {
    // Mantissa with up to 4 decimal places, both signs.
    (-1_000_000_000_000i64..=1_000_000_000_000).prop_flat_map(|mantissa| {
        (0u32..=4).prop_map(move |scale| Decimal::from_i128_with_scale(mantissa as i128, scale))
    })
}

fn arb_ratios() -> impl Strategy<Value = Vec<Decimal>> {
    prop::collection::vec(
        (0u32..=3, 0i64..=10_000)
            .prop_map(|(scale, mantissa)| Decimal::from_i128_with_scale(mantissa as i128, scale)),
        1..=8,
    )
    // At least one positive ratio required; filter all-zero vectors.
    .prop_filter("ratios must not all be zero", |rs| {
        rs.iter().any(|r| !r.is_zero())
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    #[test]
    fn allocate_parts_sum_exactly(amount in arb_amount(), ratios in arb_ratios()) {
        let total = CurrencyAmount::new(amount, Currency::USD);
        let parts = total.allocate(&ratios).unwrap();
        prop_assert_eq!(parts.len(), ratios.len());
        let sum: Decimal = parts.iter().map(|p| p.amount).sum();
        prop_assert_eq!(sum, total.amount);
        for part in &parts {
            prop_assert_eq!(part.currency, Currency::USD);
        }
    }

    #[test]
    fn allocate_single_ratio_is_identity(amount in arb_amount()) {
        let total = CurrencyAmount::new(amount, Currency::USD);
        let parts = total.allocate(&[Decimal::ONE]).unwrap();
        prop_assert_eq!(parts.len(), 1);
        prop_assert_eq!(parts[0].amount, total.amount);
    }

    #[test]
    fn allocate_parts_close_to_exact_share(amount in arb_amount(), ratios in arb_ratios()) {
        prop_assume!(amount.is_sign_positive() && !amount.is_zero());
        let total = CurrencyAmount::new(amount, Currency::USD);
        let parts = total.allocate(&ratios).unwrap();
        let weight_sum: Decimal = ratios.iter().sum();
        prop_assume!(!weight_sum.is_zero());
        for (part, ratio) in parts.iter().zip(ratios.iter()) {
            let exact = total.amount * ratio / weight_sum;
            // Within one minor unit of the exact share.
            let diff = (part.amount - exact).abs();
            prop_assert!(
                diff <= Decimal::new(1, total.amount.scale()),
                "part {} not within one unit of exact share {exact}",
                part.amount
            );
        }
    }

    #[test]
    fn allocate_negative_amount_sums_exactly(amount in arb_amount(), ratios in arb_ratios()) {
        prop_assume!(amount.is_sign_negative() && !amount.is_zero());
        let total = CurrencyAmount::new(amount, Currency::USD);
        let parts = total.allocate(&ratios).unwrap();
        let sum: Decimal = parts.iter().map(|p| p.amount).sum();
        prop_assert_eq!(sum, total.amount);
    }

    #[test]
    fn allocate_zero_sum_ratios_rejected(amount in arb_amount()) {
        let total = CurrencyAmount::new(amount, Currency::USD);
        prop_assert!(total.allocate(&[Decimal::ZERO, Decimal::ZERO]).is_err());
    }

    #[test]
    fn allocate_negative_ratio_rejected(amount in arb_amount(), neg in -1_000i64..0) {
        let total = CurrencyAmount::new(amount, Currency::USD);
        let ratios = [Decimal::from(neg), Decimal::ONE];
        prop_assert!(total.allocate(&ratios).is_err());
    }

    #[test]
    fn allocate_scale_variant_ratios_sum_exactly(
        mantissa in -10_000_000i64..=10_000_000,
        r1_scale in 0u32..=4,
        r1_mantissa in 0i64..=999,
        r2_scale in 0u32..=4,
        r2_mantissa in 0i64..=999,
    ) {
        prop_assume!(r1_mantissa > 0 || r2_mantissa > 0);
        let total = CurrencyAmount::new(
            Decimal::from_i128_with_scale(mantissa as i128, 4),
            Currency::EUR,
        );
        let r1 = Decimal::from_i128_with_scale(r1_mantissa as i128, r1_scale);
        let r2 = Decimal::from_i128_with_scale(r2_mantissa as i128, r2_scale);
        let parts = total.allocate(&[r1, r2]).unwrap();
        let sum: Decimal = parts.iter().map(|p| p.amount).sum();
        prop_assert_eq!(sum, total.amount);
    }
}
