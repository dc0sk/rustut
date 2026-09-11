<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Send, Sync & Fearless Concurrency

## Two marker traits, precisely

`Send` and `Sync` carry no methods — they're **marker traits**, compiler
facts about a type, not behavior:

- `T: Send` means a value of `T` may be **moved** to another thread.
- `T: Sync` means `&T` may be **shared** across threads — equivalently,
  `T` is `Sync` exactly when `&T` is `Send`.

Both are auto-derived: a type built entirely out of `Send`/`Sync` pieces
is `Send`/`Sync` automatically, with nothing to write or annotate. You
only ever see them explicitly when something *isn't* one — usually as a
compiler error, and usually for a good reason.

## Why `Rc<T>` is neither, and `Arc<T>` is both

```rust,ignore
{{#include ../../examples/ch19-send-sync/src/lib.rs:rc_not_send}}
```

`Rc<T>`'s reference count is a plain integer, incremented and decremented
on every clone and drop with no synchronization. Two threads racing on
that increment is a textbook data race — so `Rc<T>` is deliberately **not
`Send`**, and `thread::spawn` requires its argument to be `Send`: the
mismatch is caught at compile time, not discovered under load in
production. `Arc<T>` is the same API with an *atomic* reference count, and
is `Send`/`Sync` — you pay a small, deliberate cost (an atomic
increment/decrement) only in the version that actually needs to cross
thread boundaries.

> **C engineer's mental model.** This is the compiler doing, for free,
> the audit you'd otherwise do by hand: "does anything in this codebase
> share this refcounted object across threads without an atomic
> increment?" In C, that question is answered by grep and code review. In
> Rust, it's answered by `cargo build`.

## `Mutex<T>`: the lock and the data are one value

```rust,ignore
{{#include ../../examples/ch19-send-sync/src/lib.rs:mutex_guard}}
```

This is the concrete payoff for Chapter 1's "data race" row, and it's
worth being explicit about the exact mechanism: there is **no way to read
or write the `i32` inside a `Mutex<i32>` without calling `.lock()`**. The
lock isn't a separate variable you're expected to remember to take before
touching the data — the data lives *inside* the lock, and `.lock()` is the
only door in.

Compare to C's `pthread_mutex_t mu; int counter;`: `counter` is an
ordinary variable, and `mu` is a completely separate one. Nothing in the
type system connects them. Any function, anywhere in a large codebase,
can read or write `counter` directly, correctly-locked call sites and
buggy unlocked ones looking identical at the call site — the association
between "this lock" and "this data" is a comment, a naming convention, a
code review discipline. Never something the compiler checks. In Rust,
the mistake "touched the data without taking the lock" has no syntax that
type-checks — you cannot express it by accident.

`RwLock<T>` is the same idea with the C reader/writer-lock split
(`RwLock::read()`/`RwLock::write()` vs. `pthread_rwlock_rdlock`/`wrlock`) —
same guarantee, multiple simultaneous readers *or* one writer, never both.

## Exercise

**Exercise 19.1**, in
[`exercises/ch19-send-sync/ex01-shared-counter/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch19-send-sync/ex01-shared-counter),
has you build a `SharedCounter` type that hides its `Mutex` entirely
behind `add`/`get` methods — no accessor ever exposes the raw lock. See
its `README.md`.

Next: [Chapter 20 — Channels & Message Passing](20-channels.md).
