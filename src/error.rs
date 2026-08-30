/// Errors that can occur in money operations.
#[derive(Debug, thiserror::Error)]
pub enum MoneyError {
    /// Attempted to operate on amounts with different currencies.
    #[error("Currency mismatch: cannot operate on {left} and {right}")]
    CurrencyMismatch { left: String, right: String },

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
