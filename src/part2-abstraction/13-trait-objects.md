<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Trait Objects & Dynamic Dispatch

Chapter 12's generics are resolved entirely at compile time: call
`largest::<i32>` and `largest::<f64>`, and the compiler emits two separate
specialized functions, with no indirection at runtime. That's **static
dispatch**. Sometimes you genuinely don't know the concrete type until
runtime — a heterogeneous collection is the clearest case, and that's what
`dyn Trait` is for.

## `dyn Trait` is a real vtable, at runtime

```rust,ignore
{{#include ../../examples/ch13-trait-objects/src/lib.rs:trait_object}}
```

This is the honest, direct analogue of the C vtable pattern Chapter 12
contrasted itself with — except here it's real, not a metaphor. At
runtime, a `Box<dyn Describe>` genuinely **is** a pair of pointers: one to
the data, one to a vtable of function pointers for that concrete type's
trait implementation. Calling `.describe()` on it is one indirect call
through that vtable — exactly the cost (and exactly the mechanism) of
calling through a hand-rolled `struct { void *data; const struct
describe_vtable *vtable; }` in C.

`Vec<T>` can only ever hold one concrete `T` — `Vec<Circle>` cannot also
hold a `Square`. `Vec<Box<dyn Describe>>` can, because every element has
been boxed behind the same trait interface, discarding (erasing) its
specific concrete type:

```rust,ignore
{{#include ../../examples/ch13-trait-objects/src/lib.rs:trait_object}}
```

(Same include — `describe_all` is the function that iterates a
heterogeneous, boxed collection.)

## Static vs. dynamic dispatch: a real tradeoff, not a style choice

| | Static dispatch (generics, Ch. 12) | Dynamic dispatch (`dyn Trait`, this chapter) |
|---|---|---|
| Resolved | At compile time | At runtime, via vtable |
| Cost per call | None (direct call) | One indirect call |
| Code size | One copy per concrete type used (can grow the binary) | One shared implementation |
| Can mix concrete types in one collection? | No | Yes |
| Needs the concrete type known at compile time? | Yes | No |

This isn't a stylistic preference — it's a measurable tradeoff in both
speed and binary size, the same tradeoff you'd weigh in C between inlining
a function per call site versus calling through one shared function
pointer. Reach for generics by default; reach for `dyn Trait` specifically
when you need to store or pass around values whose concrete type varies at
runtime.

## Not every trait can become `dyn Trait`

A trait is **dyn compatible** (the older, still-common term is "object
safe") only if a vtable can actually be built for it — a trait with a
generic method can't, because the vtable would need an unbounded number of
function-pointer slots, one per type that method could ever be called
with:

```rust,ignore
{{#include ../../examples/ch13-trait-objects/src/lib.rs:object_safety}}
```

## Exercise

**Exercise 13.1**, in
[`exercises/ch13-trait-objects/ex01-shape-collection/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch13-trait-objects/ex01-shape-collection),
asks you to sum the area of a `Vec<Box<dyn HasArea>>` mixing two unrelated
shape types — dispatch through the trait object, don't downcast.

Next: [Chapter 14 — Modules, Crates & Visibility](14-modules-and-visibility.md).
