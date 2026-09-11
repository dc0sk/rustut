<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 15.1 — sum_where

## Task

Implement `sum_where(data, keep, f)`: sum `f(x)` for every `x` in `data`
for which `keep(x)` is true. Use iterator combinators — `.iter()`,
`.filter()`, `.map()`, `.sum()` — not a hand-written `for` loop with
indexing.

## Done when

- `cargo test -p ch15-ex01-sum-where` passes
- `cargo clippy -p ch15-ex01-sum-where --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Passing closures as ordinary parameters (`impl Fn(...) -> ...`) — the
  same job C would need a function pointer plus a hand-threaded context
  parameter to do.
- Composing `.filter()`/`.map()`/`.sum()` instead of a manual indexed loop
  with an accumulator variable.
