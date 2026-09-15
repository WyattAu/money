//! Config-knob behavior matrix for `decimal-money`.
//!
//! Knobs: `FormatConfig`'s five fields, `RoundingPolicy`'s five variants,
//! and the currency's `decimal_places`. Each must observably change the
//! formatted/rounded output. In-module tests cover these individually;
//! this file is the consolidated per-knob table (default vs configured).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use decimal_money::{Currency, CurrencyAmount, FormatConfig, RoundingPolicy, SymbolPosition};
use rust_decimal_macros::dec;

fn usd(v: DecimalWrapper) -> CurrencyAmount {
    CurrencyAmount::new(v.0, Currency::USD)
}
struct DecimalWrapper(rust_decimal::Decimal);
fn d(v: rust_decimal::Decimal) -> DecimalWrapper {
    DecimalWrapper(v)
}

// --- FormatConfig knobs -----------------------------------------------------

#[test]
fn knob_symbol_position_prefix_vs_suffix() {
    let mut cfg = FormatConfig::us();
    let amount = usd(d(dec!(1234.56)));

    cfg.symbol_position = SymbolPosition::Prefix;
    let prefix = cfg.format(&amount);
    cfg.symbol_position = SymbolPosition::Suffix;
    let suffix = cfg.format(&amount);

    assert_eq!(prefix, "$1,234.56");
    assert_eq!(suffix, "1,234.56 $");
    assert_ne!(prefix, suffix, "symbol_position must change output");
}

#[test]
fn knob_thousands_separator_changes_output() {
    let mut cfg = FormatConfig::us();
    let amount = usd(d(dec!(1234.56)));

    cfg.thousands_separator = ',';
    assert_eq!(cfg.format(&amount), "$1,234.56");

    cfg.thousands_separator = ' ';
    assert_eq!(
        cfg.format(&amount),
        "$1 234.56",
        "changing the separator knob must change output"
    );
}

#[test]
fn knob_decimal_separator_changes_output() {
    let mut cfg = FormatConfig::us();
    let amount = usd(d(dec!(1234.56)));

    cfg.decimal_separator = '.';
    assert_eq!(cfg.format(&amount), "$1,234.56");

    cfg.decimal_separator = ',';
    assert_eq!(cfg.format(&amount), "$1,234,56");
}

#[test]
fn knob_show_symbol_on_vs_off() {
    let mut cfg = FormatConfig::us();
    let amount = usd(d(dec!(1234.56)));

    cfg.show_symbol = true;
    assert_eq!(cfg.format(&amount), "$1,234.56");

    cfg.show_symbol = false;
    assert_eq!(cfg.format(&amount), "1,234.56");
}

#[test]
fn knob_use_iso_code_symbol_vs_iso() {
    let mut cfg = FormatConfig::us();
    let amount = usd(d(dec!(1234.56)));

    cfg.use_iso_code = false;
    let with_symbol = cfg.format(&amount);
    cfg.use_iso_code = true;
    let with_iso = cfg.format(&amount);

    assert_eq!(with_symbol, "$1,234.56");
    assert_eq!(with_iso, "USD 1,234.56");
}

// --- RoundingPolicy knobs -----------------------------------------------------

#[test]
fn knob_rounding_policy_each_variant_changes_midpoint_output() {
    // 2.5 at zero places: each policy produces a distinct documented result.
    let amount = usd(d(dec!(2.5)));

    assert_eq!(
        amount
            .round_to_with_policy(0, RoundingPolicy::HalfUp)
            .amount,
        dec!(3),
        "HalfUp: 2.5 -> 3"
    );
    assert_eq!(
        amount
            .round_to_with_policy(0, RoundingPolicy::HalfEven)
            .amount,
        dec!(2),
        "HalfEven: 2.5 -> 2"
    );
    assert_eq!(
        amount
            .round_to_with_policy(0, RoundingPolicy::HalfDown)
            .amount,
        dec!(2),
        "HalfDown: 2.5 -> 2"
    );
    assert_eq!(
        amount.round_to_with_policy(0, RoundingPolicy::Floor).amount,
        dec!(2),
        "Floor: 2.5 -> 2"
    );
    assert_eq!(
        amount
            .round_to_with_policy(0, RoundingPolicy::Ceiling)
            .amount,
        dec!(3),
        "Ceiling: 2.5 -> 3"
    );
}

#[test]
fn knob_rounding_policy_floor_vs_ceiling_differ_off_midpoint() {
    // Policies agree at non-midpoints except Floor/Ceiling.
    let amount = usd(d(dec!(2.1)));
    assert_ne!(
        amount.round_to_with_policy(0, RoundingPolicy::Floor).amount,
        amount
            .round_to_with_policy(0, RoundingPolicy::Ceiling)
            .amount
    );
}

// --- Currency decimal_places knob -----------------------------------------------

#[test]
fn knob_currency_decimal_places_changes_format_width() {
    // JPY has 0 decimal places; USD has 2 — same numeric value formats differently.
    let jpy = CurrencyAmount::new(dec!(1234), Currency::JPY);
    let usd_amount = CurrencyAmount::new(dec!(1234), Currency::USD);

    let cfg = FormatConfig::us();
    let jpy_str = cfg.format(&jpy);
    let usd_str = cfg.format(&usd_amount);

    assert_eq!(jpy_str, "¥1,234");
    assert_eq!(usd_str, "$1,234.00");
    assert_ne!(jpy_str, usd_str);
}

#[test]
fn knob_currency_decimal_places_changes_rounding_output() {
    let jpy = CurrencyAmount::new(dec!(1234.567), Currency::JPY);
    let rounded = jpy.round_to(0);
    assert_eq!(rounded.amount, dec!(1235));
    assert_eq!(jpy.round_to(2).amount, dec!(1234.57));
}
