<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 26.1 — bind a C function by hand

`csrc/stats.c` defines `int round_to_nearest(double value)` and is already
compiled by `build.rs` via the `cc` crate — same mechanism the chapter's
guided example used. Your job is the Rust side: declare the `extern "C"`
binding and call it from `round_half_away_from_zero`.

## Task

In `src/lib.rs`:
1. Add an `unsafe extern "C" { ... }` block declaring `round_to_nearest`
   (edition 2024 syntax — the block itself needs `unsafe`).
2. Implement `round_half_away_from_zero` to call it.

## Done when

- `cargo test -p ch26-ex01-clamp` passes
- `cargo clippy -p ch26-ex01-clamp --all-targets --all-features -- -D warnings` is clean

## Learning goals

- The same `cc`-crate + `build.rs` pattern from the chapter, applied to a
  different (still hand-authored, still small) C function.
- A hand-written `extern "C"` declaration has no external tooling
  dependency (unlike `bindgen`, which needs `libclang` at build time) —
  appropriate for a small C file you wrote yourself, as opposed to wrapping
  a large existing library from its header.
