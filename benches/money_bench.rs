// Benchmarks run on fixed, known-good inputs; unwrap failures abort the
// bench run visibly, which is the desired behavior here.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_money::{Currency, CurrencyAmount, FormatConfig, InMemoryFxProvider};
use rust_decimal_macros::dec;

fn bench_add(c: &mut Criterion) {
    c.bench_function("currency_amount_add", |b| {
        let a = CurrencyAmount::new(dec!(19.99), Currency::USD);
        let summand = CurrencyAmount::new(dec!(1.60), Currency::USD);
        b.iter(|| (a.clone() + summand.clone()).unwrap());
    });
}

fn bench_format_us(c: &mut Criterion) {
    let config = FormatConfig::us();
    let amount = CurrencyAmount::new(dec!(1234.56), Currency::USD);
    c.bench_function("format_us", |b| {
        b.iter(|| config.format(&amount));
    });
}

fn bench_format_european(c: &mut Criterion) {
    let config = FormatConfig::european();
    let amount = CurrencyAmount::new(dec!(1234.56), Currency::EUR);
    c.bench_function("format_european", |b| {
        b.iter(|| config.format(&amount));
    });
}

fn bench_format_iso(c: &mut Criterion) {
    let config = FormatConfig::iso();
    let amount = CurrencyAmount::new(dec!(1234.56), Currency::USD);
    c.bench_function("format_iso", |b| {
        b.iter(|| config.format(&amount));
    });
}

fn bench_fx_conversion(c: &mut Criterion) {
    let mut provider = InMemoryFxProvider::new();
    provider
        .set_rate(Currency::USD, Currency::EUR, dec!(0.85))
        .expect("valid FX rate for benchmark setup");
    let amount = CurrencyAmount::new(dec!(100.00), Currency::USD);

    c.bench_function("fx_rate_conversion", |b| {
        b.iter(|| decimal_money::convert(&amount, Currency::EUR, &provider).unwrap());
    });
}

criterion_group!(
    benches,
    bench_add,
    bench_format_us,
    bench_format_european,
    bench_format_iso,
    bench_fx_conversion,
);
criterion_main!(benches);
