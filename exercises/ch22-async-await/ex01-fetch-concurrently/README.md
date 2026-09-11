<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 22.1 — fetch concurrently

`fetch_one` (already written, don't touch it) simulates one network fetch:
it `.await`s a sleep proportional to `delay_ms`, then returns it. Your job
is `fetch_all`, which must run every delay in `delays_ms` **concurrently**
— not sequentially — and return the results in the same order as the
input.

## Task

Implement `fetch_all` using `tokio::spawn` (one task per delay) and await
every `JoinHandle` afterward, preserving input order. Don't use
`std::thread::sleep` anywhere — that blocks the runtime thread instead of
yielding to it (Chapter 22 covers exactly why that's a trap).

## Done when

- `cargo test -p ch22-ex01-fetch-concurrently` passes
- `cargo clippy -p ch22-ex01-fetch-concurrently --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `tokio::spawn` schedules a future to run concurrently with the current
  task, returning a `JoinHandle` you `.await` to get its result back.
- "Concurrently" is a claim you can actually measure: the test suite
  checks wall-clock time, not just the returned values, the same way this
  chapter's guided example does.
