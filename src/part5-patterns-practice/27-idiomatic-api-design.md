<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Idiomatic API Design: Builder, Newtype & Typestate

## Builder: making required fields actually required

C's usual options for "construct a complex value step by step" are a
giant struct literal with designated initializers (nothing stops you
from forgetting a field — it just silently zero-initializes) or a
`memset` + "set the fields you care about" convention, with no compiler
help distinguishing *intentionally* left at its default from *forgotten*.

```rust,ignore
{{#include ../../examples/ch27-idiomatic-api-design/src/lib.rs:builder}}
```

Each builder method consumes `self` and returns `Self`, so calls chain;
`build()` is where a genuinely required field (`url`) is finally checked,
returning `Result` if it's missing. This is the simplest correct design —
you *can* go further and make a missing required field a compile error
(a separate builder type per "field set" state), but that's real added
complexity, and simple-and-correct beats clever-and-obscure for most APIs.
When you do want a compile-time guarantee, that's exactly what typestate
(below) is for.

## Newtype: a distinct type, not just an alias

```rust,ignore
{{#include ../../examples/ch27-idiomatic-api-design/src/lib.rs:newtype}}
```

C's `typedef double Meters; typedef double Seconds;` is *only* an
alias — the compiler sees `double` either way, so a `Seconds` value
passes silently into a parameter typed `Meters`, with zero complaint.
`struct Meters(f64)` and `struct Seconds(f64)` are, to Rust's compiler,
completely different types. Swapping them isn't a style mistake caught in
review — it doesn't compile:

```text
error[E0308]: mismatched types
  |
  |     speed(distance, time);
  |           ^^^^^^^^ expected `Meters`, found `Seconds`
```

> **C engineer's mental model.** A newtype costs nothing at runtime (it's
> `#[repr(transparent)]`-eligible — the same bits as the wrapped value)
> and buys you everything a `typedef` promises but never delivers: actual
> type-level separation between values that happen to share a
> representation.

## Typestate: invalid transitions become compile errors

This is the most valuable pattern in the chapter. Instead of a runtime
flag checked (or forgotten) at the top of every method, protocol state
lives in the *type itself*:

```rust,ignore
{{#include ../../examples/ch27-idiomatic-api-design/src/lib.rs:typestate}}
```

`connect()` consumes the `Connection<Disconnected>` and returns a
`Connection<Connected>` — the old value is gone, not just mutated in
place, so there's no lingering handle still typed `Disconnected` that a
caller could mistakenly call `.send()` on. And `.send()` doesn't exist at
all on `Connection<Disconnected>` — not a method that checks a flag and
panics, a method that isn't there:

```text
error[E0599]: no method named `send` found for struct `Connection<Disconnected>`
```

Compare to the C shape of this problem: a `struct conn { bool
connected; ... }` where every function that touches the socket opens with
`if (!conn->connected) return -EINVAL;` — a runtime check that exists
purely because the compiler has no way to know, at the call site, whether
`connect()` already ran. Typestate moves that entire class of bug from
"caught by a test, if you wrote one" to "does not compile."

## Exercise

**Exercise 27.1**, in
[`exercises/ch27-idiomatic-api-design/ex01-typestate-file/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch27-idiomatic-api-design/ex01-typestate-file),
gives you a runtime-checked, panic-on-misuse `NaiveFileHandle` and asks
you to redesign it as a typestate `FileHandle<State>`. It's genuinely
open-ended (there's no single correct internal representation), so it's
graded differently from most exercises so far: a `tests/api_shape.rs`
mechanically checks that your public API has the required shape, and the
README's Design Rubric lists what a reviewer (human or agent) should
check by hand. See its `README.md`.

Next: [Chapter 28 — Design Patterns in Rust vs. C Idioms](28-design-patterns.md).
