<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Borrowing & References

Chapter 5 showed that assigning or passing most values *moves* them —
whoever gets the value now owns it, and the original binding is gone.
That's often not what you want: a function that just wants to *look at*
a `String` shouldn't have to take ownership of it (or force you to
`.clone()` it just to keep your own copy). **References** are the
answer: a way to let code access a value without taking ownership of it.

## `&T` and `&mut T`

```rust,ignore
{{#include ../../examples/ch06-borrowing-and-references/src/lib.rs:sum_slice}}
```

`&[i32]` **borrows** the data — `sum_slice` can read it, but doesn't own
it, doesn't drop it, and the caller keeps using their `Vec`/array
afterward exactly as before. This is close to passing a C pointer, with
one difference that changes everything: the compiler tracks *what kind*
of access a reference grants.

- `&T` is a **shared, read-only** borrow. Any number of these can exist
  at once.
- `&mut T` is an **exclusive, read-write** borrow. While one exists,
  nothing else — not even another `&T` — may access the same value.

## The aliasing rule

> **Many readers, or one writer — never both, at the same time.**

```rust,ignore
{{#include ../../examples/ch06-borrowing-and-references/src/lib.rs:aliasing_ok}}
```

Two `&[i32]` borrows of the same `data`, alive together, are completely
fine — nothing can go wrong when everyone's only reading.

```rust,ignore
{{#include ../../examples/ch06-borrowing-and-references/src/lib.rs:aliasing_sequenced}}
```

A `&mut Vec<i32>` borrow (inside `push`) followed by a `&[i32]` borrow
(inside `sum_slice`) also compiles — because they don't overlap in
*time*. The compiler's borrow-checking is based on how long a borrow is
actually used (an analysis called **non-lexical lifetimes**), not on
textual nesting, so sequential access like this needs no extra ceremony.

What the compiler *does* reject is exactly the case that produces
undefined behavior in C — a write happening while a read through another
alias is still in flight:

```rust,ignore
{{#include ../../examples/ch06-borrowing-and-references/src/lib.rs:aliasing_violation}}
```

> **C engineer's mental model.** In C, nothing stops you from holding two
> `int *` to the same `int` and mutating through one while another
> function reads through the other — that's a data race if the two
> happen on different threads, or a much subtler bug (iterator/pointer
> invalidation) if they happen on the same thread, e.g. mutating a `Vec`
> through one alias while iterating it through another, which in C might
> silently read stale or reallocated memory. Rust's aliasing rule turns
> both failure modes into the *same* compile error, whether or not
> threads are involved — Chapter 19 revisits this rule specifically for
> the multi-threaded case.

## Slices: a fat pointer, not a convention

`&[i32]` and `&str` are **slices** — a `(pointer, length)` pair the
compiler treats as a single, inseparable unit (sometimes called a "fat
pointer"). Compare the C idiom for the same job:

```c
int sum(const int *data, size_t len);
```

Nothing connects `data` and `len` in that signature — a caller can pass a
`len` larger than what `data` actually points to, and the function has no
way to know. `data.len()` on a Rust slice, by contrast, can never disagree
with the slice's real bounds: there is no way to construct a `&[i32]`
whose reported length doesn't match its actual extent, so `.get()`
(Chapter 1) and iteration are bounds-safe by construction, not by
discipline.

## References as function parameters

Prefer `&str` over `String` and `&[T]` over `&Vec<T>` for a function
parameter whenever the function only needs to read the data — it lets
the function accept borrows from *any* owner (a `String`, a string
literal, a slice of a larger buffer) without forcing a move or a clone at
the call site. You'll see this pattern in almost every function signature
from here on.

## Looking ahead

Every reference in this chapter's examples had an *obvious* lifetime —
tied to a variable that was clearly still in scope. What happens when
that isn't obvious — when a function returns a reference, or a struct
wants to hold one? That's Chapter 7.

## Exercise

**Exercise 6.1**, in
[`exercises/ch06-borrowing-and-references/ex01-largest-without-cloning/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch06-borrowing-and-references/ex01-largest-without-cloning),
has you implement two small functions that borrow instead of clone. See
its `README.md`.

Next: [Chapter 7 — Lifetimes](07-lifetimes.md).
