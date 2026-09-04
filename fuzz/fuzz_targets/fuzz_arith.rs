#![no_main]

use decimal_money::{Currency, CurrencyAmount, FxRate, InMemoryFxProvider, convert};
use libfuzzer_sys::fuzz_target;
use rust_decimal::Decimal;

/// Build a bounded Decimal from fuzz bytes: an i64 mantissa (always within
/// Decimal's i96 storage) and a scale capped at 27 (Decimal caps at 28).
fn bounded_decimal(bytes: &[u8]) -> Decimal {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(bytes);
    let mantissa = i64::from_be_bytes(buf);
    let scale = (buf[7] % 28) as u32;
    Decimal::from_i128_with_scale(mantissa as i128, scale)
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 17 {
        return;
    }
    let a = bounded_decimal(&data[0..8]);
    let b = bounded_decimal(&data[8..16]);
    let pick = data[16];

    // Checked arithmetic on raw decimals — overflow must be None, not panic.
    let _ = a.checked_add(b);
    let _ = a.checked_sub(b);
    let _ = a.checked_mul(b);
    let _ = a.checked_div(b);

    // CurrencyAmount operators — same currency computes, mismatched returns
    // Err, and overflow paths must never panic.
    let usd: Currency = "USD".parse().unwrap();
    let eur: Currency = "EUR".parse().unwrap();
    let x = CurrencyAmount::new(a, usd);
    let y_same = CurrencyAmount::new(b, usd);
    let y_diff = CurrencyAmount::new(b, eur);
    let _ = x.clone() + y_same.clone();
    let _ = x.clone() - y_same;
    let _ = x.clone() + y_diff.clone();
    let _ = x.clone() - y_diff;

    // FX path: FxRate validation (Err on zero rate), inverse, conversion
    // against an empty provider (Err on missing rate) — never panic.
    let from = if pick & 1 == 0 { usd } else { eur };
    let to = if pick & 1 == 0 { eur } else { usd };
    if let Ok(rate) = FxRate::new(from, to, a) {
        let _ = rate.inverse();
        let _ = convert(&CurrencyAmount::new(b, from), to, &InMemoryFxProvider::new());
    }
    let _ = FxRate::new(from, to, Decimal::ZERO);
});
