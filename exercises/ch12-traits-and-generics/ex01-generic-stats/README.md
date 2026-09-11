<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 12.1 — generic stats

Implement `highest_scored`, a function generic over **any** type that
implements the `Scored` trait — not just one hardcoded struct. `tests/public.rs`
exercises it with two unrelated types (`Player` and `Bid`) that only have
`Scored` in common, to make sure your implementation doesn't accidentally
assume anything else about `T`.

## Task

Fill in `highest_scored` in `src/lib.rs`. It should return a reference to
the item with the maximum `.score()` in the slice.

## Done when

- `cargo test -p ch12-ex01-generic-stats` passes
- `cargo clippy -p ch12-ex01-generic-stats --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Writing a function generic over a trait bound (`T: Scored`), the same
  shape as C's "I'd need `void*` or a macro for this" problem, but
  type-checked at compile time with zero runtime cost per call.
- The bound is a compile-time contract: this function will only compile
  for types that actually implement `Scored`, so the compiler catches a
  missing implementation instead of you catching a garbage `void*` cast
  at runtime.
