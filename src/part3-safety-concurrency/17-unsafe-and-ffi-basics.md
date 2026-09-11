<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Unsafe Rust & FFI Basics

Everything so far has been **safe Rust**: code the compiler fully checks
for the bug classes in Chapter 1's table. `unsafe` is the deliberate,
narrow escape hatch — and this chapter's first job is correcting the most
common misconception about it.

## What `unsafe` actually unlocks — exactly five things

`unsafe` does **not** disable the borrow checker. It does **not** disable
bounds checks on ordinary `[]` indexing. It does not turn off any other
analysis the compiler normally does. It unlocks exactly five operations,
and nothing else:

1. Dereference a raw pointer (`*const T` / `*mut T`).
2. Call an `unsafe fn` (including an FFI function).
3. Implement an `unsafe trait`.
4. Read or write a mutable `static`.
5. Access a `union` field.

```rust,ignore
{{#include ../../examples/ch17-unsafe-and-ffi-basics/src/lib.rs:raw_pointer}}
```

```rust,ignore
{{#include ../../examples/ch17-unsafe-and-ffi-basics/src/lib.rs:unsafe_fn}}
```

Proof that the borrow checker is still fully active inside `unsafe`:

```rust,compile_fail
fn main() {
    let mut x = 5;
    let r1 = &x;
    unsafe {
        let r2 = &mut x; // still an error — `unsafe` changed nothing here
        println!("{r1} {r2}");
    }
}
```

```text
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
 --> src/lib.rs:5:18
  |
4 |     let r1 = &x;
  |              -- immutable borrow occurs here
5 |         let r2 = &mut x; // still an error — `unsafe` changed nothing here
  |                  ^^^^^^ mutable borrow occurs here
6 |         println!("{r1} {r2}");
  |                    -- immutable borrow later used here
```

> **C engineer's mental model.** Don't think of `unsafe` as "C mode." All
> of C's rules about, say, not aliasing pointers in ways the optimizer
> assumes you won't are still just as real inside an `unsafe` block —
> `unsafe` only means "I am personally upholding the specific invariant
> these five operations need, because the compiler has no way to check
> it for you here."

## `static mut` and `union`: the other two

```rust,ignore
{{#include ../../examples/ch17-unsafe-and-ffi-basics/src/lib.rs:static_mut}}
```

A C global is readable and writable from any thread with zero compiler
involvement — a data race waiting to happen, invisibly. Rust makes every
touch of a mutable static an explicit, `unsafe`-marked, `// SAFETY:`
comment-carrying operation; Chapter 21 covers `AtomicU32` as the actual
tool you'd reach for in real code needing a shared counter.

```rust,ignore
{{#include ../../examples/ch17-unsafe-and-ffi-basics/src/lib.rs:union_field}}
```

A `union` is the direct Rust equivalent of C's manual tagged-union
pattern from Chapter 8 — and it has exactly the same risk C's does:
nothing stops you from reading the field that wasn't last written. Rust
just makes you write `unsafe` at the one place that risk actually lives,
instead of it being ambient risk anywhere the union is touched at all.

## The `// SAFETY:` comment convention

Every `unsafe` block in this book (and in almost every well-maintained
Rust codebase) carries a `// SAFETY:` comment immediately above it,
explaining *why* the operation is sound at this specific call site — not
restating what the operation does. Look back at the four examples above:
each one's `// SAFETY:` names the specific precondition being relied on.
This is a convention, not something `rustc` enforces, but it's the single
most load-bearing convention in unsafe Rust: it's what turns "I was
careful" into an auditable claim someone else (or you, in six months) can
actually check.

## A minimal, real FFI call

```rust,ignore
{{#include ../../examples/ch17-unsafe-and-ffi-basics/src/lib.rs:ffi_abs}}
```

Two edition-2024-specific details worth being precise about, both
verified against a real `rustc 1.98` build, not guessed:

- The `extern "C" { ... }` block itself is written `unsafe extern "C" {
  ... }` — declaring the block is now its own unsafe operation, separate
  from calling anything inside it.
- An individual item inside can be marked `safe fn` if you're personally
  vouching that it's safe to call with *any* input — libc's `abs` on a
  plain `i32` can't do anything unsound, so it's marked `safe fn` here and
  callable from ordinary code with no `unsafe` block at the call site.
  Leave off `safe` and the item is an ordinary `unsafe fn`, callable only
  from inside an `unsafe` block.

This is deliberately the smallest possible FFI example — one already-safe
libc function, no build script, no external crate. Chapter 26 covers the
real thing: wrapping a multi-function C library with `build.rs`, the `cc`
crate, and `bindgen`.

## Exercise

**Exercise 17.1**, in
[`exercises/ch17-unsafe-and-ffi-basics/ex01-checked-buffer/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch17-unsafe-and-ffi-basics/ex01-checked-buffer),
has you wrap `slice::get_unchecked` in a bounds check you write yourself —
the direct, concrete version of "`unsafe` doesn't disable bounds checks on
`[]`, it just means `get_unchecked` skips the one `[]` always does." See
its `README.md`.

Next: [Chapter 18 — Threads](18-threads.md).
