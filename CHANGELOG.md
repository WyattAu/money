# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

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
