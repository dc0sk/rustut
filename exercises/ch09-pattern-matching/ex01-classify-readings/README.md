<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 9.1 — classify readings

Implement `classify` (see the doc comment on it in `src/lib.rs` for the
exact rules) using a single `match` over `SensorReading`, with match
guards for the numeric thresholds.

## Done when

- `cargo test -p ch09-ex01-classify-readings` passes
- `cargo clippy -p ch09-ex01-classify-readings --all-targets --all-features -- -D warnings` is clean

## Learning goals

- A `match` guard (`Pattern if condition`) lets a single arm add a
  condition on top of the shape it already matched — something C's
  `switch` can't express, since `case` labels must be compile-time
  constants.
- Order matters: guards are checked top to bottom, so the `> 90.0` arm
  must come before the `> 75.0` arm, or the stricter case would never be
  reached.
- Destructuring `Error { code, message }` directly in the pattern gives
  you both fields at once, instead of a struct literal plus separate
  `.field` accesses.
