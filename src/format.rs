use rust_decimal::Decimal;
use std::fmt;

use crate::amount::CurrencyAmount;
use crate::currency::Currency;

/// Position of the currency symbol relative to the amount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolPosition {
    /// Before the amount: `$19.99`
    Prefix,
    /// After the amount: `19.99$`
    Suffix,
}

/// Configuration for formatting monetary amounts.
#[derive(Debug, Clone)]
pub struct FormatConfig {
    /// Position of the currency symbol.
    pub symbol_position: SymbolPosition,
    /// Character used as thousands separator.
    pub thousands_separator: char,
    /// Character used as decimal separator.
    pub decimal_separator: char,
    /// Whether to include the currency symbol.
    pub show_symbol: bool,
    /// Whether to use ISO code instead of symbol.
    pub use_iso_code: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            symbol_position: SymbolPosition::Prefix,
            thousands_separator: ',',
            decimal_separator: '.',
            show_symbol: true,
            use_iso_code: false,
        }
    }
}

impl FormatConfig {
    /// US-style formatting: `$1,234.56`
    pub fn us() -> Self {
        Self::default()
    }

    /// European-style formatting: `1.234,56 €`
    pub fn european() -> Self {
        Self {
            symbol_position: SymbolPosition::Suffix,
            thousands_separator: '.',
            decimal_separator: ',',
            show_symbol: true,
            use_iso_code: false,
        }
    }

    /// ISO code formatting: `USD 1,234.56`
    pub fn iso() -> Self {
        Self {
            show_symbol: false,
            use_iso_code: true,
            ..Default::default()
        }
    }

    /// Formats a `Decimal` value according to this configuration.
    pub fn format_decimal(&self, value: &Decimal, currency: Currency) -> String {
        let (symbol_str, separator) = if self.use_iso_code {
            (currency.code(), self.thousands_separator)
        } else if self.show_symbol {
            (currency.symbol(), self.thousands_separator)
        } else {
            ("", self.thousands_separator)
        };

        let places = currency.decimal_places() as usize;
        let abs_val = value.abs();
        let sign = if *value < Decimal::ZERO { "-" } else { "" };

        let whole = abs_val.to_string();
        let parts: Vec<&str> = whole.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = if parts.len() > 1 { parts[1] } else { "" };

        // Add thousands separators
        let mut formatted_int = String::new();
        for (i, c) in integer_part.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                formatted_int.push(separator);
            }
            formatted_int.push(c);
        }
        let formatted_int: String = formatted_int.chars().rev().collect();

        // Pad or truncate decimal part
        let formatted_dec = if decimal_part.is_empty() {
            "0".repeat(places)
        } else if decimal_part.len() > places {
            decimal_part[..places].to_string()
        } else {
            format!("{decimal_part}{:0>width$}", "", width = places - decimal_part.len())
        };

        let amount_str = if places > 0 {
            format!("{formatted_int}{}{formatted_dec}", self.decimal_separator)
        } else {
            formatted_int
        };

        match self.symbol_position {
            SymbolPosition::Prefix => {
                if symbol_str.is_empty() {
                    format!("{sign}{amount_str}")
                } else {
                    format!("{sign}{symbol_str}{amount_str}")
                }
            }
            SymbolPosition::Suffix => {
                if symbol_str.is_empty() {
                    format!("{sign}{amount_str}")
                } else {
                    format!("{sign}{amount_str} {symbol_str}")
                }
            }
        }
    }

    /// Formats a `CurrencyAmount` according to this configuration.
    pub fn format(&self, amount: &CurrencyAmount) -> String {
        self.format_decimal(&amount.amount, amount.currency)
    }
}

impl fmt::Display for FormatConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormatConfig({}, {}, {})",
            match self.symbol_position {
                SymbolPosition::Prefix => "prefix",
                SymbolPosition::Suffix => "suffix",
            },
            self.thousands_separator,
            self.decimal_separator
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_us_format() {
        let config = FormatConfig::us();
        let amount = CurrencyAmount::new(Decimal::try_from("1234.56").unwrap(), Currency::USD);
        assert_eq!(config.format(&amount), "$1,234.56");
    }

    #[test]
    fn test_european_format() {
        let config = FormatConfig::european();
        let amount = CurrencyAmount::new(Decimal::try_from("1234.56").unwrap(), Currency::EUR);
        assert_eq!(config.format(&amount), "1.234,56 \u{20AC}");
    }

    #[test]
    fn test_negative_amount() {
        let config = FormatConfig::us();
        let amount = CurrencyAmount::new(Decimal::try_from("-99.5").unwrap(), Currency::USD);
        assert_eq!(config.format(&amount), "-$99.50");
    }

    #[test]
    fn test_zero_decimal_places() {
        let config = FormatConfig::us();
        let amount = CurrencyAmount::new(Decimal::from(1234), Currency::JPY);
        assert_eq!(config.format(&amount), "\u{00A5}1,234");
    }
}
