# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [1.1.0] - 2026-09-09

### Added

- `CurrencyAmount::allocate(&[Decimal])` — splits an amount into parts
  proportional to the given ratio weights using the largest-remainder
  method, so the parts always sum **exactly** to the original amount (no
  lost cents). Rejects empty, negative, and all-zero ratio sets; works for
  zero-decimal currencies, negative amounts, and amounts more precise than
  the currency's exponent. Extreme amounts combined with extremely precise
  ratios return `MoneyError::Overflow` instead of losing exactness.
- `RoundingPolicy` enum (`HalfUp`, `HalfEven`, `HalfDown`, `Floor`,
  `Ceiling`) and `CurrencyAmount::round_to_with_policy(places, policy)`.
- 19 new ISO 4217 currencies: ILS, TWD, ARS, CLP, COP, PEN, BGN, RON, UAH,
  ISK, MAD, DZD, KES, QAR, KWD, BHD, OMR, JOD, TND (56 total), with correct
  minor-unit exponents: CLP/ISK are 0-decimal and KWD/BHD/OMR/JOD/TND are
  3-decimal.
- Property tests: allocation sum invariance across arbitrary
  amounts/ratios (500 cases/case set), single-ratio identity, per-part
  exact-share proximity, and negative-amount sum invariance.

### Fixed

- `Currency::VND` decimal places corrected from 2 to 0 per the ISO 4217
  minor-unit table (the dong has no circulating minor unit). Display of VND
  amounts no longer appends `.00`. Note: `round_to`/`round` use banker's
  rounding (half to even), matching `rust_decimal`'s default; this is now
  documented.

### Semver notes

- Adding `Currency` variants is a breaking change for downstream exhaustive
  matches; the new variants were appended after all existing variants so
  existing discriminant values are unchanged. Callers matching
  exhaustively must add a wildcard arm when upgrading.

## [1.0.0] - 2026-09-05

First stable release. The public API is now covered by the project's
semver guarantees: breaking changes require a major version bump.

### Fixed

- `FormatConfig::iso()` documented `USD 1,234.56` but rendered `USD1,234.56`
  with no separator. ISO-code prefixes now include a space before the
  amount, matching the documented format.

## [0.2.0] - 2026-09-02

### Added

- `Eq` for `FxRate`; FX rates are validated to be > 0.
- `#[must_use]` annotations on pure constructors and conversions.

### Testing

- cargo-fuzz targets (`fuzz_parse`, `fuzz_arith`).

## [0.1.0] - 2026-09-01

### Added

- `CurrencyAmount` — money type with exact decimal precision
  (`rust_decimal`); arithmetic checks currency compatibility instead of
  panicking.
- 36+ currencies (fiat and crypto: USD, EUR, GBP, JPY, BTC, ETH, ...).
- FX conversion: pluggable `FxProvider` trait with an in-memory
  implementation.
- Formatting: configurable symbol position, separators, and ISO codes.
- Criterion benches.
