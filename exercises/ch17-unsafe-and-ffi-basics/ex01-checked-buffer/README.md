<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 17.1 — checked buffer

## Task

Implement `checked_get_unchecked` so it behaves exactly like the safe
`.get(index)` from the outside (returns `None` on an out-of-bounds
`index`, never panics, never reads out-of-bounds memory) — but internally
must read the element through `data.get_unchecked(index)`, not `[]` and
not `.get()`. That means **you** are responsible for the bounds check
`get_unchecked` skips; do it before the `unsafe` block, and write the
`// SAFETY:` comment explaining why the access is sound at that point.

## Done when

- `cargo test -p ch17-ex01-checked-buffer` passes
- `cargo clippy -p ch17-ex01-checked-buffer --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `unsafe` on `get_unchecked` doesn't mean "no bounds check happens" — it
  means "no bounds check happens *automatically*." The check still has to
  exist; you just have to write it yourself, and say why it's sufficient.
- A safe function can wrap an unsafe operation completely: nothing about
  this function's public signature is `unsafe`, because you've upheld the
  precondition internally, unconditionally, for every possible input.
