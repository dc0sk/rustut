<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 4.1 — safe narrowing

## Task

Implement `to_port`, which narrows an `i64` into a `u16`, and
`checked_sum`, which adds two `u8`s — both without C's silent
truncation/wraparound behavior:

- `to_port` must return `Err(value)` (the original, unmodified input) when
  it doesn't fit in a `u16`, not a truncated low-16-bits value.
- `checked_sum` must return `None` on overflow, not a wrapped or panicking
  result.

## Done when

- `cargo test -p ch04-ex01-safe-narrowing` passes
- `cargo clippy -p ch04-ex01-safe-narrowing --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `TryFrom`/`try_into` is the checked, explicit alternative to C's
  implicit narrowing conversions — see Chapter 4's `compile_fail` example
  for what happens if you try to skip it.
- `checked_add` (and its siblings `checked_sub`/`checked_mul`/...) turn
  "did this overflow?" into an `Option` you're forced to handle, instead
  of a debug-only panic or a release-mode silent wrap.
