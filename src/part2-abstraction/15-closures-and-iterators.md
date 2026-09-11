<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Closures & Iterators

## Closures: a function pointer that remembers where it came from

A C function pointer is just an address — if the function needs access to
some state beyond its arguments, you thread a `void *context` parameter
through by hand, alongside the pointer, everywhere it's called:

```c
int (*callback)(int x, void *context);
```

A Rust **closure** is the compiler doing that "context struct" for you.
`|x| x + offset` captures `offset` from its environment, and the closure's
*type* encodes exactly what it captured and how:

```rust,ignore
{{#include ../../examples/ch15-closures-and-iterators/src/lib.rs:closure_vs_fn_pointer}}
```

There are three closure traits, differing in how much the closure is
allowed to do to what it captured:

- `Fn` — calls with just `&self`; can read captured state, can't mutate it.
- `FnMut` — calls with `&mut self`; can mutate captured state.
- `FnOnce` — calls with `self`, consuming the closure; can move captured
  state out, but only once.

A closure that mutates what it captured is `FnMut`, and calling it needs
`&mut` access to the closure itself — the same rule the borrow checker
applies to anything else:

```rust,ignore
{{#include ../../examples/ch15-closures-and-iterators/src/lib.rs:fnmut_example}}
```

## Iterators vs. hand-indexed loops

C gives you one real way to walk an array: an index variable and a
condition on it. Rust's `Iterator` trait and its combinators
(`.filter()`, `.map()`, `.fold()`, `.sum()`, `.collect()`, and dozens
more) let you build the same computation by composing named operations
instead of managing an index yourself. Here's the same function written
both ways:

```rust,ignore
{{#include ../../examples/ch15-closures-and-iterators/src/lib.rs:index_loop}}
```

```rust,ignore
{{#include ../../examples/ch15-closures-and-iterators/src/lib.rs:iterator_chain}}
```

Both are tested against each other and against known values in the
example crate — see `loop_and_iterator_versions_agree` in
[`examples/ch15-closures-and-iterators/src/lib.rs`](https://github.com/dc0sk/rustut/blob/main/examples/ch15-closures-and-iterators/src/lib.rs).

## The zero-cost abstraction claim — verify it yourself

Rust's iterator chains are often called a **zero-cost abstraction**: the
compiler is expected to compile the iterator version down to machine code
comparable to the hand-indexed loop, not to something slower wrapped in
extra function calls. This book won't paste an assembly listing and ask
you to trust it — an asm diff is exactly the kind of claim that should be
checked, not asserted. Check it yourself:

Paste both `sum_even_squares_loop` and `sum_even_squares_iter` (from
[`examples/ch15-closures-and-iterators/src/lib.rs`](https://github.com/dc0sk/rustut/blob/main/examples/ch15-closures-and-iterators/src/lib.rs))
into [godbolt.org](https://godbolt.org), pick the `rustc` compiler, and
compile with optimizations on (`-O` or `-C opt-level=3`) — this is the
standard, reliable way to compare generated machine code without needing
to fight `objdump`/symbol-mangling locally yourself. What you should find: the two versions compile
to comparable machine code — no allocation, no dynamic dispatch, no
per-element function-call overhead from the closures passed to `.filter()`
and `.map()`, because the compiler inlines and monomorphizes them away
(the same mechanism behind generics' cost — see Chapter 12).

## Exercise

**Exercise 15.1**, in
[`exercises/ch15-closures-and-iterators/ex01-sum-where/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch15-closures-and-iterators/ex01-sum-where),
has you implement a function taking two closures as parameters, built
using iterator combinators rather than a manual loop. See its `README.md`.

This closes Part 2. Next: [Chapter 16 — Memory Safety Deep Dive: Drop & RAII](../part3-safety-concurrency/16-memory-safety-raii.md).
