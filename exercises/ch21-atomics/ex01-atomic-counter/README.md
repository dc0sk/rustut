<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 21.1 — atomic counter

## Task

Implement `record_request(counter: &AtomicU32, max: u32) -> bool`: a
saturating counter that increments unless it's already reached `max`.

The naive approach — `let v = counter.load(...); if v < max {
counter.store(v + 1, ...); true } else { false }` — has a race: two
threads can both load the same value below `max`, both decide there's
room, and both store, overshooting `max`. Use `compare_exchange` in a
retry loop instead, so the "check, then act" is a single indivisible
step relative to every other thread.

## Done when

- `cargo test -p ch21-ex01-atomic-counter` passes (including a test that
  hammers the counter from 16 threads and checks it never exceeds `max`)
- `cargo clippy -p ch21-ex01-atomic-counter --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Why "load, check, store" as three separate steps is not thread-safe
  even though each individual step is atomic — the race is *between* the
  steps, not within any one of them.
- `compare_exchange`'s retry-loop pattern: the standard shape for any
  atomic "read-modify-write, but only if the value hasn't changed
  since I read it."
