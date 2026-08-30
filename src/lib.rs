#![forbid(unsafe_code)]
//! Money types for Rust.
//!
//! `money` provides `CurrencyAmount` with `Decimal` precision, multi-currency
//! support, FX conversion, and configurable formatting.
//!
//! # Quick Start
//!
//! ```rust
//! use money::{CurrencyAmount, Currency};
//!
//! let price = CurrencyAmount::new(rust_decimal_macros::dec!(19.99), Currency::USD);
//! let tax = CurrencyAmount::new(rust_decimal_macros::dec!(1.60), Currency::USD);
//! let total = (price + tax).unwrap();
//! assert_eq!(total.amount, rust_decimal_macros::dec!(21.59));
//! ```

pub mod amount;
pub mod currency;
pub mod error;
pub mod format;
pub mod fx;

pub use amount::CurrencyAmount;
pub use currency::Currency;
pub use error::{MoneyError, Result};
pub use format::{FormatConfig, SymbolPosition};
pub use fx::{convert, FxProvider, FxRate, InMemoryFxProvider};
