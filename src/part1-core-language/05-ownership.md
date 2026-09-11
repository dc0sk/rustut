<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Ownership & Move Semantics

This is the chapter Chapter 1 promised: the mechanism behind "the borrow
checker refuses to compile a use-after-free." Take your time here — once
ownership is muscle memory, most of the rest of the language falls out of
it.

## The three rules

1. Every value has exactly one **owner** at any given time.
2. When the owner goes out of scope, the value is dropped (Ch. 1's RAII —
   ownership is precisely *whose* `Drop` responsibility a value is).
3. Ownership can be **transferred** ("moved") — assignment, passing by
   value, and returning by value all move ownership rather than copying
   or aliasing by default.

Rule 3 is the one with no real C equivalent, so it's worth slowing down on.

## Move: not a copy, not aliasing

```rust,ignore
{{#include ../../examples/ch05-ownership/src/lib.rs:move}}
```

Three things are true about `let s2 = s1;` that are worth naming
precisely, because each rules out a wrong C-shaped mental model:

- It is **not** a C struct assignment — a C `struct Foo b = a;` bitwise-
  copies every field, producing two independent structs that can now
  diverge (and, if either field is a pointer the struct is supposed to
  own, two things that will each try to free it — a double free).
- It is **not** a raw pointer copy — `char *p2 = p1;` gives you two
  pointers aliasing the same memory with no tracking of which one is
  "responsible" for it.
- It **is** ownership transfer: `s1`'s heap allocation is now `s2`'s, and
  the compiler statically forbids using `s1` again. There is exactly one
  owner at every point in the program — rule 1, mechanically enforced.

> **C engineer's mental model.** The *bits* that move are exactly what a C
> struct assignment would copy (for a `String`: a pointer, a length, a
> capacity — three words). The difference is entirely in what the
> compiler does afterward: C lets both copies live on, unaware they now
> share a heap allocation; Rust statically kills the old name so only one
> owner ever exists to free it.

This is the direct mechanism behind eliminating use-after-free and double-
free (Ch. 1's table): a double free is impossible when only one binding is
*allowed* to exist for a given owned value at a time, and the compiler —
not a convention, not a code reviewer — is what enforces "at a time."

## `Copy` types: when there's nothing exclusive to move

```rust,ignore
{{#include ../../examples/ch05-ownership/src/lib.rs:copy_vs_clone}}
```

Types like `i32`, `bool`, and `char` — anything that's just plain data with
no heap allocation and no responsibility to release anything — implement
the `Copy` trait, which changes assignment's behavior back to what a C
engineer expects: a bitwise copy, with both bindings remaining valid. This
isn't a special case bolted on top of move semantics; it's the same rule
(rule 3) applied to a type that has nothing exclusive to transfer in the
first place, so the compiler is free to duplicate it instead of moving it.

When you *do* want two independent owners of a heap-allocated value —
`String`, `Vec<T>`, and similar — `.clone()` is the explicit,
always-visible way to ask for it, playing the same role a C engineer would
reach for `strdup`/a manual deep-copy function to fill: a real second
allocation, deliberately requested, never accidental.

## Ownership flows through function calls too

```rust,ignore
{{#include ../../examples/ch05-ownership/src/lib.rs:ownership_through_functions}}
```

Passing a non-`Copy` value by value moves it into the function — the
caller's binding is gone, exactly as if they'd written `let x = value;`
inside the callee. Returning a value by value moves ownership back out.
There is no equivalent to a C function returning a `malloc`'d pointer with
an unenforced "and by the way, you now own this, remember to `free()` it"
comment — the *type itself* (owned `String`/`Vec<T>`/etc., not a
reference) is the ownership contract, and the compiler enforces both ends
of it.

## Exercise

**Exercise 5.1**, in
[`exercises/ch05-ownership/ex01-move-and-clone/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch05-ownership/ex01-move-and-clone),
has you write three small functions that between them cover taking
ownership, returning it back out, and borrowing instead of taking it — see
its `README.md`.

Next: [Chapter 6 — Borrowing & References](06-borrowing-and-references.md).
