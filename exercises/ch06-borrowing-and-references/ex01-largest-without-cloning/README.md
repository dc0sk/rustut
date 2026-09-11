<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 6.1 — borrow, don't clone

## Task 1 — `largest`

Implement `largest` so it returns a reference to the largest element of
`nums`, or `None` for an empty slice. The point is what you *don't* do:
no `.clone()`, no copying elements into a new `Vec`, no indexing that
allocates — just a borrow into the slice the caller already owns.

## Task 2 — `grow_then_summarize`

Implement `grow_then_summarize` so it pushes `extra` onto `data`, then
returns a message like `"5 numbers, sum 15"` describing `data`'s new
state. This exercises the "mutable borrow, then immutable borrow, in
sequence" pattern from the guided example — not at the same time.

## Done when

- `cargo test -p ch06-ex01-largest-without-cloning` passes
- `cargo clippy -p ch06-ex01-largest-without-cloning --all-targets --all-features -- -D warnings` is clean

## Learning goals

- A function can return a reference derived from its input without any
  explicit lifetime annotation, because lifetime elision covers the
  common "one reference in, one reference out" case automatically
  (Chapter 7 covers the rule this relies on).
- Borrowing a slice costs nothing at runtime beyond a pointer and a
  length — no allocation, no copy — and the compiler guarantees that
  length can never be wrong for the data it describes.
