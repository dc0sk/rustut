<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 7.1 — a struct that holds a reference

Implement `Parser<'a>::new` and `Parser::next_word`, a hand-rolled
whitespace tokenizer that never allocates: every word it returns is a
slice of the original input string, not a new `String`.

## Done when

- `cargo test -p ch07-ex01-parser-struct` passes
- `cargo clippy -p ch07-ex01-parser-struct --all-targets --all-features -- -D warnings` is clean

## Hints

- `str::trim_start`, `str::find`, and `str::split_at` are enough to
  implement `next_word` without a loop or an external crate.
- You do not need to add or change any lifetime annotations — the struct
  and method signatures are already correct. The exercise is filling in
  the *bodies*.

## Learning goals

- `Parser<'a>` cannot outlive the string slice it borrows — the compiler
  enforces this the same way it would reject a C struct's pointer field
  being read after whatever it pointed to was freed, except here it's
  checked before the program ever runs, not discovered by a fuzzer or a
  customer.
- A method's return type can borrow from the struct's own lifetime
  parameter (`'a`) rather than from `&mut self` — which is exactly why
  the last test in `tests/public.rs` is allowed to use `word` after `p`
  itself has gone out of scope.
