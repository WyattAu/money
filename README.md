# decimal-money

Money types for Rust — `CurrencyAmount` with `Decimal` precision, multi-currency support, FX conversion, and formatting.

## Features

- **Decimal precision** — Uses `rust_decimal` for exact arithmetic
- **36+ currencies** — Fiat and crypto (USD, EUR, GBP, JPY, BTC, ETH, ...)
- **FX conversion** — Pluggable `FxProvider` trait with in-memory implementation
- **Formatting** — Configurable symbol position, separators, ISO codes
- **Type-safe** — Currency mismatch errors at runtime, not panics

## Quick Start

```rust
use money::{CurrencyAmount, Currency, FormatConfig};

let price = CurrencyAmount::new(rust_decimal_macros::dec!(19.99), Currency::USD);
let tax = CurrencyAmount::new(rust_decimal_macros::dec!(1.60), Currency::USD);
let total = (price + tax).unwrap();

assert_eq!(total.amount, rust_decimal_macros::dec!(21.59));
assert_eq!(format!("{total}"), "$21.59");
```

## Formatting

```rust
use money::{CurrencyAmount, Currency, FormatConfig};

let amount = CurrencyAmount::new(rust_decimal_macros::dec!(1234.56), Currency::EUR);

// US style: $1,234.56
let us = FormatConfig::us();

// European style: 1.234,56 €
let eu = FormatConfig::european();
assert_eq!(eu.format(&amount), "1.234,56 \u{20AC}");
```

## FX Conversion

```rust
use money::{CurrencyAmount, Currency, InMemoryFxProvider};
use rust_decimal::Decimal;

let mut provider = InMemoryFxProvider::new();
provider.set_rate(Currency::USD, Currency::EUR, Decimal::try_from("0.85").unwrap());

let dollars = CurrencyAmount::new(rust_decimal_macros::dec!(100), Currency::USD);
let euros = money::convert(&dollars, &provider).unwrap();
```

## Comparison with Manual `rust_decimal` Usage

Without `money`, you'd track currency manually:

```rust
use rust_decimal::Decimal;

let amount = Decimal::from(1999); // cents
let currency = "USD";
// ... hope you don't mix up currencies
```

With `money`, you get:

- `CurrencyAmount` bundles amount + currency
- Arithmetic checks currency compatibility
- Formatting handles symbols and decimal places
- No "stringly-typed" currency codes in your business logic

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
