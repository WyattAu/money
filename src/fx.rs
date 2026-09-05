use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::amount::CurrencyAmount;
use crate::currency::Currency;
use crate::error::{MoneyError, Result};

/// An exchange rate between two currencies.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct FxRate {
    /// Source currency.
    pub from: Currency,
    /// Target currency.
    pub to: Currency,
    /// Exchange rate.
    pub rate: Decimal,
}

impl FxRate {
    /// Creates a new `FxRate`.
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::InvalidAmount`] if `rate` is zero or negative.
    pub fn new(from: Currency, to: Currency, rate: Decimal) -> Result<Self> {
        if rate <= Decimal::ZERO {
            return Err(MoneyError::InvalidAmount(
                "exchange rate must be positive".into(),
            ));
        }
        Ok(Self { from, to, rate })
    }

    /// Creates a rate that represents identity (1:1).
    pub fn identity(currency: Currency) -> Self {
        Self {
            from: currency,
            to: currency,
            rate: Decimal::from(1),
        }
    }

    /// Returns the inverse rate.
    ///
    /// # Panics
    ///
    /// Panics if the rate is zero (should be impossible after `new()` validation).
    pub fn inverse(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            rate: Decimal::from(1) / self.rate,
        }
    }
}

/// A trait for providing foreign exchange rates.
pub trait FxProvider {
    /// Returns the exchange rate between two currencies, if available.
    fn get_rate(&self, from: Currency, to: Currency) -> Result<FxRate>;

    /// Returns all available rates for a given currency.
    fn get_rates_from(&self, from: Currency) -> Result<Vec<FxRate>>;
}

/// A simple in-memory FX rate provider.
pub struct InMemoryFxProvider {
    rates: HashMap<(Currency, Currency), Decimal>,
}

impl InMemoryFxProvider {
    /// Creates a new empty provider.
    pub fn new() -> Self {
        Self {
            rates: HashMap::new(),
        }
    }

    /// Adds or updates an exchange rate.
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::InvalidAmount`] if `rate` is zero or negative.
    pub fn set_rate(&mut self, from: Currency, to: Currency, rate: Decimal) -> Result<()> {
        if rate <= Decimal::ZERO {
            return Err(MoneyError::InvalidAmount(
                "exchange rate must be positive".into(),
            ));
        }
        self.rates.insert((from, to), rate);
        Ok(())
    }

    /// Loads rates from a slice of `FxRate` structs.
    pub fn load_rates(&mut self, rates: &[FxRate]) {
        for rate in rates {
            self.rates.insert((rate.from, rate.to), rate.rate);
        }
    }
}

impl Default for InMemoryFxProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FxProvider for InMemoryFxProvider {
    fn get_rate(&self, from: Currency, to: Currency) -> Result<FxRate> {
        if from == to {
            return Ok(FxRate::identity(from));
        }
        let rate = self
            .rates
            .get(&(from, to))
            .ok_or_else(|| MoneyError::InvalidAmount(format!("No rate for {from} -> {to}")))?;
        FxRate::new(from, to, *rate)
    }

    fn get_rates_from(&self, from: Currency) -> Result<Vec<FxRate>> {
        self.rates
            .iter()
            .filter(|((f, _), _)| *f == from)
            .map(|((f, t), r)| FxRate::new(*f, *t, *r))
            .collect()
    }
}

/// Converts a `CurrencyAmount` using an `FxProvider`.
pub fn convert(
    amount: &CurrencyAmount,
    to: Currency,
    provider: &impl FxProvider,
) -> Result<CurrencyAmount> {
    let rate = provider.get_rate(amount.currency, to)?;
    Ok(CurrencyAmount::new(amount.amount * rate.rate, to))
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
    fn test_fx_rate_inverse() {
        let rate = FxRate::new(
            Currency::USD,
            Currency::EUR,
            Decimal::try_from("0.85").unwrap(),
        )
        .unwrap();
        let inv = rate.inverse();
        assert_eq!(inv.from, Currency::EUR);
        assert_eq!(inv.to, Currency::USD);
    }

    #[test]
    fn test_fx_rate_rejects_zero() {
        assert!(FxRate::new(Currency::USD, Currency::EUR, Decimal::ZERO).is_err());
    }

    #[test]
    fn test_fx_rate_rejects_negative() {
        assert!(
            FxRate::new(
                Currency::USD,
                Currency::EUR,
                Decimal::try_from("-0.85").unwrap()
            )
            .is_err()
        );
    }

    #[test]
    fn test_in_memory_provider() {
        let mut provider = InMemoryFxProvider::new();
        provider
            .set_rate(
                Currency::USD,
                Currency::EUR,
                Decimal::try_from("0.85").unwrap(),
            )
            .unwrap();

        let rate = provider.get_rate(Currency::USD, Currency::EUR).unwrap();
        assert_eq!(rate.rate, Decimal::try_from("0.85").unwrap());

        let identity = provider.get_rate(Currency::USD, Currency::USD).unwrap();
        assert_eq!(identity.rate, Decimal::from(1));
    }
}
