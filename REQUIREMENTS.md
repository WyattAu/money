# Requirements — money

Numbered, testable requirements. Every requirement maps to at least one named
test; every security-relevant test cites at least one requirement. Doc
comments on the implementing public item carry `REQ-MNY-NNN` tags.

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-MNY-001 | `CurrencyAmount::new` stores the exact decimal value with its `Currency` | MUST |
| REQ-MNY-002 | `CurrencyAmount::from_str_values` parses decimal strings into an amount, rejecting malformed input with `Err` | MUST |
| REQ-MNY-003 | `is_zero` / `is_positive` / `is_negative` agree with the sign of the stored value for all amounts | MUST |
| REQ-MNY-004 | `abs` returns a non-negative amount of the same currency; `negate` flips the sign; `negate` twice yields the original | MUST |
| REQ-MNY-005 | `Add`/`Sub` require equal currencies and return `MoneyError::CurrencyMismatch` otherwise (never silently mix currencies) | MUST |
| REQ-MNY-006 | `round()` rounds to the currency's `decimal_places`; `round_to(n)` to `n` places; both preserve the currency | MUST |
| REQ-MNY-007 | `round_to_with_policy` honours every `RoundingPolicy` variant (HalfUp, HalfDown, HalfEven, HalfCeiling, HalfFloor, Ceiling, Floor, AwayFromZero) including negative amounts | MUST |
| REQ-MNY-008 | `allocate(ratios)` partitions an amount so parts sum to exactly the original (largest-remainder, no penny loss/gain), with scale-independent and non-unity ratio weights | MUST |
| REQ-MNY-009 | `allocate` rejects empty ratios, zero-sum ratios, and negative ratios; zero ratios are allowed and yield zero parts; negative amounts allocate exactly | MUST |
| REQ-MNY-010 | `allocate` with a single ratio returns the (rounded) original amount | SHOULD |
| REQ-MNY-011 | `parts()` splits an amount into major/minor decimal components per the currency's exponent | MUST |
| REQ-MNY-012 | `to_f64` reports lossy conversion via `Option` rather than silently truncating | SHOULD |
| REQ-MNY-013 | `Currency` exposes correct ISO 4217 `code`, `symbol`, `decimal_places`, and `is_crypto` for every variant; `Display` is the ISO code | MUST |
| REQ-MNY-014 | `Currency::from_str` parses ISO codes case-insensitively and round-trips with `Display` for all variants | MUST |
| REQ-MNY-015 | `FormatConfig::us` / `european` / `iso` presets format per their conventions; `iso` prefixes the ISO code; symbol position and thousands/decimal separators are honoured | MUST |
| REQ-MNY-016 | Formatting truncates extra fractional digits (never rounds) and renders zero without a negative sign; `Display` of `CurrencyAmount` is stable | MUST |
| REQ-MNY-017 | `FxRate::new` rejects zero and negative rates with `Err`; `inverse()` inverts the pair; `identity(c)` yields rate 1 | MUST |
| REQ-MNY-018 | `convert` multiplies by the quoted rate and returns the target currency; conversion with a missing pair is an error | MUST |
| REQ-MNY-019 | `InMemoryFxProvider::set_rate` rejects non-positive rates, `load_rates` bulk-inserts/overwrites, and lookups filter by source currency and error on unknown pairs | MUST |
| REQ-MNY-020 | `RoundingPolicy::to_strategy` maps each policy to the expected `rust_decimal` strategy | SHOULD |
| REQ-MNY-021 | `MoneyError` variants render stable `Display` messages and wrap `rust_decimal` parse errors as `InvalidAmount` | SHOULD |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-MNY-100 | No arithmetic or parsing path panics on arbitrary input — bad strings and invalid rates return `Err(MoneyError)` (fuzz target `fuzz_arith` exercises this) | MUST |
| REQ-MNY-101 | Monetary values are stored as fixed-point `Decimal`, never binary floats; float conversion is explicit and lossy-flagged (`to_f64`) | MUST |
| REQ-MNY-102 | `allocate` conserves value exactly: sum of parts equals the input for arbitrary ratio sets (no creation or destruction of value beyond rounding) | MUST |
| REQ-MNY-103 | Currency mixing is rejected with `CurrencyMismatch` on arithmetic — cross-currency leakage requires an explicit `convert` with a validated positive rate | MUST |
| REQ-MNY-104 | FX rates must be strictly positive and finite; zero/negative rates are rejected at both `FxRate::new` and provider `set_rate` | MUST |
| REQ-MNY-105 | The crate forbids `unsafe` code; no secret-bearing inputs (e.g. full PAN-style data) are required or logged by any public function | MUST |

## Robustness

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-MNY-200 | Property: addition is commutative with zero as identity; subtraction is the inverse — for arbitrary amounts and currencies | MUST |
| REQ-MNY-201 | Property: sign predicates, `abs`, and `round` maintain invariants for arbitrary generated amounts (abs non-negative, round preserves currency) | MUST |
| REQ-MNY-202 | Property: allocation part sums equal the original for arbitrary ratio vectors, including scaled variants and negative amounts | MUST |
| REQ-MNY-203 | Property: `from_str_values` round-trips and `Currency` from-str round-trips for all valid inputs | MUST |
| REQ-MNY-204 | Formatting of arbitrarily generated amounts always starts with a currency symbol when configured and never emits malformed grouping | SHOULD |

## Traceability Matrix

| Requirement | Test (fn, file) | Property class |
|-------------|-----------------|----------------|
| REQ-MNY-001 | `test_new_amount` (`src/amount.rs`) | unit |
| REQ-MNY-002 | `test_from_str_values` (`src/amount.rs`), `from_str_values_roundtrip` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-003 | `test_predicates` (`src/amount.rs`), `is_zero_matches_value` / `is_positive_matches_value` / `is_negative_matches_value` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-004 | `test_predicates` (`src/amount.rs`), `abs_always_non_negative`, `negation_doubles_to_zero` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-005 | `test_addition_currency_mismatch`, `test_subtraction_currency_mismatch` (`src/amount.rs`), `addition_currency_mismatch_fails` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-006 | `test_round`, `test_round_to_explicit_places` (`src/amount.rs`), `round_preserves_currency` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-007 | `test_round_to_with_policy_half_up_and_even`, `test_round_to_with_policy_half_down_floor_ceiling`, `test_round_to_with_policy_negative_half_up_away_from_zero` (`src/amount.rs`) | unit |
| REQ-MNY-008 | `test_allocate_even_split_largest_remainder`, `test_allocate_scale_independent_weights`, `test_allocate_ratio_weights_not_unity`, `test_allocate_tie_breaks_by_ratio_order` (`src/amount.rs`), `allocate_parts_sum_exactly`, `allocate_scale_variant_ratios_sum_exactly` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-009 | `test_allocate_rejects_empty`, `test_allocate_rejects_negative_ratio`, `test_allocate_rejects_zero_sum_ratios`, `test_allocate_zero_ratio_is_allowed`, `test_allocate_zero_amount`, `test_allocate_negative_amount_sums_exactly` (`src/amount.rs`), `allocate_negative_ratio_rejected`, `allocate_zero_sum_ratios_rejected` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-010 | `test_allocate_single_ratio_is_identity` (`src/amount.rs`), `allocate_single_ratio_is_identity` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-011 | `test_parts_splits_major_and_minor` (`src/amount.rs`) | unit |
| REQ-MNY-012 | `test_to_f64` (`src/amount.rs`) | unit |
| REQ-MNY-013 | `test_all_currencies_metadata`, `test_currency_codes`, `test_currency_symbols`, `test_decimal_places`, `test_is_crypto`, `test_currency_display_is_iso_code`, `test_new_currencies_exponents` (`src/currency.rs`) | unit |
| REQ-MNY-014 | `test_from_str`, `test_all_currencies_parse_display_roundtrip` (`src/currency.rs`), `currency_from_str_roundtrip` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-015 | `test_us_format`, `test_european_format`, `test_iso_format_prefixes_iso_code`, `test_no_symbol_omits_currency_entirely`, `test_zero_decimal_places` (`src/format.rs`) | unit |
| REQ-MNY-016 | `test_extra_fractional_digits_are_truncated_not_rounded`, `test_zero_amount_has_no_negative_sign`, `test_negative_amount`, `test_display` (`src/amount.rs`, `src/format.rs`), `display_always_starts_with_symbol` (`tests/proptest.rs`) | unit/property |
| REQ-MNY-017 | `test_fx_rate_rejects_zero`, `test_fx_rate_rejects_negative`, `test_fx_rate_inverse` (`src/fx.rs`) | unit |
| REQ-MNY-018 | `test_convert_multiplies_by_rate` (`src/fx.rs`) | unit |
| REQ-MNY-019 | `test_in_memory_provider`, `test_set_rate_rejects_non_positive`, `test_load_rates_bulk_inserts_and_overwrites`, `test_get_rate_unknown_pair_is_an_error`, `test_get_rates_from_filters_by_source` (`src/fx.rs`) | unit |
| REQ-MNY-020 | `policies_map_to_expected_strategies` (`src/rounding.rs`) | unit |
| REQ-MNY-021 | `display_messages_are_stable`, `from_rust_decimal_error_wraps_as_invalid_amount` (`src/error.rs`) | unit |
| REQ-MNY-100 | `fuzz/fuzz_targets/fuzz_arith.rs` (`bounded_decimal`) | fuzz |
| REQ-MNY-101 | `test_to_f64` (`src/amount.rs`); `Decimal`-only storage asserted by `tests/proptest.rs` arithmetic properties | unit/property |
| REQ-MNY-102 | `allocate_parts_sum_exactly`, `allocate_parts_close_to_exact_share` (`tests/proptest.rs`), `test_allocate_even_split_largest_remainder` (`src/amount.rs`) | property |
| REQ-MNY-103 | `test_addition_currency_mismatch` (`src/amount.rs`), `test_convert_multiplies_by_rate` (`src/fx.rs`) | unit |
| REQ-MNY-104 | `test_fx_rate_rejects_zero`, `test_set_rate_rejects_non_positive` (`src/fx.rs`) | unit |
| REQ-MNY-105 | `#![forbid(unsafe_code)]` in `src/lib.rs`; fuzz target exercises parse paths with arbitrary bytes (`fuzz/fuzz_targets/fuzz_arith.rs`) | fuzz/build |
| REQ-MNY-200 | `addition_commutative`, `addition_zero_identity`, `subtraction_inverse` (`tests/proptest.rs`) | property |
| REQ-MNY-201 | `abs_always_non_negative`, `round_preserves_currency`, `negation_doubles_to_zero` (`tests/proptest.rs`) | property |
| REQ-MNY-202 | `allocate_parts_sum_exactly`, `allocate_negative_amount_sums_exactly`, `allocate_scale_variant_ratios_sum_exactly` (`tests/proptest.rs`) | property |
| REQ-MNY-203 | `from_str_values_roundtrip`, `currency_from_str_roundtrip` (`tests/proptest.rs`) | property |
| REQ-MNY-204 | `display_always_starts_with_symbol` (`tests/proptest.rs`), `test_format_config_display` (`src/format.rs`) | property/unit |
