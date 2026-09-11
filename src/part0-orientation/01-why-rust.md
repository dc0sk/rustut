<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Why Rust for a C Engineer

You don't need to be convinced that memory bugs are real — you've shipped
firmware that worked in the lab and corrupted its own heap in the field
three weeks later. This chapter is not a sales pitch about "Rust is safe."
It's an inventory: which specific bug classes you've debugged in C map to
which specific Rust feature, so that for the rest of the book, when you hit
that feature, you already know *why* it exists.

## The bug classes, by name

| Bug class | What it looks like in C | What eliminates it in Rust | Where this book covers it |
|---|---|---|---|
| Buffer overrun | `arr[i]` with no bounds check; UB, not a crash | Bounds-checked indexing, `.get()` returning `Option` | This chapter, [Ch. 11](../part1-core-language/11-collections.md) |
| Use-after-free | A `free()`'d pointer read or written later | The borrow checker refuses to compile it | [Ch. 5](../part1-core-language/05-ownership.md), [Ch. 6](../part1-core-language/06-borrowing-and-references.md) |
| Double free | `free()` called twice on the same pointer | Ownership means only one place can ever call the deallocator | [Ch. 5](../part1-core-language/05-ownership.md), [Ch. 16](../part3-safety-concurrency/16-memory-safety-raii.md) |
| Dangling pointer | A pointer that outlives what it points to | Lifetimes are checked at compile time | [Ch. 7](../part1-core-language/07-lifetimes.md) |
| Uninitialized read | Reading a `malloc`'d struct before filling every field | The compiler rejects reading anything not fully initialized | [Ch. 4](../part1-core-language/04-variables-mutability-types.md) |
| Data race | Two threads touching the same memory, at least one writing, no synchronization | `Send`/`Sync` make "this type may cross a thread boundary" part of the type system | [Ch. 19](../part3-safety-concurrency/19-send-sync.md) |
| Forgotten cleanup | A `goto fail`/early `return` that skips a `free()`/`fclose()` | `Drop` runs automatically on every path out of scope | This chapter, [Ch. 16](../part3-safety-concurrency/16-memory-safety-raii.md) |
| Integer overflow | Silent wraparound (or UB, for signed overflow) | Panics in debug builds; explicit `wrapping_*`/`checked_*` methods in release | [Ch. 31](../part5-patterns-practice/31-security-best-practices.md) |

Every row is a bug you have personally spent hours on. Every row is also a
bug an entire subfield of static analysis, sanitizers (ASan, TSan, MSan),
and coding standards (MISRA C) exist to catch *some* fraction of, at
runtime or on a good day at compile time with enough tooling. Rust's pitch
is narrower and more mechanical than "safe language": a fixed set of rules,
enforced by the compiler, on every build, for free.

> **C engineer's mental model.** You already trust a compiler to catch a
> type mismatch (`int` assigned to a `struct Foo*`) at compile time instead
> of at runtime. Rust just extends the set of things the compiler checks to
> include "who owns this memory" and "how long is this reference valid."
> The stack and the heap work exactly like they do in C — nothing here
> is a new runtime model, it's a stricter static one.

## Two bugs, made concrete

### Buffer overrun

In C:

```c
int data[3] = {10, 20, 30};
int value = data[5]; // undefined behavior: could be garbage, could crash,
                      // could silently corrupt an adjacent variable
```

In Rust, indexing with `[]` still exists and still assumes you've checked —
but if you're wrong, it panics loudly instead of reading memory that isn't
yours. And `.get()` sidesteps even the panic:

```rust,ignore
{{#include ../../examples/ch01-why-rust/src/bin/bug_classes.rs:bounds_check}}
```

`data.get(5)` returns `None`. No undefined behavior is reachable — the
worst case is a controlled `Option` you're forced to handle, not a read
into whatever happened to be adjacent to `data` in memory.

### Forgotten cleanup

In C, every exit path between acquiring a resource and releasing it is a
place a bug can hide:

```c
FILE *f = fopen("scratch.tmp", "w");
if (something_failed()) {
    return -1; // leaked: fclose(f) never happens on this path
}
fclose(f);
return 0;
```

Real codebases solve this with `goto cleanup;` patterns precisely because
it's so easy to add a new early return and forget the corresponding
release. Rust's answer is `Drop`: a destructor that the compiler guarantees
runs when a value goes out of scope, on *every* path, including early
returns and even panics unwinding through the function:

```rust,ignore
{{#include ../../examples/ch01-why-rust/src/bin/bug_classes.rs:raii_guard}}
```

There is no `goto cleanup`. There is no path through `process` that skips
closing the file, because closing it isn't something the function body
does — it's something that happens when `_guard`'s scope ends, and the
compiler tracks every scope exit for you. This pattern is called **RAII**
(Resource Acquisition Is Initialization, a term inherited from C++) and it
comes up constantly for the rest of this book — not just for files, but
for locks (Ch. 19), for memory itself (Ch. 5), and for anything else that
needs a matched acquire/release pair.

## What Rust does *not* give you for free

To calibrate expectations before you invest in the rest of this book:

- **Logic bugs are still your problem.** Rust catches memory and
  concurrency bugs mechanically; it has no opinion on whether your
  algorithm is correct.
- **`unsafe` exists, and it means what it says.** Some things — talking to
  hardware, calling into a C library, implementing a data structure the
  borrow checker can't express — genuinely require opting out of the
  compiler's checks. Chapter 17 covers exactly what `unsafe` does and does
  not turn off.
- **Performance is comparable to C, not automatically better.** Rust's
  safety checks are almost entirely at compile time; the binary you get is
  not paying a bounds-check tax everywhere (the compiler elides checks it
  can prove are redundant) but it is not magic either.
- **The learning curve is real, and it front-loads.** The next six
  chapters (ownership through collections) are the steepest part of this
  book. Once the borrow checker's rules are in your muscle memory, most
  Rust code reads like straightforward, if occasionally verbose, C.

## Exercise

Work through **Exercise 1.1** in
[`exercises/ch01-why-rust/ex01-raii-vs-bounds-checking/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch01-why-rust/ex01-raii-vs-bounds-checking) —
its `README.md` has the full task. You'll implement the bounds-checked
lookup and the RAII guard from this chapter yourself. If you're working
with a coding agent, point it at the repository's `AGENTS.md` first.

Next: [Chapter 2 — Toolchain & Cargo vs. Makefiles](02-toolchain-and-cargo.md).
