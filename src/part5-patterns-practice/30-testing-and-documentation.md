<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Testing & Documentation

Every chapter's exercise in this book has already been using three of the
four testing styles Rust supports, without naming them. This chapter
names them, adds the fourth, and contrasts the whole ecosystem with what a
C project has to bolt on separately.

## Unit tests

An inline `#[cfg(test)] mod tests` block, compiled only when running
`cargo test`, sitting right next to the code it tests — this book's guided
examples have used this since Chapter 1:

```rust,ignore
{{#include ../../examples/ch30-testing-and-documentation/src/lib.rs:unit_test}}
```

`#[should_panic(expected = "...")]` asserts a *specific* panic happens —
not just any panic, but one whose message contains the given substring.
It's the mechanical way to test "this input is invalid and the function
correctly refuses it via panic," the same way you might test that an
`assert()` in C fires for a given input, except `cargo test` actually
runs and checks it rather than you eyeballing an assertion message once.

## Integration tests

A file under `tests/` is compiled as its own separate crate that can only
see your crate's `pub` items — exactly the relationship this book's
`tests/public.rs` convention has had with every exercise crate all along,
formalized here as its own category:

```rust,ignore
{{#include ../../examples/ch30-testing-and-documentation/tests/integration.rs:integration_test}}
```

## Doc-tests

A fenced code block inside a `///` doc comment is not just documentation —
`cargo test` compiles and runs it, so a doc example that no longer
compiles or produces the wrong answer **fails the build**:

```rust,ignore
{{#include ../../examples/ch30-testing-and-documentation/src/lib.rs:average}}
```

This book's `compile_fail` doctests (used since Chapter 5, for "why
doesn't this compile" demonstrations) are the same mechanism with one
attribute changed: instead of asserting the example runs and produces a
value, `compile_fail` asserts it fails to compile at all.

> **C engineer's mental model.** `cargo doc --open`'s output cannot lie
> about whether its examples still work, because `cargo test` runs every
> one of them. A Doxygen comment with a `\code` example has no such
> guarantee — nothing stops the example from silently rotting out of sync
> with the function it documents.

## Property tests

Unit tests check specific, hand-picked inputs. **Property-based testing**
(the `proptest` crate here) instead states an invariant and lets the
library generate hundreds of random inputs checking it holds for all of
them — closer to how you might reason about correctness, further from
"did I happen to think of this edge case":

```rust,ignore
{{#include ../../examples/ch30-testing-and-documentation/src/lib.rs:property_test}}
```

If this property test ever fails, `proptest` doesn't just report the
random input it found — it automatically **shrinks** the failing case down
to the smallest input that still reproduces the failure, so you debug a
minimal counterexample instead of whatever large random slice it first
stumbled on.

## What C bolts on separately

A C project reaching for this same set of guarantees typically needs: a
separate unit-test framework (Unity, Check, CMock, ...) with its own build
integration; a separate documentation generator (Doxygen) with no
enforced relationship to whether its examples actually compile or run; and,
for property-based testing, a separate library (Theft, or hand-rolled
fuzzing) that most C codebases simply don't have. In Rust, all four
categories above are `cargo test`, full stop.

## Exercise

**Exercise 30.1**, in
[`exercises/ch30-testing-and-documentation/ex01-clamp-range/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch30-testing-and-documentation/ex01-clamp-range),
has you implement a small function against a mix of unit tests, a
`#[should_panic]` case, and a property test already written for you. See
its `README.md`.

Next: [Chapter 31 — Security Best Practices](31-security-best-practices.md).
