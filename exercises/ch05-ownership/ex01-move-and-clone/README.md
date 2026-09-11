<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 5.1 — move and clone

## Task

Implement three small functions that between them cover the three things
this chapter is about:

- `append_owned` — takes ownership of *both* arguments (they're `String`,
  not `&str`); the caller gives them up entirely.
- `double_in_place` — takes ownership of a `Vec<i32>`, mutates it, and
  hands the same allocation back through the return value.
- `duplicate` — takes only a *borrow* (`&str`) and returns a fresh, owned
  `String` via `.clone()`-equivalent behavior, leaving the original
  usable. (`to_owned()`/`to_string()` are both fine here — they do the
  same job as `.clone()` for a `&str`.)

## Done when

- `cargo test -p ch05-ex01-move-and-clone` passes
- `cargo clippy -p ch05-ex01-move-and-clone --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Taking a parameter by value (`String`, `Vec<i32>`) moves it — the
  function now owns it, and the caller cannot use their original binding
  afterward.
- Ownership can flow back out through a return value with no new
  allocation and no "who frees this?" ambiguity — the type signature IS
  the ownership contract.
- Taking a parameter by shared reference (`&str`) only borrows it; the
  caller keeps ownership, and if you want your own independent copy
  inside the function, you clone explicitly.
