<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 30.1 — clamp range

Implement `clamp_all(values: &[i32], lo: i32, hi: i32) -> Vec<i32>`,
returning a copy of `values` with every element pulled into `lo..=hi`
(`i32::clamp` does the per-element work). If `lo > hi`, panic — that's an
invalid range, not something to silently paper over.

## Task

Fill in `clamp_all`'s body. The crate already includes:
- unit tests (a few hand-picked examples, including a `#[should_panic]`
  case for the invalid-range panic),
- a property test (`property_tests` module, using `proptest`) checking
  that for **any** input slice and **any** two bounds, every element of
  the output ends up within `lo..=hi` and the output length matches the
  input length — an invariant checked against hundreds of randomly
  generated cases, not just the examples above.

## Done when

- `cargo test -p ch30-ex01-clamp-range` passes (this runs the unit tests,
  the property test, and `tests/public.rs`)
- `cargo clippy -p ch30-ex01-clamp-range --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Unit tests (inline, `#[cfg(test)]`) vs. integration tests (`tests/*.rs`,
  seeing only the public API) vs. property tests (`proptest!`, checking an
  invariant across many generated inputs) are three different tools, not
  three names for the same thing — this exercise deliberately uses all
  three.
- `#[should_panic(expected = "...")]` asserts a specific panic happens,
  not just any panic.
