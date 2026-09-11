<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 10.1 — lookup and parse

## Task

1. `first_positive(values: &[i32]) -> Option<i32>` — the first positive
   value, or `None`. Use iterator combinators (`.iter().find(...)`, or
   similar) instead of a manual loop with a `found`/`result` flag variable.
2. `parse_pair(input: &str) -> Result<(i64, i64), String>` — parse a
   string like `"3,4"` into `(3, 4)`. Three things can go wrong, and all
   three must come back as `Err(String)` with a message, not a panic:
   - there's no `,` in the input at all (`str::split_once` returns
     `Option`, not `Result` — turn its `None` into an `Err` with
     `.ok_or_else()`)
   - the left side doesn't parse as an `i64`
   - the right side doesn't parse as an `i64`

   Use the `?` operator to propagate each failure instead of nested
   `match`.

## Done when

- `cargo test -p ch10-ex01-lookup-and-parse` passes
- `cargo clippy -p ch10-ex01-lookup-and-parse --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Iterator combinators (`.find()`, `.map()`) express "look for the first
  match" without a hand-written loop-and-flag, and return `Option`
  directly.
- `Option` and `Result` compose: `.ok_or_else()` converts a `None` into an
  `Err`, letting one `?`-chain handle both "missing" and "malformed"
  failures uniformly.
