<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Threads

## Spawning and joining

`std::thread::spawn` is Rust's `pthread_create`; `JoinHandle::join()` is
`pthread_join`. The interesting difference isn't the API shape — it's what
the compiler forces you to prove before it lets you call `spawn` at all:

```rust,ignore
{{#include ../../examples/ch18-threads/src/lib.rs:spawn_join}}
```

The `move` keyword transfers ownership of `data` into the closure. That's
not a stylistic choice — try to use `data` again after the `spawn` call
and the compiler rejects it, the same use-after-move error Chapter 5
introduced. In C, `pthread_create(&t, NULL, worker, &data)` compiles
identically whether `worker` is the only thing touching `data` or whether
the spawning thread keeps reading and writing it concurrently — the
correctness of that depends entirely on what the programmer does next,
checked by nobody. Here, "who's allowed to touch this, and when" is a
fact the type system tracks.

> **C engineer's mental model.** A `JoinHandle<T>` is a `pthread_t` that
> remembers what type its thread returns. `handle.join()` blocks like
> `pthread_join`, but comes back with a `Result` instead of an output
> parameter — see the next section for why that `Result` matters.

## A panic doesn't take the process down

```rust,ignore
{{#include ../../examples/ch18-threads/src/lib.rs:thread_panic}}
```

A panic inside a spawned thread unwinds *that thread*, and is caught at
the thread boundary — `join()` returns `Err` holding the panic payload,
and every other thread, including the one that called `join`, keeps
running. Compare to C: an unhandled fault in one thread — a bad
dereference, an assertion via `abort()` — typically delivers a
process-wide signal that takes every thread down with it, logs included,
often mid-write. Rust's default is closer to "a failed request in one
worker doesn't crash the server" than to "one bad pointer takes everything
down."

This is not a reason to ignore panics — a poisoned `Mutex` (Chapter 19) or
a genuinely corrupted invariant is still a real problem — but it does mean
a thread crashing is *information you can act on*, not an unrecoverable
process death.

## Scoped threads: borrowing without `Arc`

```rust,ignore
{{#include ../../examples/ch18-threads/src/lib.rs:scoped_threads}}
```

`thread::scope` guarantees every thread spawned inside the scope is joined
before the scope returns — and that guarantee is exactly what lets the
borrow checker allow those threads to hold references (`left`, `right`)
into `data`, which lives on the *caller's* stack and isn't `'static`.
Without that guarantee, a thread could in principle outlive the data it
borrowed — which is precisely the C bug of handing a spawned thread a
pointer into a stack frame and hoping the parent doesn't return first,
made structurally impossible here instead of merely discouraged.

Before scoped threads existed, sharing non-`'static` data across a plain
`thread::spawn` required wrapping it in `Arc` (Chapter 19) specifically to
get a `'static` owned handle each thread could hold independently. Reach
for `thread::scope` first when the data doesn't need to outlive the
function that spawned the threads — it's simpler and avoids the
reference-counting overhead entirely.

## Exercise

**Exercise 18.1**, in
[`exercises/ch18-threads/ex01-parallel-sum/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch18-threads/ex01-parallel-sum),
has you split a slice across scoped threads and sum it in parallel. See
its `README.md`.

Next: [Chapter 19 — Send, Sync & Fearless Concurrency](19-send-sync.md).
