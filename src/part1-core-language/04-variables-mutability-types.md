<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Variables, Mutability & Types

## Immutable by default

```rust,ignore
{{#include ../../examples/ch04-variables-mutability-types/src/lib.rs:immutability}}
```

In C, every variable is mutable unless you write `const`. Rust inverts the
default: `let x = 5;` is immutable, and you have to opt in to mutation
with `let mut x = 5;`. This isn't stylistic — the borrow checker (Ch. 6)
leans on "is this binding declared `mut`?" as one of its core signals for
whether a reference can safely be written through.

## Shadowing is not mutation

```rust,ignore
{{#include ../../examples/ch04-variables-mutability-types/src/lib.rs:shadowing}}
```

A second `let spaces = ...` doesn't reuse or overwrite the first
`spaces` — it introduces a brand-new binding that happens to reuse the
name, and can even have a different type. This has no C equivalent:
redeclaring a variable of a different type in the same C scope is a
compile error. Shadowing is genuinely useful for a sequence of
transformations on a value where you don't want to invent a new name for
every step (`let raw = ...; let raw = raw.trim(); let count = raw.len();`).

`const` (as opposed to `let`) is closer to C's `#define`/`const`: it must
be a compile-time-evaluable value, has no fixed memory address guaranteed,
and by convention is `SCREAMING_SNAKE_CASE`.

## Integer types: explicit width, always

| Rust | Width | Closest C type |
|---|---|---|
| `i8`/`u8` | 8-bit | `int8_t`/`uint8_t` |
| `i16`/`u16` | 16-bit | `int16_t`/`uint16_t` |
| `i32`/`u32` | 32-bit | `int32_t`/`uint32_t` (also Rust's default integer type when unannotated) |
| `i64`/`u64` | 64-bit | `int64_t`/`uint64_t` |
| `isize`/`usize` | pointer-width | `intptr_t`/`size_t` — used for indexing, array lengths |

There is no bare `int` whose width depends on the platform — every integer
type states its exact width in its name, all the time. `usize` is the one
platform-dependent type, and it's used specifically where C would use
`size_t`: indices, lengths, and anything else that must be able to
address all of memory.

## No implicit narrowing

C will silently truncate on assignment: `uint8_t small = wide_value;`
compiles and just keeps the low 8 bits, bug or not. Rust rejects the
equivalent outright:

```rust,ignore
{{#include ../../examples/ch04-variables-mutability-types/src/lib.rs:narrowing}}
```

Two escape hatches, both explicit:

- **`as`** performs C-style truncation — but you have to write it, so
  it's visible in a code review and greppable in an audit.
- **`TryFrom`/`try_into`** performs a *checked* conversion, returning
  `Result` so you handle the out-of-range case instead of silently
  losing data.

> **C engineer's mental model.** Read every `as` you write in Rust the way
> you'd read an explicit `(uint8_t)` cast in C: a real, silent-truncation
> escape hatch you're choosing on purpose. The difference is that in Rust,
> *not* writing a cast for a narrowing conversion is a compile error
> instead of a silent bug — the cast is opt-in, not the default.

## Integer overflow

Arithmetic overflow (`u8::MAX + 1`) **panics** in a debug build and
**wraps** (two's-complement, like C) in a release build by default — a
deliberate compromise that catches most overflow bugs during development
without paying a runtime check in the optimized binary. Neither behavior
is usually what you actually want in production code that must handle
untrusted input; the explicit, portable-across-build-modes answer is
`checked_add`/`checked_sub`/`checked_mul` (returns `Option`),
`wrapping_add` (always wraps, explicitly), or `saturating_add` (clamps at
the type's bounds). Chapter 31 covers why relying on the debug/release
difference is itself a security footgun — for now, the exercise below has
you use the checked form.

## Arrays don't decay

```rust,ignore
{{#include ../../examples/ch04-variables-mutability-types/src/lib.rs:arrays}}
```

A C array parameter silently decays to a pointer, losing its length —
`sizeof(arr)` inside the function that received it gives you the pointer's
size, not the array's, which is a classic beginner trap. A Rust `[T; N]`'s
length `N` is part of its *type*: `.len()` works everywhere, decays to
nothing, and a function that wants a fixed-size array specifically (not a
slice — Ch. 6) can require exactly `[i32; 4]` in its signature.

## `if` is an expression

```rust,ignore
{{#include ../../examples/ch04-variables-mutability-types/src/lib.rs:if_expression}}
```

C's `if` is purely a control-flow statement; the closest it gets to
producing a value is the `?:` ternary, which is limited to simple
expressions. In Rust, `if`/`else` (and `match`, and blocks `{ ... }`
themselves) are expressions that evaluate to a value, as long as every
branch produces the same type — which is how `let status = if ... { .. }
else { .. };` above type-checks at all.

## Exercise

**Exercise 4.1**, in
[`exercises/ch04-variables-mutability-types/ex01-safe-narrowing/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch04-variables-mutability-types/ex01-safe-narrowing),
has you implement a checked narrowing conversion and a checked addition —
see its `README.md`.

Next: [Chapter 5 — Ownership & Move Semantics](05-ownership.md).
