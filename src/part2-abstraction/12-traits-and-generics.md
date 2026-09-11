<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Traits & Generics

Part 1 built the language's core data and control flow. Part 2 is about
**abstraction**: writing code once that works across many types, safely.
Chapters 12 and 13 cover the two ways Rust does this — static dispatch via
generics (this chapter) and dynamic dispatch via trait objects (next) —
and the tradeoff between them is one of the most concrete, measurable
decisions you'll make as a Rust programmer.

## Traits: a contract, checked at compile time

A **trait** is a set of methods a type promises to implement — the
closest C analogue is a `struct` of function pointers you'd hand-roll to
get polymorphism (a manual vtable):

```rust,ignore
{{#include ../../examples/ch12-traits-and-generics/src/lib.rs:trait_def}}
```

The critical difference from a hand-rolled C vtable: when you call
`some_circle.describe()`, the compiler already knows at compile time
exactly which `describe` implementation that is — for `Circle`, always
`Circle`'s. There's no function-pointer indirection here at all (that's
next chapter's `dyn Trait`). This chapter's trait usage compiles down to
an ordinary direct call, the same as if you'd written `circle_describe(&c)`
by hand.

> **C engineer's mental model.** A C vtable struct gives you runtime
> polymorphism by paying for an indirect call every time. A Rust trait,
> used the way this chapter uses it (as a bound on a generic function),
> gives you the *same abstraction at the source level* with *zero* of that
> runtime cost — the compiler generates a direct call per concrete type,
> the same as if you'd written the type-specific version by hand.

## Default methods

A trait method can have a body — every implementor gets it for free
unless they override it, something a C vtable struct can't express (every
function-pointer slot there must be filled in explicitly, even if most
implementations would do the same thing):

```rust,ignore
{{#include ../../examples/ch12-traits-and-generics/src/lib.rs:trait_def}}
```

(Same include as above — `describe_loudly` is the default method; look for
it in the trait definition.)

## Generics: write it once, for every type that qualifies

```rust,ignore
{{#include ../../examples/ch12-traits-and-generics/src/lib.rs:static_dispatch}}
```

C has three ways to write "this logic, for any type": duplicate the
function per type, write a macro, or take a `void*` and cast it back
inside the function (giving up type safety — nothing stops you from
calling it with the wrong kind of pointer, and the compiler won't catch
it). Rust's generics are none of these: `T: Describe` is a **compile-time
constraint** — the function simply does not compile for a `T` that isn't
`Describe`, and at compile time the compiler generates one specialized
copy of the function per concrete type it's actually called with, a
process called **monomorphization**. You get the ergonomics of writing it
once, the safety of the compiler checking every call, and the performance
of a type-specific function — because that's genuinely what gets compiled.

The classic example, comparing any two orderable, copyable values without
committing to a specific type up front:

```rust,ignore
{{#include ../../examples/ch12-traits-and-generics/src/lib.rs:generic_largest}}
```

`T: PartialOrd + Copy` reads as "any type that can be ordered and
copied" — try calling `largest` with a type that isn't `Copy` and the
error is a compile-time trait-bound failure, not a runtime crash from
misinterpreting a `void*`.

## Deriving traits (a callback to Chapter 8)

`#[derive(Debug, Clone, PartialEq)]` on a struct or enum generates trait
implementations for you at compile time — the direct alternative to
writing your own `print_struct`/`structs_equal` functions in C by hand,
and it can never drift out of sync with the struct's actual fields the
way a hand-written comparison function can after someone adds a field and
forgets to update it.

## Exercise

**Exercise 12.1**, in
[`exercises/ch12-traits-and-generics/ex01-generic-stats/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch12-traits-and-generics/ex01-generic-stats),
asks you to write a function generic over any type implementing a small
`Scored` trait — the test file exercises it with two unrelated types to
make sure your solution doesn't quietly assume anything beyond the trait
bound.

Next: [Chapter 13 — Trait Objects & Dynamic Dispatch](13-trait-objects.md).
