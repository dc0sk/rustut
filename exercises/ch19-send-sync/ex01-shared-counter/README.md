<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 19.1 — a counter safe to share

Implement `SharedCounter`: a counter that can be wrapped in an `Arc` and
shared across as many threads as you like, with `add`/`get` that are
race-free no matter how many threads call them concurrently.

## Task

Fill in `SharedCounter::new`, `add`, and `get` using the `Mutex<i64>`
field already declared. The point of this exercise is the *shape* of the
API, not just making the lock work: nothing outside this module should
ever be able to touch the counter's value without going through `add`/
`get` — there's no `pub` accessor to the raw `Mutex`.

## Done when

- `cargo test -p ch19-ex01-shared-counter` passes (including a test that
  spawns 8 threads doing 1000 increments each and asserts the total is
  exactly 8000 — if your locking is wrong, this test will flake or fail,
  not just "look wrong")
- `cargo clippy -p ch19-ex01-shared-counter --all-targets --all-features -- -D warnings` is clean

## Learning goals

- The lock and the data it protects can be one type, not two things a
  caller has to remember to use together — the same point Chapter 19's
  guided example makes with a free function, taken one step further by
  hiding the `Mutex` inside the struct entirely.
- `Arc<T>` (not `Rc<T>`) is what makes `SharedCounter` shareable across
  threads at all — try swapping in `Rc` and watch `cargo build` reject it.
