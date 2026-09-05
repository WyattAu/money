# Threat Model — decimal-money

Status: **v1.0** · One-page STRIDE over the public API surface
(`CurrencyAmount`, `Currency`, `FxRate`, `InMemoryFxProvider`, `FormatConfig`).

Assets: (A1) monetary value integrity — amounts cannot silently overflow,
round the wrong way, or mix currencies; (A2) FX rate validity — a zero or
negative rate silently destroys value; (A3) display correctness.

| # | Threat | Category | Surface | Mitigation | Verifying test |
|---|--------|----------|---------|------------|----------------|
| T1 | Cross-currency addition (mixing USD + EUR) | Tampering | `Add`/`Sub` impls | Currency mismatch returns `MoneyError`, never a raw number | `tests/proptest.rs::addition_currency_mismatch_fails` |
| T2 | Arithmetic overflow / precision drift | Tampering | amount ops | `rust_decimal` 96-bit fixed-point (no float path in arithmetic); ops are checked | `tests/proptest.rs::addition_commutative`, `subtraction_inverse`, `negation_doubles_to_zero`, `abs_always_non_negative` |
| T3 | Non-positive FX rate accepted | Elevation | `FxRate::new` | `rate <= 0` rejected with `InvalidAmount` | `src/fx.rs` validation; `tests/proptest.rs::from_str_values_roundtrip` |
| T4 | Malformed numeric strings panic | DoS | `from_str_values` | Parse errors returned as `MoneyError` | `src/amount.rs::from_str_values` rejection paths; `tests/proptest.rs::bounded_decimal`, `from_str_values_roundtrip` |
| T5 | Float leakage into value logic | Spoofing | `to_f64` | Conversion exists but returns `Option` and is isolated from arithmetic; display formats from `Decimal` | `tests/proptest.rs::display_always_starts_with_symbol`, `round_preserves_currency` |

**OPEN RISKS**

- **OPEN-1 — `FxRate::inverse` panics on a zero rate** (documented
  `# Panics` section; unreachable via `new` but reachable if a caller
  hand-constructs `FxRate { .. }` literally, since fields are public).
- **OPEN-2 — rounding mode is caller-invisible.** `round_to` uses
  `rust_decimal`'s default (banker's? half-up?) without an explicit mode
  parameter or test pinning mid-value behavior — financial-grade rounding
  policy is unverified.
- **OPEN-3 — no amount magnitude cap.** `bounded_decimal` bounds the
  *property strategy*, not the API; extreme-but-valid decimals (near
  96-bit bounds) are accepted.

**Out of scope:** price-feed authenticity (an `FxProvider` serves whatever
it is given); double-entry/ledger concerns; locale-correct formatting beyond
`FormatConfig` presets.

**Residual risk:** `to_f64` remains available for display/serialization
boundaries — value comparisons done on the f64 result are lossy by
construction.
