#[cfg(feature = "serde_impl")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a supported currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    CHF,
    CAD,
    AUD,
    CNY,
    INR,
    BRL,
    KRW,
    MXN,
    SEK,
    NOK,
    DKK,
    PLN,
    CZK,
    HUF,
    RUB,
    ZAR,
    SGD,
    HKD,
    NZD,
    THB,
    TRY,
    AED,
    SAR,
    NGN,
    EGP,
    PHP,
    IDR,
    MYR,
    VND,
    PKR,
    BDT,
    BTC,
    ETH,
}

impl Currency {
    /// Returns the ISO 4217 currency code.
    pub fn code(&self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::CHF => "CHF",
            Currency::CAD => "CAD",
            Currency::AUD => "AUD",
            Currency::CNY => "CNY",
            Currency::INR => "INR",
            Currency::BRL => "BRL",
            Currency::KRW => "KRW",
            Currency::MXN => "MXN",
            Currency::SEK => "SEK",
            Currency::NOK => "NOK",
            Currency::DKK => "DKK",
            Currency::PLN => "PLN",
            Currency::CZK => "CZK",
            Currency::HUF => "HUF",
            Currency::RUB => "RUB",
            Currency::ZAR => "ZAR",
            Currency::SGD => "SGD",
            Currency::HKD => "HKD",
            Currency::NZD => "NZD",
            Currency::THB => "THB",
            Currency::TRY => "TRY",
            Currency::AED => "AED",
            Currency::SAR => "SAR",
            Currency::NGN => "NGN",
            Currency::EGP => "EGP",
            Currency::PHP => "PHP",
            Currency::IDR => "IDR",
            Currency::MYR => "MYR",
            Currency::VND => "VND",
            Currency::PKR => "PKR",
            Currency::BDT => "BDT",
            Currency::BTC => "BTC",
            Currency::ETH => "ETH",
        }
    }

    /// Returns the display symbol for the currency.
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::USD => "$",
            Currency::EUR => "\u{20AC}",
            Currency::GBP => "\u{00A3}",
            Currency::JPY => "\u{00A5}",
            Currency::CHF => "CHF",
            Currency::CAD => "C$",
            Currency::AUD => "A$",
            Currency::CNY => "\u{00A5}",
            Currency::INR => "\u{20B9}",
            Currency::BRL => "R$",
            Currency::KRW => "\u{20A9}",
            Currency::MXN => "MX$",
            Currency::SEK => "kr",
            Currency::NOK => "kr",
            Currency::DKK => "kr",
            Currency::PLN => "z\u{0142}",
            Currency::CZK => "K\u{010D}",
            Currency::HUF => "Ft",
            Currency::RUB => "\u{20BD}",
            Currency::ZAR => "R",
            Currency::SGD => "S$",
            Currency::HKD => "HK$",
            Currency::NZD => "NZ$",
            Currency::THB => "\u{0E3F}",
            Currency::TRY => "\u{20BA}",
            Currency::AED => "AED",
            Currency::SAR => "SAR",
            Currency::NGN => "\u{20A6}",
            Currency::EGP => "E\u{00A3}",
            Currency::PHP => "\u{20B1}",
            Currency::IDR => "Rp",
            Currency::MYR => "RM",
            Currency::VND => "\u{20AB}",
            Currency::PKR => "\u{20A8}",
            Currency::BDT => "\u{09F3}",
            Currency::BTC => "\u{20BF}",
            Currency::ETH => "\u{039E}",
        }
    }

    /// Returns the number of decimal places for the currency.
    pub fn decimal_places(&self) -> u32 {
        match self {
            Currency::JPY | Currency::KRW => 0,
            Currency::BTC => 8,
            Currency::ETH => 18,
            _ => 2,
        }
    }

    /// Returns true if this is a cryptocurrency.
    pub fn is_crypto(&self) -> bool {
        matches!(self, Currency::BTC | Currency::ETH)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl std::str::FromStr for Currency {
    type Err = crate::error::MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            "CHF" => Ok(Currency::CHF),
            "CAD" => Ok(Currency::CAD),
            "AUD" => Ok(Currency::AUD),
            "CNY" => Ok(Currency::CNY),
            "INR" => Ok(Currency::INR),
            "BRL" => Ok(Currency::BRL),
            "KRW" => Ok(Currency::KRW),
            "MXN" => Ok(Currency::MXN),
            "SEK" => Ok(Currency::SEK),
            "NOK" => Ok(Currency::NOK),
            "DKK" => Ok(Currency::DKK),
            "PLN" => Ok(Currency::PLN),
            "CZK" => Ok(Currency::CZK),
            "HUF" => Ok(Currency::HUF),
            "RUB" => Ok(Currency::RUB),
            "ZAR" => Ok(Currency::ZAR),
            "SGD" => Ok(Currency::SGD),
            "HKD" => Ok(Currency::HKD),
            "NZD" => Ok(Currency::NZD),
            "THB" => Ok(Currency::THB),
            "TRY" => Ok(Currency::TRY),
            "AED" => Ok(Currency::AED),
            "SAR" => Ok(Currency::SAR),
            "NGN" => Ok(Currency::NGN),
            "EGP" => Ok(Currency::EGP),
            "PHP" => Ok(Currency::PHP),
            "IDR" => Ok(Currency::IDR),
            "MYR" => Ok(Currency::MYR),
            "VND" => Ok(Currency::VND),
            "PKR" => Ok(Currency::PKR),
            "BDT" => Ok(Currency::BDT),
            "BTC" => Ok(Currency::BTC),
            "ETH" => Ok(Currency::ETH),
            _ => Err(crate::error::MoneyError::InvalidAmount(format!(
                "Unknown currency: {s}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_codes() {
        assert_eq!(Currency::USD.code(), "USD");
        assert_eq!(Currency::BTC.code(), "BTC");
    }

    #[test]
    fn test_currency_symbols() {
        assert_eq!(Currency::USD.symbol(), "$");
        assert_eq!(Currency::EUR.symbol(), "\u{20AC}");
    }

    #[test]
    fn test_decimal_places() {
        assert_eq!(Currency::USD.decimal_places(), 2);
        assert_eq!(Currency::JPY.decimal_places(), 0);
        assert_eq!(Currency::BTC.decimal_places(), 8);
    }

    #[test]
    fn test_is_crypto() {
        assert!(Currency::BTC.is_crypto());
        assert!(Currency::ETH.is_crypto());
        assert!(!Currency::USD.is_crypto());
    }

    #[test]
    fn test_from_str() {
        assert_eq!("USD".parse::<Currency>().unwrap(), Currency::USD);
        assert_eq!("btc".parse::<Currency>().unwrap(), Currency::BTC);
        assert!("XYZ".parse::<Currency>().is_err());
    }
}
