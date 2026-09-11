<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 20.1 — fan-in aggregator

## Task

Implement `square_all_concurrently`: spawn one thread per input value,
each computing that value's square and sending the result back over a
shared `mpsc` channel, then collect every result into a sorted `Vec<i32>`.

Requirements:
- Use `std::sync::mpsc::channel()` — clone the `Sender` for each spawned
  thread, the same "multiple producers" pattern the chapter's guided
  example uses.
- Join every spawned thread before returning (don't just rely on the
  channel closing to prove they're done — that only tells you every
  `Sender` was dropped, not that the thread itself has fully finished
  and won't panic silently).
- The order results arrive in is not guaranteed — sort before returning.

## Done when

- `cargo test -p ch20-ex01-fan-in-aggregator` passes
- `cargo clippy -p ch20-ex01-fan-in-aggregator --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Fan-in: many producer threads, one consumer, coordinated by a channel
  instead of a shared, locked data structure.
- A channel closing (`rx.iter()` ending) and a thread finishing
  (`JoinHandle::join()` returning) are two different guarantees — you
  generally want both, not just one.
