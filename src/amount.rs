use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::ops::{Add, Sub};
use std::str::FromStr;

use crate::currency::Currency;
use crate::error::{MoneyError, Result};

/// A monetary amount with a currency.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CurrencyAmount {
    /// The monetary amount.
    pub amount: Decimal,
    /// The currency.
    pub currency: Currency,
}

impl CurrencyAmount {
    /// Creates a new `CurrencyAmount` from an integer value.
    pub fn new(amount: impl Into<Decimal>, currency: Currency) -> Self {
        Self {
            amount: amount.into(),
            currency,
        }
    }

    /// Creates a new `CurrencyAmount` from a string representation.
    ///
    /// # Arguments
    /// * `amount` - A string like `"19.99"`
    /// * `currency` - The currency code like `"USD"`
    pub fn from_str_values(amount: &str, currency: Currency) -> Result<Self> {
        let decimal = Decimal::from_str(amount)
            .map_err(|e| MoneyError::InvalidAmount(format!("Invalid decimal '{amount}': {e}")))?;
        Ok(Self::new(decimal, currency))
    }

    /// Returns true if the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    /// Returns true if the amount is positive.
    pub fn is_positive(&self) -> bool {
        self.amount.is_sign_positive() && !self.amount.is_zero()
    }

    /// Returns true if the amount is negative.
    pub fn is_negative(&self) -> bool {
        self.amount.is_sign_negative() && !self.amount.is_zero()
    }

    /// Returns the absolute value of the amount.
    pub fn abs(&self) -> Self {
        Self {
            amount: self.amount.abs(),
            currency: self.currency,
        }
    }

    /// Negates the amount.
    pub fn negate(&self) -> Self {
        Self {
            amount: -self.amount,
            currency: self.currency,
        }
    }

    /// Rounds the amount to the currency's decimal places.
    pub fn round(&self) -> Self {
        let places = self.currency.decimal_places();
        Self {
            amount: self.amount.round_dp(places),
            currency: self.currency,
        }
    }

    /// Rounds the amount to the specified number of decimal places.
    pub fn round_to(&self, decimal_places: u32) -> Self {
        Self {
            amount: self.amount.round_dp(decimal_places),
            currency: self.currency,
        }
    }

    /// Returns the amount as a f64 (may lose precision).
    pub fn to_f64(&self) -> Option<f64> {
        self.amount.to_f64()
    }

    /// Returns the major amount (whole number part) and minor amount (fractional part).
    pub fn parts(&self) -> (Decimal, Decimal) {
        let rounded = self.round();
        let major = rounded.amount.floor();
        let minor = rounded.amount - major;
        (major, minor)
    }
}

impl Add for CurrencyAmount {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.currency != rhs.currency {
            return Err(MoneyError::CurrencyMismatch {
                left: self.currency.code().to_string(),
                right: rhs.currency.code().to_string(),
            });
        }
        let amount = self.amount + rhs.amount;
        Ok(Self {
            amount,
            currency: self.currency,
        })
    }
}

impl Sub for CurrencyAmount {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.currency != rhs.currency {
            return Err(MoneyError::CurrencyMismatch {
                left: self.currency.code().to_string(),
                right: rhs.currency.code().to_string(),
            });
        }
        let amount = self.amount - rhs.amount;
        Ok(Self {
            amount,
            currency: self.currency,
        })
    }
}

impl std::fmt::Display for CurrencyAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = self.currency.symbol();
        let places = self.currency.decimal_places();
        let places = places as usize;
        let formatted = format!("{:.places$}", self.amount);
        write!(f, "{symbol}{formatted}")
    }
}

// Tests exercise failure paths and invariants directly; unwrap/expect,
// slicing, and panicking asserts are acceptable here — violations
// surface as test failures, not production panics.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_amount() {
        let amount = CurrencyAmount::new(Decimal::from(100), Currency::USD);
        assert_eq!(amount.amount, Decimal::from(100));
        assert_eq!(amount.currency, Currency::USD);
    }

    #[test]
    fn test_from_str_values() {
        let amount = CurrencyAmount::from_str_values("19.99", Currency::USD).unwrap();
        assert_eq!(amount.amount, Decimal::try_from("19.99").unwrap());
        assert!(CurrencyAmount::from_str_values("abc", Currency::USD).is_err());
    }

    #[test]
    fn test_predicates() {
        let zero = CurrencyAmount::new(Decimal::ZERO, Currency::USD);
        let positive = CurrencyAmount::new(Decimal::from(5), Currency::USD);
        let negative = CurrencyAmount::new(Decimal::from(-5), Currency::USD);

        assert!(zero.is_zero());
        assert!(!zero.is_positive());
        assert!(!zero.is_negative());
        assert!(positive.is_positive());
        assert!(negative.is_negative());
    }

    #[test]
    fn test_addition() {
        let a = CurrencyAmount::new(Decimal::from(10), Currency::USD);
        let b = CurrencyAmount::new(Decimal::from(20), Currency::USD);
        let result = (a + b).unwrap();
        assert_eq!(result.amount, Decimal::from(30));
    }

    #[test]
    fn test_addition_currency_mismatch() {
        let a = CurrencyAmount::new(Decimal::from(10), Currency::USD);
        let b = CurrencyAmount::new(Decimal::from(20), Currency::EUR);
        assert!(matches!(a + b, Err(MoneyError::CurrencyMismatch { .. })));
    }

    #[test]
    fn test_subtraction() {
        let a = CurrencyAmount::new(Decimal::from(30), Currency::USD);
        let b = CurrencyAmount::new(Decimal::from(10), Currency::USD);
        let result = (a - b).unwrap();
        assert_eq!(result.amount, Decimal::from(20));
    }

    #[test]
    fn test_display() {
        let amount = CurrencyAmount::new(Decimal::try_from("19.99").unwrap(), Currency::USD);
        assert_eq!(format!("{amount}"), "$19.99");
    }

    #[test]
    fn test_round() {
        let amount = CurrencyAmount::new(Decimal::try_from("19.999").unwrap(), Currency::USD);
        let rounded = amount.round();
        assert_eq!(rounded.amount, Decimal::try_from("20.00").unwrap());
    }

    #[test]
    fn test_round_to_explicit_places() {
        let amount = CurrencyAmount::new(Decimal::try_from("19.999").unwrap(), Currency::USD);
        let one_place = amount.round_to(1);
        assert_eq!(one_place.amount, Decimal::try_from("20.0").unwrap());
        assert_eq!(one_place.currency, Currency::USD);

        let zero_places = amount.round_to(0);
        assert_eq!(zero_places.amount, Decimal::from(20));
    }

    #[test]
    fn test_to_f64() {
        let amount = CurrencyAmount::new(Decimal::try_from("19.99").unwrap(), Currency::USD);
        assert_eq!(amount.to_f64(), Some(19.99));
    }

    #[test]
    fn test_parts_splits_major_and_minor() {
        let amount = CurrencyAmount::new(Decimal::try_from("19.99").unwrap(), Currency::USD);
        let (major, minor) = amount.parts();
        assert_eq!(major, Decimal::from(19));
        assert_eq!(minor, Decimal::try_from("0.99").unwrap());

        // Zero-decimal currency: minor part is always zero.
        let yen = CurrencyAmount::new(Decimal::from(1234), Currency::JPY);
        assert_eq!(yen.parts(), (Decimal::from(1234), Decimal::ZERO));
    }

    #[test]
    fn test_subtraction_currency_mismatch() {
        let a = CurrencyAmount::new(Decimal::from(30), Currency::USD);
        let b = CurrencyAmount::new(Decimal::from(10), Currency::GBP);
        assert!(matches!(a - b, Err(MoneyError::CurrencyMismatch { .. })));
    }
}
