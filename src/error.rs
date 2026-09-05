/// Errors that can occur in money operations.
#[derive(Debug, thiserror::Error)]
pub enum MoneyError {
    /// Attempted to operate on amounts with different currencies.
    #[error("Currency mismatch: cannot operate on {left} and {right}")]
    CurrencyMismatch {
        /// The left operand's currency code.
        left: String,
        /// The right operand's currency code.
        right: String,
    },

    /// Arithmetic overflow occurred.
    #[error("Overflow during operation")]
    Overflow,

    /// An invalid amount was provided.
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    /// Rounding error occurred.
    #[error("Rounding error: {0}")]
    Rounding(String),

    /// Serialization or deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// A convenience result type for money operations.
pub type Result<T> = std::result::Result<T, MoneyError>;

impl From<rust_decimal::Error> for MoneyError {
    fn from(e: rust_decimal::Error) -> Self {
        MoneyError::InvalidAmount(e.to_string())
    }
}

// Tests exercise Display formatting and conversions directly; unwrap/expect
// is the test signal here.
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn display_messages_are_stable() {
        let e = MoneyError::CurrencyMismatch {
            left: "USD".to_string(),
            right: "EUR".to_string(),
        };
        assert_eq!(
            e.to_string(),
            "Currency mismatch: cannot operate on USD and EUR"
        );

        assert_eq!(MoneyError::Overflow.to_string(), "Overflow during operation");
        assert_eq!(
            MoneyError::InvalidAmount("negative rate".to_string()).to_string(),
            "Invalid amount: negative rate"
        );
        assert_eq!(
            MoneyError::Rounding("half-even".to_string()).to_string(),
            "Rounding error: half-even"
        );
        assert_eq!(
            MoneyError::Serialization("bad json".to_string()).to_string(),
            "Serialization error: bad json"
        );
    }

    #[test]
    fn from_rust_decimal_error_wraps_as_invalid_amount() {
        let err = rust_decimal::Decimal::from_str("not-a-number").unwrap_err();
        let wrapped = MoneyError::from(err);
        assert!(matches!(wrapped, MoneyError::InvalidAmount(_)));
        assert!(!wrapped.to_string().is_empty());
    }
}
