//! Rounding policies for monetary amounts.
//!
//! Different financial domains require different tie-breaking rules when a
//! value falls exactly halfway between two representable amounts (e.g.
//! `1.25` rounded to one decimal place). [`RoundingPolicy`] selects the
//! strategy; it is applied via
//! [`CurrencyAmount::round_to_with_policy`](crate::CurrencyAmount::round_to_with_policy).

use rust_decimal::RoundingStrategy;

/// A tie-breaking rule applied when rounding a monetary amount.
///
/// All policies agree on values that are *not* exactly halfway between two
/// candidates; they differ only on the exact midpoint (e.g. `2.5` to zero
/// decimal places).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum RoundingPolicy {
    /// Round halfway values away from zero.
    ///
    /// `2.5 -> 3`, `-2.5 -> -3`. This is the convention most people learn
    /// in school and is common in retail pricing.
    HalfUp,

    /// Round halfway values to the nearest even neighbor ("banker's rounding").
    ///
    /// `2.5 -> 2`, `3.5 -> 4`. This is the default for
    /// [`CurrencyAmount::round`](crate::CurrencyAmount::round) and
    /// [`CurrencyAmount::round_to`](crate::CurrencyAmount::round_to); over
    /// many roundings it avoids the upward bias that `HalfUp` introduces.
    HalfEven,

    /// Round halfway values toward zero.
    ///
    /// `2.5 -> 2`, `-2.5 -> -2`.
    HalfDown,

    /// Round toward negative infinity (floor).
    ///
    /// `2.9 -> 2`, `-2.1 -> -3`. Useful when the merchant must always
    /// absorb the difference.
    Floor,

    /// Round toward positive infinity (ceiling).
    ///
    /// `2.1 -> 3`, `-2.9 -> -2`. Useful when the customer must always
    /// absorb the difference.
    Ceiling,
}

impl RoundingPolicy {
    /// Maps the policy onto the underlying `rust_decimal` strategy.
    #[must_use]
    pub fn to_strategy(self) -> RoundingStrategy {
        match self {
            RoundingPolicy::HalfUp => RoundingStrategy::MidpointAwayFromZero,
            RoundingPolicy::HalfEven => RoundingStrategy::MidpointNearestEven,
            RoundingPolicy::HalfDown => RoundingStrategy::MidpointTowardZero,
            RoundingPolicy::Floor => RoundingStrategy::ToNegativeInfinity,
            RoundingPolicy::Ceiling => RoundingStrategy::ToPositiveInfinity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policies_map_to_expected_strategies() {
        assert_eq!(
            RoundingPolicy::HalfUp.to_strategy(),
            RoundingStrategy::MidpointAwayFromZero
        );
        assert_eq!(
            RoundingPolicy::HalfEven.to_strategy(),
            RoundingStrategy::MidpointNearestEven
        );
        assert_eq!(
            RoundingPolicy::HalfDown.to_strategy(),
            RoundingStrategy::MidpointTowardZero
        );
        assert_eq!(
            RoundingPolicy::Floor.to_strategy(),
            RoundingStrategy::ToNegativeInfinity
        );
        assert_eq!(
            RoundingPolicy::Ceiling.to_strategy(),
            RoundingStrategy::ToPositiveInfinity
        );
    }
}
