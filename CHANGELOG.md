# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

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
