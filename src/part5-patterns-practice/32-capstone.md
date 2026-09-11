<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Capstone Project

Every chapter so far has taught one thing at a time, with a small, focused
exercise to match. Real programs don't come in one-concept pieces. This
capstone is a single, bounded project that pulls a good fraction of the
book together at once: a concurrent, persistent, networked key-value
store — small enough to build in an evening, real enough that every
design decision in it is one you'll actually make again.

Unlike every other chapter, there's no separate `examples/` crate here.
The exercise itself — `exercises/ch32-capstone-project/ex01-kv-server/` —
*is* the guided material: a complete, working shell with three focused
gaps for you to fill in, plus a full reference solution to compare
against afterward.

## The spec

A TCP server that speaks a tiny newline-delimited text protocol:

```text
SET <key> <value...>   -> OK / ERROR <message>
GET <key>               -> VALUE <value> / NOT_FOUND
DELETE <key>            -> OK / NOT_FOUND
```

```rust,ignore
{{#include ../../solutions/ch32-capstone-project/ex01-kv-server/src/lib.rs:command_enum}}
```

Concretely, it needs to:

- **Parse CLI arguments** (Ch. 23) for the listen address and the path to
  a JSON persistence file.
- **Load** that file at startup if it exists, and **persist** to it after
  every mutation (Ch. 24, Ch. 29) — simple beats clever here: no
  background save thread, no write batching, just "a `SET`/successful
  `DELETE` writes the whole store back out before it returns `OK`."
- **Accept multiple simultaneous client connections** (Ch. 18, Ch. 25),
  each on its own thread, all sharing one in-memory store behind a
  `Mutex` (Ch. 19) — the point being that two connections really do see
  each other's writes, which is the actual thing "shared mutable state"
  has to mean for a key-value store to be useful.
- **Never panic on malformed client input.** A garbage line gets an
  `ERROR` response on that connection; it does not take down the thread,
  and it certainly does not take down the process.
- **Be clippy-clean at `-D warnings`** (Ch. 31) — this repository's own
  CI standard applies to the capstone too, not just the earlier chapters.

## The pattern: typestate, not a builder

Ch. 27 covered both the builder and typestate patterns, and this project
had to pick one for its server lifecycle. A builder would fit
`ServerConfig` construction if that config had optional fields or
validation worth staging — it doesn't; it's two plain fields. What *does*
have a real invariant worth enforcing is the sequence "configure, then
bind a socket, then run" — you cannot meaningfully call `.run()` before
`.bind()` has actually claimed a port, and (this is the detail that makes
it more than an academic distinction) you cannot read back *which* port
you got, when you asked for an OS-assigned one, until after binding
either. That's exactly what a typestate encodes as a compile-time fact
instead of a runtime precondition:

```rust,ignore
{{#include ../../solutions/ch32-capstone-project/ex01-kv-server/src/lib.rs:typestate}}
```

`Server<Unbound>` has `bind`; `Server<Bound>` has `local_addr` and `run`.
There is no method that exists on both, and no way to call `run` a
compile error away from having a real listener behind it. This is also
precisely what makes the test suite's ephemeral-port pattern from Ch. 25
(`"127.0.0.1:0"`, then read back the real port) work cleanly here: bind
first, read `local_addr()`, *then* hand the bound server off to a spawned
thread to `run()` while the test connects to the now-known port.

## Where the shared lock actually matters

```rust,ignore
{{#include ../../solutions/ch32-capstone-project/ex01-kv-server/src/lib.rs:handle_command}}
```

Every command — even a read-only `Get` — takes the same `Mutex<Store>`.
That's the entire concurrency story for this project: no lock-free
tricks, no per-key locking, just one mutex serializing access to one
`HashMap`. It's not the most scalable design possible, and that's fine —
Ch. 21 covers when you'd reach for something finer-grained (an atomic
counter, a `RwLock` for read-heavy workloads); the point of this capstone
is that the *default*, boring, obviously-correct choice is one line of
type (`Mutex<Store>`) and one lock call per operation, not a hand-rolled
locking protocol you have to get right by convention the way you would in
C.

## What you build

Read `exercises/ch32-capstone-project/ex01-kv-server/README.md` for the
exact task, its `## Done when` commands, and a Design Rubric checklist —
this exercise is graded partly by test (`cargo test`), partly by a human
or agent walking the rubric, the same free-form-exercise protocol
`AGENTS.md` establishes for Ch. 27's typestate exercise. Try it yourself
before reading `solutions/ch32-capstone-project/ex01-kv-server/`.

## That's the book

Chapter 1 opened with a table of bug classes you'd already spent hours
debugging in C, and a promise: that by the time you'd finished this book,
you'd know exactly which Rust feature closes off each one, and why. If
you've worked through the exercises, that promise is now something you've
built with your own hands thirty-some times over — a buffer that can't
overrun, a resource that can't leak on an early return, a `Mutex` that
makes "forgot to take the lock" a type error instead of a 3am page, a
server that can't accidentally accept a connection before it's bound a
socket.

None of that makes you done learning Rust — no book does that for any
language. What it should have done is put the borrow checker's rules into
your hands well enough that you can now read an error message and know
roughly what it's asking of you, instead of it reading as noise. From
here, the [Appendix](../appendix/glossary-c-to-rust.md) — a C-to-Rust
glossary, a cheat sheet, and further reading — is reference material
worth keeping open in a tab for the next few months, not something to
read straight through.
