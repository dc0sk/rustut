<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Memory Safety Deep Dive: Drop & RAII

Chapter 1 showed a `FileGuard` whose `Drop` impl closed a file on every
return path, no `goto cleanup` required. Chapter 5 showed that ownership
is precisely *whose* `Drop` responsibility a value is. This chapter is
where those two ideas get their full treatment: exactly when `Drop` runs,
in what order, under what conditions — and, just as importantly, what
`Drop` does *not* guarantee.

## Drop order: reverse of declaration

When a scope ends, its stack-local values drop in **reverse** declaration
order — the last one created is the first one destroyed:

```rust,ignore
{{#include ../../examples/ch16-memory-safety-raii/src/lib.rs:drop_order}}
```

`drop_order_demo` records `["third", "second", "first"]`. There's nothing
extra to remember here, and nothing to get backward: it's the same rule a
call stack itself unwinds by. Compare this to C, where the cleanup order
is whatever sequence of `free()`/`fclose()` calls you happened to type —
correct only if you typed them in the right order, and silently wrong
forever if you didn't.

> **C engineer's mental model.** You already know that a function's stack
> frame is torn down in one shot when it returns. `Drop` order is that
> same LIFO discipline, just applied per-value instead of per-frame — and
> unlike a hand-written cleanup sequence, it can't drift out of sync with
> the order things were declared in, because it isn't separately written
> at all.

## Drop runs through a panic, too

A C program that hits an uncaught signal, or `longjmp`s past a chunk of
code, skips every cleanup call written in the code it jumps over — the
cleanup was a *line of code*, and jumping past code means not running it.
Rust's `Drop` isn't a line of code you could jump past: it's invoked by
the runtime as it unwinds the stack, so it fires even while a panic is
propagating through the scope:

```rust,ignore
{{#include ../../examples/ch16-memory-safety-raii/src/lib.rs:drop_through_panic}}
```

`drop_runs_even_on_panic` panics immediately after creating `_guard` —
and the guard's `Drop` still records `"panicking-scope"` before the panic
continues unwinding past this function. This is a *structural* guarantee,
not a "we remembered to check" one.

## You can't call `.drop()` yourself

If you could call a destructor directly and then keep using the value
afterward, you'd have reintroduced exactly the double-free-shaped bug
`Drop` exists to prevent — the value would look "already released" to
whatever runs at end-of-scope, while your code kept touching it. Rust
rejects this at compile time:

```rust,ignore
{{#include ../../examples/ch16-memory-safety-raii/src/lib.rs:leak_caveat}}
```

(See `explicit_drop_is_a_compile_error`'s doc comment in that same file
for the `.drop()` compile error itself, captured from a real `rustc` run.)
`std::mem::drop` — the free function you call as plain `drop(value)` — is
not special syntax. It's an ordinary function that takes ownership of its
argument and does nothing else with it; the value's normal end-of-scope
`Drop` then fires immediately, because the function that now owns it is
about to return.

## RAII as a pattern, not just this one example

**RAII** (Resource Acquisition Is Initialization — the name is inherited
from C++, though Rust's compiler-enforced version is considerably
stricter) is the general pattern behind all three of the guards you've
now seen: a type's constructor acquires a resource, and its `Drop` impl
releases it, so "resource is held" and "guard object exists" become the
same fact, checked by the same machinery that checks everything else
about ownership. You'll see this same shape reused deliberately later in
this book — a mutex guard (Chapter 19) and a file/socket handle (Chapter
24) are both, structurally, the exact same idea as `FileGuard` from
Chapter 1.

## What this does *not* guarantee: leaks are still possible

Chapter 1 already flagged this, and it's worth restating precisely now
that you've seen `Drop` in depth: Rust's guarantee is against
**use-after-free** and **double-free** — never against **leaks**. A value
whose `Drop` never runs because it's never actually deallocated is not a
violation of anything the compiler checks. The clearest example is a
reference-counted cycle:

```rust,ignore
{{#include ../../examples/ch16-memory-safety-raii/src/lib.rs:leak_caveat}}
```

`leak_a_cycle` builds two `Rc<Node>`s that point at each other. Once both
returned handles are dropped, each node's *strong count* is still 1 (held
by the other node), so neither one's `Drop` ever runs — `rc_cycle_leaks_instead_of_dropping`
proves it: the shared log stays empty. No `unsafe`, no bug in the borrow
checker — just a data structure that keeps itself alive forever. This is
a real, safe-Rust memory leak, and `std::mem::forget` is the other
standard, deliberate way to produce one. Neither is common in everyday
code, but "safe Rust" was never a promise that leaks are impossible —
only that the two much nastier bug classes are.

## Exercise

**Exercise 16.1**, in
[`exercises/ch16-memory-safety-raii/ex01-guard-order/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch16-memory-safety-raii/ex01-guard-order),
has you implement a `Drop` impl and a nested-scope function, then prove to
yourself (via the test suite) that the drop order comes out exactly as
this chapter predicts. See its `README.md`.

Next: [Chapter 17 — Unsafe Rust & FFI Basics](17-unsafe-and-ffi-basics.md).
