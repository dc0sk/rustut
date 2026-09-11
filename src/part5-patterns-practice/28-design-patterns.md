<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Design Patterns in Rust vs. C Idioms

Chapters 16 and 19 already showed the **guard** pattern twice (a
`FileGuard`/`Drop` releasing a resource, a `MutexGuard` releasing a lock)
without naming it as a recurring idiom — it's worth naming explicitly
here: "wrap a resource in a type whose `Drop` releases it" is Rust's
general answer to a whole family of classic patterns built around
guaranteed cleanup.

## Strategy: a trait instead of a function-pointer field

```rust,ignore
{{#include ../../examples/ch28-design-patterns/src/lib.rs:strategy}}
```

C's usual tool for swappable behavior is a function pointer stored in a
struct field, often paired with a `void*` context argument threaded
through by hand so the callback has somewhere to keep its own state.
A trait bound (`fn compress_static<C: Compressor>(...)`, Ch. 12) gives you
the same swappability with **static** dispatch — no pointer, no null
check, monomorphized to a direct call, zero indirection. `&dyn Compressor`
(Ch. 13) gives you **dynamic** dispatch when the concrete strategy really
isn't known until runtime — still no raw pointer, no manual vtable, and
the one indirect call it does cost is explicit and visible in the type
(`dyn`), not implicit the way a C function pointer field is.

## Observer: consider a channel instead of a callback list

A direct port of the GoF Observer pattern looks like
`Vec<Box<dyn Fn(&Event)>>` — a list of callbacks, each invoked in-line
by the publisher, each capturing whatever state it needs by reference or
closure capture. It works, but often a **channel** (Ch. 20) is the more
idiomatic Rust replacement:

```rust,ignore
{{#include ../../examples/ch28-design-patterns/src/lib.rs:observer}}
```

"Observers" become receivers reading events off a queue, fully decoupled
from the publisher's call stack — the publisher never holds a `Vec` of
closures with unclear lifetimes, and an observer that's slow, panics, or
is dropped can't do anything to the publisher's own execution.

## Visitor: usually just a `match`

```rust,ignore
{{#include ../../examples/ch28-design-patterns/src/lib.rs:visitor}}
```

This is the chapter's real "aha." In a language without exhaustive
pattern matching, "do something different per variant of a closed set of
types" needs hand-rolled double dispatch: an `accept(Visitor*)` method on
every type, calling back into `visit_circle`/`visit_rectangle`, plus
discipline to keep every visitor implementation in sync as types are
added. An exhaustive `match` over an `enum` (Ch. 9) already guarantees
every variant is handled — which is the entire problem Visitor exists to
solve. You will very rarely need to hand-roll Visitor machinery in Rust;
reach for `match` first.

## GoF patterns that mostly disappear

- **Iterator** → built into the language via the `Iterator` trait
  (Ch. 15). You almost never hand-implement per-collection iteration
  machinery; you implement `next()` once and every combinator (`.map()`,
  `.filter()`, `.collect()`, ...) comes for free.
- **Simple Factory** → often just an associated function
  (`Connection::new(...)`) or an enum constructor — a dedicated Factory
  *type* is rarely needed when the language already gives you namespaced
  constructors and closed enums.

**Singleton** deserves its own paragraph rather than a bullet, since it's
worth seeing in full: `std::sync::OnceLock` (or `std::sync::LazyLock`)
gives you a lazily-initialized, thread-safe global, with no hand-rolled
double-checked locking — the classic C pattern of a `static` pointer, a
mutex, and a check-lock-check-again dance to initialize exactly once:

```rust,ignore
{{#include ../../examples/ch28-design-patterns/src/lib.rs:singleton}}
```

## Exercise

**Exercise 28.1**, in
[`exercises/ch28-design-patterns/ex01-logger-strategy/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch28-design-patterns/ex01-logger-strategy),
has you implement a `Logger<S: LogSink>` that formats a message and
forwards it to whichever sink it was built with — the strategy pattern,
statically dispatched. See its `README.md`.

Next: [Chapter 29 — Serialization & Data Interchange](29-serialization.md).
