#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Money types for Rust.
//!
//! `money` provides `CurrencyAmount` with `Decimal` precision, multi-currency
//! support, FX conversion, and configurable formatting.
//!
//! # Quick Start
//!
//! ```rust
//! use decimal_money::{CurrencyAmount, Currency};
//!
//! let price = CurrencyAmount::new(rust_decimal_macros::dec!(19.99), Currency::USD);
//! let tax = CurrencyAmount::new(rust_decimal_macros::dec!(1.60), Currency::USD);
//! let total = (price + tax).unwrap();
//! assert_eq!(total.amount, rust_decimal_macros::dec!(21.59));
//! ```

/// Currency amount types.
pub mod amount;
/// Currency definitions.
pub mod currency;
/// Error types.
pub mod error;
/// Formatting configuration.
pub mod format;
/// Foreign exchange rate support.
pub mod fx;

pub use amount::CurrencyAmount;
pub use currency::Currency;
pub use error::{MoneyError, Result};
pub use format::{FormatConfig, SymbolPosition};
pub use fx::{convert, FxProvider, FxRate, InMemoryFxProvider};
