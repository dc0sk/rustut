<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Async/Await & the Async Ecosystem

Chapters 18-21 covered OS threads: the kernel schedules them, preemptively,
with no extra library required — you `spawn`, the scheduler does the rest,
the same as a C program calling `pthread_create`. Async is a different
concurrency model, used for a different problem: running a very large
number of mostly-waiting tasks (network connections, timers, file I/O)
without paying one OS thread's worth of stack and scheduling overhead per
task.

## What `async fn` and `.await` actually are

An `async fn` doesn't run when you call it. Calling it produces a value — a
**future** — that represents "this computation, not yet driven to
completion." Conceptually, the compiler transforms the function body into
a state machine implementing the `Future` trait, where each `.await` point
is a place the state machine can pause and later resume. You never have to
hand-write that state machine or implement `Future` yourself to use async
Rust day-to-day, but it's worth knowing `.await` is fundamentally a
**suspend point**, not a blocking call: it says "pause me here until this
inner future is ready, and let something else run in the meantime."

> **C engineer's mental model.** If you've hand-rolled an `epoll`/`select`
> event loop — register a file descriptor, stash some callback state,
> resume it when the FD becomes ready — you've already built the mental
> model async Rust is based on. `async`/`.await` is the compiler generating
> that state machine and callback bookkeeping for you, instead of you
> writing it by hand for every operation.

## Async needs a runtime; threads don't

A future that's never polled never makes progress — nothing "just runs" it
the way the kernel just runs a spawned thread. That's the job of an async
**runtime** like [`tokio`](https://tokio.rs): it polls futures, drives
timers and I/O readiness, and (for `tokio`'s multi-threaded flavor)
distributes tasks across a pool of OS threads under the hood. This is a
real, structural difference from Chapter 18's threads, which need no
runtime at all — the kernel's scheduler is not optional infrastructure you
opt into, it's just always there.

## A concrete failure mode: cooperative vs. preemptive scheduling

OS threads (Ch. 18) are **preemptively** scheduled: the kernel can suspend
a thread at essentially any point, whether it cooperates or not. Async
tasks on a given runtime worker thread are **cooperatively** scheduled:
a task only yields control at an `.await` point. A task that never
`.await`s anything — or that calls a *blocking* function instead of an
async one — never yields, and starves every other task sharing that
worker thread:

```rust,ignore
{{#include ../../examples/ch22-async-await/src/lib.rs:blocking_vs_yielding}}
```

`blocks_the_runtime_thread` compiles fine, looks unremarkable, and is a
real, common mistake: calling `std::thread::sleep` (or any blocking I/O)
inside an `async fn` blocks the *entire worker thread* the runtime is using
to drive potentially many other tasks — none of them can progress until it
returns. This is the async analogue of a C `epoll` event loop's single
callback calling a blocking `read()` and freezing every other file
descriptor's handling until it returns. (`tokio` provides
`tokio::task::spawn_blocking` for the cases where you genuinely need to run
blocking code without stalling everything else — worth knowing exists,
out of scope for this chapter.)

## A real, running example

```rust,ignore
{{#include ../../examples/ch22-async-await/src/lib.rs:concurrent_sleeps}}
```

Run it yourself:

```sh
cargo run -p ch22-async-await-example --bin demo
```

Five tasks each "wait" 100ms. If they ran sequentially, that's ~500ms. Run
it, and you'll see the real elapsed time is close to 100ms — the tasks
made progress concurrently, each yielding at its `sleep(...).await` point
so the runtime could poll the others. The guided example crate's own test
suite asserts this with a generous bound (comfortably under the sequential
total, without asserting a suspiciously tight number that could flake
under CI scheduling variance) — read it to see the same claim expressed as
a mechanical check instead of a printed number you have to trust.

## Where this goes next

Chapter 25 (Networking Basics) builds directly on this: `tokio::net`'s
`TcpListener`/`TcpStream` are async, and a network server handling many
concurrent connections is the canonical case async was built for — far
more tasks than you'd want one OS thread each.

## Exercise

**Exercise 22.1**, in
[`exercises/ch22-async-await/ex01-fetch-concurrently/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch22-async-await/ex01-fetch-concurrently),
has you implement `fetch_all`, spawning one task per simulated fetch and
awaiting all of them — with a test that, like the guided example, proves
concurrency by measuring wall-clock time. See its `README.md`.

This closes Part 3. Next: [Chapter 23 — Application Error Handling & CLI
Foundations](../part4-pc-applications/23-error-handling-and-cli.md).
