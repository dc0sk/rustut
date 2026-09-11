<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Atomics & Memory Ordering

This chapter is short on purpose: Rust's atomic memory model is, by
deliberate design, the **same** model as C11/C++11's
`stdatomic.h`/`<atomic>` — same operations, same ordering names, same
semantics. If you've used `atomic_fetch_add`, `atomic_compare_exchange_*`,
or reasoned about `memory_order_acquire`/`memory_order_release` in C,
almost none of that understanding needs to be relearned here. What
changes is the syntax, and the fact that Rust's type system tracks which
type is atomic for you, rather than trusting you to only ever touch a
`volatile`/`_Atomic`-qualified variable through the right functions.

## The types and operations

`std::sync::atomic` provides `AtomicBool`, `AtomicUsize`, `AtomicU32`,
`AtomicU64`, and friends, each wrapping a plain integer/boolean with
methods for indivisible read-modify-write operations:

```rust,ignore
{{#include ../../examples/ch21-atomics/src/lib.rs:counter}}
```

`fetch_add` is exactly `atomic_fetch_add` — one CPU instruction (or a
short lock-free instruction sequence) that increments and returns the
previous value, with no other thread able to observe a half-completed
increment.

## `Ordering`: the same five you already know

`Ordering::Relaxed`, `Acquire`, `Release`, `AcqRel`, `SeqCst` map directly
onto `memory_order_relaxed`, `_acquire`, `_release`, `_acq_rel`, `_seq_cst`.
The rule of thumb carries over unchanged too: reach for `Relaxed` when the
only thing that matters is the final value of the atomic itself (a
counter nobody else's logic depends on being "fresh" at a specific
moment); reach for `Acquire`/`Release` when one thread's write must become
visible to another thread *together with* other memory it touched before
that write — i.e. when the atomic is being used as a synchronization
point for other, non-atomic data:

```rust,ignore
{{#include ../../examples/ch21-atomics/src/lib.rs:guard}}
```

`compare_exchange` here is `atomic_compare_exchange_strong`: "if the
current value is `false`, set it to `true`, atomically; tell me whether
you won the race." This is the standard building block for "run this
exactly once, even under concurrent callers" — no mutex, no blocking,
just one atomic instruction.

## Atomic vs. `Mutex`: what each one actually protects

An atomic gives you exactly one indivisible value. A `Mutex` (Ch. 19)
gives you exclusive access to however much state you put inside it — one
field or twenty, as one invariant. The moment you need to update *more
than one* related piece of state consistently (a length and a pointer, a
balance and a transaction log), a single atomic can't express that
invariant — you'd need one atomic per field, updated in some order, with
no way to make the whole update appear atomic to an observer. That's
exactly the case a `Mutex` is for. Use an atomic when "one number,
updated by many threads" is the whole problem; reach for a `Mutex` the
moment it isn't.

## Exercise

**Exercise 21.1**, in
[`exercises/ch21-atomics/ex01-atomic-counter/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch21-atomics/ex01-atomic-counter),
has you implement a saturating counter using a `compare_exchange` retry
loop — the pattern that generalizes to almost any "read, check, maybe
update" atomic operation. See its `README.md`.

Next: [Chapter 22 — Async/Await & the Async Ecosystem](22-async-await.md).
