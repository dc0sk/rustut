<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Channels & Message Passing

Chapter 19 gave threads shared access to memory, protected by a lock.
There's a different, and often simpler, way to coordinate threads: don't
share the memory at all — hand ownership of each value from one thread to
another. Rust's slogan for this, borrowed from Go/CSP, is: **share memory
by communicating, rather than communicate by sharing memory.**

## `std::sync::mpsc`: multi-producer, single-consumer

```rust,ignore
{{#include ../../examples/ch20-channels/src/lib.rs:single_producer}}
```

`tx.send(v)` **moves** `v` into the channel — the sending thread no longer
has it (Chapter 5's ownership rules apply here exactly as everywhere
else). The receiving side (`rx`) gets that same value, owned, with no
lock ever taken on either side. `for v in rx` keeps pulling values until
the channel **closes** — which happens automatically once every `Sender`
tied to this channel has been dropped.

> **C engineer's mental model.** A hand-rolled C producer/consumer queue
> needs: a mutex protecting the queue, a condition variable to wake a
> waiting consumer, and *some* way for the consumer to know the producer
> is finished — usually a sentinel value pushed onto the queue, or a
> separate `done` flag that itself needs to be read under the same lock
> as the queue (get that wrong and you deadlock or miss the signal). A
> Rust channel gives you all three for free: the queue, the wakeup, and a
> structural "no more data" signal derived from ownership itself — there
> is no way to "forget" to send the sentinel, because there is no
> sentinel; the channel just closes when the last `Sender` goes out of
> scope, the same automatic-cleanup guarantee `Drop` gives you everywhere
> else in this book.

## Multiple producers

```rust,ignore
{{#include ../../examples/ch20-channels/src/lib.rs:multi_producer}}
```

Cloning a `Sender` gives you another handle to the *same* channel — that's
the "mp" in `mpsc`. The channel only closes once **every** clone (plus the
original) has been dropped, which is why the example explicitly `drop(tx)`
after spawning: the original `tx` is a live `Sender` too, and forgetting
to drop it would mean the channel never closes, and `rx.iter()` would
block forever waiting for more values that will never come — a real,
common bug the first time you write this pattern.

Note the two separate guarantees the example keeps distinct: the channel
closing tells you every `Sender` was dropped; joining each `JoinHandle`
tells you every thread actually finished running (and didn't panic). You
generally want both.

## When to reach for a channel vs. a `Mutex`

- A channel moves ownership of discrete values between threads — a
  natural fit for "pipeline" or "worker pool" shapes, where each item is
  handled by exactly one consumer.
- A `Mutex` (Ch. 19) protects state that's read *and* written by multiple
  threads over time, in place — a natural fit for a shared counter, cache,
  or table that many threads touch repeatedly.

Both are safe by construction in Rust; picking between them is a design
question, not a safety one. (The standard library's `mpsc` is
single-consumer; if you need multiple consumers pulling from the same
channel, the community crate `crossbeam-channel` provides that — not used
in this book, but worth knowing it exists.)

## Exercise

**Exercise 20.1**, in
[`exercises/ch20-channels/ex01-fan-in-aggregator/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch20-channels/ex01-fan-in-aggregator),
has you fan out work across several threads and fan the results back in
through a shared channel. See its `README.md`.

Next: [Chapter 21 — Atomics & Memory Ordering](21-atomics.md).
