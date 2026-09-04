#![no_main]

use decimal_money::{Currency, CurrencyAmount, FormatConfig};
use libfuzzer_sys::fuzz_target;
use rust_decimal::Decimal;
use std::str::FromStr;

fuzz_target!(|data: &[u8]| {
    // Bound input so parse attempts stay fast.
    let s = String::from_utf8_lossy(&data[..data.len().min(256)]);

    // Currency code parsing — malformed codes must be Err, never panic.
    let _ = Currency::from_str(&s);
    let _ = s.parse::<Currency>();

    // Amount parsing — invalid decimals must be Err, never panic.
    let _ = Decimal::from_str(&s);
    let _ = CurrencyAmount::from_str_values(&s, Currency::USD);

    // Formatting always succeeds and must not panic on any parsed value.
    if let Ok(d) = Decimal::from_str(&s) {
        for cfg in [FormatConfig::us(), FormatConfig::european(), FormatConfig::iso()] {
            let _ = cfg.format_decimal(&d, Currency::USD);
        }
    }
    if let Ok(amount) = CurrencyAmount::from_str_values(&s, Currency::EUR) {
        let _ = amount.to_string();
        let _ = FormatConfig::us().format(&amount);
    }
});
