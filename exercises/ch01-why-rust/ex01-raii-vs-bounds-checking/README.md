<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 1.1 — RAII vs. bounds checking

Two of the bug classes Chapter 1 talks about, made concrete.

## Task 1 — `safe_get`: buffer overrun, prevented

In C, `arr[i]` performs no bounds check — an out-of-range `i` is undefined
behavior. Implement `safe_get` so an out-of-range `index` returns `None`
instead of reading memory that isn't part of `data`.

## Task 2 — `Resource`: RAII vs. forgotten cleanup

In C, every `return`/`goto fail`/early exit between a resource being
acquired and released is a place you can forget to release it. Implement
`Resource`'s `Drop` impl so that going out of scope — on *any* return path
in `use_resource`, including the early one — increments `releases` exactly
once. You don't need to understand the `'a` lifetime annotation in detail
yet (Chapter 7 covers that); just fill in the body of `drop`.

## Done when

- `cargo test -p ch01-ex01-raii-vs-bounds-checking` passes
- `cargo clippy -p ch01-ex01-raii-vs-bounds-checking --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Rust's `[]` indexing panics rather than reading out-of-bounds memory, and
  `.get()` sidesteps even the panic by returning `Option`.
- `Drop` runs deterministically when a value goes out of scope, regardless
  of *which* control-flow path caused the scope to end — this is the
  mechanism behind "RAII" that later chapters (16, 23) build on heavily.
