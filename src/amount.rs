use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::ops::{Add, Sub};
use std::str::FromStr;

use crate::currency::Currency;
use crate::error::{MoneyError, Result};
use crate::rounding::RoundingPolicy;

/// Greatest common divisor of two non-negative magnitudes.
fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.unsigned_abs(), b.unsigned_abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a as i128
}
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

    /// Rounds the amount to the currency's decimal places using banker's
    /// rounding (half to even).
    pub fn round(&self) -> Self {
        let places = self.currency.decimal_places();
        Self {
            amount: self.amount.round_dp(places),
            currency: self.currency,
        }
    }

    /// Rounds the amount to the specified number of decimal places using
    /// banker's rounding (half to even).
    ///
    /// For a different tie-breaking rule, use
    /// [`round_to_with_policy`](Self::round_to_with_policy).
    pub fn round_to(&self, decimal_places: u32) -> Self {
        Self {
            amount: self.amount.round_dp(decimal_places),
            currency: self.currency,
        }
    }

    /// Rounds the amount to the specified number of decimal places using
    /// the given [`RoundingPolicy`].
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_money::{CurrencyAmount, Currency, RoundingPolicy};
    ///
    /// let amount = CurrencyAmount::from_str_values("2.5", Currency::USD).unwrap();
    ///
    /// let up = amount.round_to_with_policy(0, RoundingPolicy::HalfUp);
    /// assert_eq!(up.amount.to_string(), "3");
    ///
    /// let even = amount.round_to_with_policy(0, RoundingPolicy::HalfEven);
    /// assert_eq!(even.amount.to_string(), "2");
    ///
    /// let floor = amount.round_to_with_policy(0, RoundingPolicy::Floor);
    /// assert_eq!(floor.amount.to_string(), "2");
    /// ```
    pub fn round_to_with_policy(&self, decimal_places: u32, policy: RoundingPolicy) -> Self {
        Self {
            amount: self
                .amount
                .round_dp_with_strategy(decimal_places, policy.to_strategy()),
            currency: self.currency,
        }
    }

    /// Splits the amount into parts proportional to `ratios`, such that the
    /// parts sum **exactly** to the original amount (no lost or invented
    /// minor units).
    ///
    /// The `ratios` are relative weights and do not need to sum to 1
    /// (e.g. `[1, 1, 1]` splits evenly three ways). Allocation uses the
    /// largest-remainder method: each part first receives its floored exact
    /// share, then any remaining minor units are distributed to the parts
    /// with the largest fractional leftovers (ties broken by ratio order).
    /// Every part keeps the original amount's precision and currency.
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::InvalidAmount`] if `ratios` is empty, if any
    /// ratio is negative, or if the ratios sum to zero. Returns
    /// [`MoneyError::Overflow`] if an intermediate product exceeds the
    /// internal 128-bit working range (extremely large amounts combined
    /// with extremely precise ratios).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_money::{CurrencyAmount, Currency};
    /// use rust_decimal_macros::dec;
    ///
    /// let total = CurrencyAmount::from_str_values("100.00", Currency::USD).unwrap();
    /// let parts = total.allocate(&[dec!(1), dec!(1), dec!(1)]).unwrap();
    /// assert_eq!(parts[0].amount.to_string(), "33.34");
    /// assert_eq!(parts[1].amount.to_string(), "33.33");
    /// assert_eq!(parts[2].amount.to_string(), "33.33");
    ///
    /// let sum: rust_decimal::Decimal = parts.iter().map(|p| p.amount).sum();
    /// assert_eq!(sum, total.amount);
    /// ```
    pub fn allocate(
        &self,
        ratios: &[Decimal],
    ) -> std::result::Result<Vec<CurrencyAmount>, MoneyError> {
        if ratios.is_empty() {
            return Err(MoneyError::InvalidAmount(
                "allocation requires at least one ratio".to_string(),
            ));
        }

        // Express every ratio as an integer weight at the highest scale
        // present, so `0.5` and `0.50` weigh identically and all splitting
        // happens in exact integer arithmetic.
        let weight_scale = ratios.iter().map(Decimal::scale).max().unwrap_or(0);
        let mut weights: Vec<i128> = Vec::with_capacity(ratios.len());
        for ratio in ratios {
            if ratio.is_sign_negative() {
                return Err(MoneyError::InvalidAmount(format!(
                    "allocation ratios must not be negative: {ratio}"
                )));
            }
            let factor = 10i128
                .checked_pow(weight_scale - ratio.scale())
                .ok_or(MoneyError::Overflow)?;
            let scaled = ratio
                .mantissa()
                .checked_mul(factor)
                .ok_or(MoneyError::Overflow)?;
            weights.push(scaled);
        }

        let total_weight = weights
            .iter()
            .try_fold(0i128, |acc, &w| acc.checked_add(w))
            .ok_or(MoneyError::Overflow)?;
        if total_weight == 0 {
            return Err(MoneyError::InvalidAmount(
                "allocation ratios must not all be zero".to_string(),
            ));
        }

        // Reduce by the gcd to keep later products small.
        let common = weights.iter().fold(total_weight, |acc, &w| gcd(acc, w));
        if common > 1 {
            for w in &mut weights {
                *w /= common;
            }
        }
        let total_weight = weights
            .iter()
            .try_fold(0i128, |acc, &w| acc.checked_add(w))
            .ok_or(MoneyError::Overflow)?;

        // Work in exact minor units of the original amount so the parts can
        // be reassembled bit-for-bit: mantissa * 10^-scale == amount.
        let units = self.amount.mantissa();
        let unit_scale = self.amount.scale();

        // Exact share per part: units * w_i / total_weight. Keep the floored
        // quotient and the (always non-negative) remainder.
        let mut floors: Vec<i128> = Vec::with_capacity(weights.len());
        let mut remainders: Vec<i128> = Vec::with_capacity(weights.len());
        let mut allocated = 0i128;
        for &w in &weights {
            let raw = units.checked_mul(w).ok_or(MoneyError::Overflow)?;
            let floored = raw.div_euclid(total_weight);
            remainders.push(raw.rem_euclid(total_weight));
            floors.push(floored);
            allocated = allocated.checked_add(floored).ok_or(MoneyError::Overflow)?;
        }

        // Distribute the undistributed units to the largest remainders.
        // A stable sort keeps ties in ratio order; leftover is bounded by
        // the number of parts.
        let leftover = units - allocated;
        let mut order: Vec<(usize, i128)> = remainders.iter().copied().enumerate().collect();
        order.sort_by(|a, b| b.1.cmp(&a.1));
        let distribute = usize::try_from(leftover.max(0)).map_err(|_| MoneyError::Overflow)?;
        let mut parts = floors;
        for &(idx, _) in order.iter().take(distribute) {
            // Indices come from enumerate over remainders, so always in bounds.
            let Some(part) = parts.get_mut(idx) else {
                continue;
            };
            *part += 1;
        }
        debug_assert_eq!(
            parts.iter().try_fold(0i128, |acc, &p| acc.checked_add(p)),
            Some(units),
            "largest-remainder allocation must preserve the total"
        );

        Ok(parts
            .into_iter()
            .map(|part| CurrencyAmount {
                amount: Decimal::from_i128_with_scale(part, unit_scale),
                currency: self.currency,
            })
            .collect())
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
    fn test_round_to_with_policy_half_up_and_even() {
        let amount = CurrencyAmount::from_str_values("2.5", Currency::USD).unwrap();
        assert_eq!(
            amount
                .round_to_with_policy(0, crate::rounding::RoundingPolicy::HalfUp)
                .amount,
            Decimal::from(3)
        );
        assert_eq!(
            amount
                .round_to_with_policy(0, crate::rounding::RoundingPolicy::HalfEven)
                .amount,
            Decimal::from(2)
        );
        assert_eq!(
            CurrencyAmount::from_str_values("3.5", Currency::USD)
                .unwrap()
                .round_to_with_policy(0, crate::rounding::RoundingPolicy::HalfEven)
                .amount,
            Decimal::from(4)
        );
    }

    #[test]
    fn test_round_to_with_policy_half_down_floor_ceiling() {
        let pos = CurrencyAmount::from_str_values("2.5", Currency::USD).unwrap();
        let neg = CurrencyAmount::from_str_values("-2.5", Currency::USD).unwrap();
        let pos_frac = CurrencyAmount::from_str_values("2.1", Currency::USD).unwrap();
        let neg_frac = CurrencyAmount::from_str_values("-2.9", Currency::USD).unwrap();

        use crate::rounding::RoundingPolicy as P;
        assert_eq!(
            pos.round_to_with_policy(0, P::HalfDown).amount,
            Decimal::from(2)
        );
        assert_eq!(
            neg.round_to_with_policy(0, P::HalfDown).amount,
            Decimal::from(-2)
        );
        assert_eq!(
            pos_frac.round_to_with_policy(0, P::Floor).amount,
            Decimal::from(2)
        );
        assert_eq!(
            neg_frac.round_to_with_policy(0, P::Floor).amount,
            Decimal::from(-3)
        );
        assert_eq!(
            pos_frac.round_to_with_policy(0, P::Ceiling).amount,
            Decimal::from(3)
        );
        assert_eq!(
            neg_frac.round_to_with_policy(0, P::Ceiling).amount,
            Decimal::from(-2)
        );
    }

    #[test]
    fn test_round_to_with_policy_negative_half_up_away_from_zero() {
        let neg = CurrencyAmount::from_str_values("-2.5", Currency::USD).unwrap();
        assert_eq!(
            neg.round_to_with_policy(0, crate::rounding::RoundingPolicy::HalfUp)
                .amount,
            Decimal::from(-3)
        );
    }

    #[test]
    fn test_allocate_even_split_largest_remainder() {
        let total = CurrencyAmount::from_str_values("100.00", Currency::USD).unwrap();
        let parts = total
            .allocate(&[
                Decimal::try_from("1").unwrap(),
                Decimal::try_from("1").unwrap(),
                Decimal::try_from("1").unwrap(),
            ])
            .unwrap();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].amount, Decimal::try_from("33.34").unwrap());
        assert_eq!(parts[1].amount, Decimal::try_from("33.33").unwrap());
        assert_eq!(parts[2].amount, Decimal::try_from("33.33").unwrap());
        let sum: Decimal = parts.iter().map(|p| p.amount).sum();
        assert_eq!(sum, total.amount);
        for part in &parts {
            assert_eq!(part.currency, Currency::USD);
        }
    }

    #[test]
    fn test_allocate_ratio_weights_not_unity() {
        // Weights 1:2 -> 10.00 becomes 3.33 and 6.67.
        let total = CurrencyAmount::from_str_values("10.00", Currency::USD).unwrap();
        let parts = total.allocate(&[Decimal::ONE, Decimal::from(2)]).unwrap();
        assert_eq!(parts[0].amount, Decimal::try_from("3.33").unwrap());
        assert_eq!(parts[1].amount, Decimal::try_from("6.67").unwrap());
    }

    #[test]
    fn test_allocate_single_ratio_is_identity() {
        let total = CurrencyAmount::from_str_values("19.99", Currency::USD).unwrap();
        let parts = total
            .allocate(&[Decimal::try_from("0.123").unwrap()])
            .unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].amount, total.amount);
        assert_eq!(parts[0].currency, Currency::USD);
    }

    #[test]
    fn test_allocate_preserves_precision_of_original() {
        // Amount more precise than the currency exponent still splits exactly.
        let total = CurrencyAmount::from_str_values("0.100", Currency::USD).unwrap();
        let parts = total
            .allocate(&[Decimal::ONE, Decimal::ONE, Decimal::ONE])
            .unwrap();
        let sum: Decimal = parts.iter().map(|p| p.amount).sum();
        assert_eq!(sum, Decimal::try_from("0.100").unwrap());
        assert_eq!(parts[0].amount, Decimal::try_from("0.034").unwrap());
        assert_eq!(parts[1].amount, Decimal::try_from("0.033").unwrap());
        assert_eq!(parts[2].amount, Decimal::try_from("0.033").unwrap());
    }

    #[test]
    fn test_allocate_zero_decimal_currency() {
        let total = CurrencyAmount::new(Decimal::from(100), Currency::JPY);
        let parts = total
            .allocate(&[Decimal::ONE, Decimal::ONE, Decimal::ONE])
            .unwrap();
        assert_eq!(parts[0].amount, Decimal::from(34));
        assert_eq!(parts[1].amount, Decimal::from(33));
        assert_eq!(parts[2].amount, Decimal::from(33));
    }

    #[test]
    fn test_allocate_negative_amount_sums_exactly() {
        let total = CurrencyAmount::from_str_values("-100.00", Currency::USD).unwrap();
        let parts = total
            .allocate(&[Decimal::ONE, Decimal::ONE, Decimal::ONE])
            .unwrap();
        let sum: Decimal = parts.iter().map(|p| p.amount).sum();
        assert_eq!(sum, total.amount);
        assert_eq!(parts[0].amount, Decimal::try_from("-33.33").unwrap());
        assert_eq!(parts[1].amount, Decimal::try_from("-33.33").unwrap());
        assert_eq!(parts[2].amount, Decimal::try_from("-33.34").unwrap());
    }

    #[test]
    fn test_allocate_zero_amount() {
        let total = CurrencyAmount::new(Decimal::ZERO, Currency::USD);
        let parts = total.allocate(&[Decimal::ONE, Decimal::from(2)]).unwrap();
        assert!(parts.iter().all(|p| p.amount.is_zero()));
    }

    #[test]
    fn test_allocate_tie_breaks_by_ratio_order() {
        // 0.02 three ways: two parts get 0.01 (first two win ties).
        let total = CurrencyAmount::from_str_values("0.02", Currency::USD).unwrap();
        let parts = total
            .allocate(&[Decimal::ONE, Decimal::ONE, Decimal::ONE])
            .unwrap();
        assert_eq!(parts[0].amount, Decimal::try_from("0.01").unwrap());
        assert_eq!(parts[1].amount, Decimal::try_from("0.01").unwrap());
        assert_eq!(parts[2].amount, Decimal::try_from("0.00").unwrap());
    }

    #[test]
    fn test_allocate_zero_ratio_is_allowed() {
        let total = CurrencyAmount::from_str_values("10.00", Currency::USD).unwrap();
        let parts = total
            .allocate(&[Decimal::ZERO, Decimal::ONE, Decimal::ONE])
            .unwrap();
        assert!(parts[0].amount.is_zero());
        assert_eq!(parts[1].amount, Decimal::try_from("5.00").unwrap());
        assert_eq!(parts[2].amount, Decimal::try_from("5.00").unwrap());
    }

    #[test]
    fn test_allocate_rejects_empty() {
        let total = CurrencyAmount::from_str_values("10.00", Currency::USD).unwrap();
        let err = total.allocate(&[]).unwrap_err();
        assert!(matches!(err, MoneyError::InvalidAmount(_)));
    }

    #[test]
    fn test_allocate_rejects_negative_ratio() {
        let total = CurrencyAmount::from_str_values("10.00", Currency::USD).unwrap();
        let err = total
            .allocate(&[Decimal::try_from("-0.5").unwrap(), Decimal::ONE])
            .unwrap_err();
        assert!(matches!(err, MoneyError::InvalidAmount(_)));
    }

    #[test]
    fn test_allocate_rejects_zero_sum_ratios() {
        let total = CurrencyAmount::from_str_values("10.00", Currency::USD).unwrap();
        let err = total.allocate(&[Decimal::ZERO, Decimal::ZERO]).unwrap_err();
        assert!(matches!(err, MoneyError::InvalidAmount(_)));
    }

    #[test]
    fn test_allocate_scale_independent_weights() {
        // 0.5 and 0.50 must weigh the same.
        let total = CurrencyAmount::from_str_values("1.00", Currency::USD).unwrap();
        let a = total
            .allocate(&[
                Decimal::try_from("0.5").unwrap(),
                Decimal::try_from("0.50").unwrap(),
            ])
            .unwrap();
        assert_eq!(a[0].amount, Decimal::try_from("0.50").unwrap());
        assert_eq!(a[1].amount, Decimal::try_from("0.50").unwrap());
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
