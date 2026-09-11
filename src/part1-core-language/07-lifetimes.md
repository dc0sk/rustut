<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Lifetimes

A **lifetime** is a compile-time-only fact about how long a reference is
valid, relative to other references and scopes in the same function. It
is not a runtime value, has no representation in the compiled binary, and
costs nothing when the program runs — it exists purely so the compiler
can reject a program where a reference might outlive the data it points
to, before that program ever ships.

## Most of the time, you never write one

```rust,ignore
{{#include ../../examples/ch07-lifetimes/src/lib.rs:elided}}
```

`first_word` takes one reference and returns one reference, and the
relationship is obvious: the output can't be valid for longer than the
input is. Rust's **lifetime elision rules** recognize this common shape
("exactly one reference parameter; the return type borrows") and fill in
the annotation for you. You've already written several functions in this
book that return borrowed data (Chapter 6's `largest`, for instance)
without ever writing `'a` — that's elision at work, not an omission.

## When elision isn't enough

```rust,ignore
{{#include ../../examples/ch07-lifetimes/src/lib.rs:explicit}}
```

Once there are *two* input references and the return type could
plausibly borrow from either one, the compiler won't guess — you have to
say so. `<'a>` declares a named lifetime; writing `a: &'a str, b: &'a
str) -> &'a str` says "the result borrows from whichever of `a` or `b` I
return, and is valid only as long as **both** `a` and `b` are." This is
the same information a comment in a C header might try (and often fail)
to convey ("caller must keep both arguments alive at least as long as the
returned pointer") — except the compiler both requires it and enforces
it at every call site.

## A struct holding a reference

```rust,ignore
{{#include ../../examples/ch07-lifetimes/src/lib.rs:struct_with_lifetime}}
```

Any struct with a reference field needs a lifetime parameter: `Parser<'a>`
cannot outlive the `&'a str` it points into. This is not a hidden extra
field or a runtime check — `'a` disappears entirely after compilation.
What it buys you is that the compiler now refuses to let a `Parser` be
used after the string it was built from goes away.

## The bug this catches

```rust,ignore
{{#include ../../examples/ch07-lifetimes/src/lib.rs:dangling}}
```

This is Chapter 1's "dangling pointer" row, made concrete: `shortest`
promises its result is valid as long as *both* inputs are, but the call
site only keeps one of them (`a`) alive long enough — `b` is dropped at
the end of its inner block while `result` (which might have borrowed from
`b`) is still going to be used afterward. In C, this exact shape — return
a pointer derived from a stack variable, then use it after that stack
frame (or block) is gone — is a use-after-free that might work by
accident in a debug build and corrupt memory in release. Here, it's
rejected before the program exists as a binary at all.

## `'static`

`'static` names the special lifetime of data that lives for the entire
program — string literals (`&'static str`) are the most common example,
since they're baked directly into the binary. You'll see `'static` show
up as a bound (`T: 'static`) far more often than as a concrete reference
lifetime, usually meaning "this type owns all its data, or only borrows
things that live forever" — Chapter 18 revisits this when it comes up for
spawning threads.

## Exercise

**Exercise 7.1**, in
[`exercises/ch07-lifetimes/ex01-parser-struct/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch07-lifetimes/ex01-parser-struct),
has you implement a small whitespace tokenizer built entirely on borrowed
`&str` slices — no allocation, no cloning. See its `README.md`.

Next: [Chapter 8 — Structs & Enums](08-structs-and-enums.md).
