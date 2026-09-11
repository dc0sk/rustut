<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Structs & Enums

## Three kinds of struct

```rust,ignore
{{#include ../../examples/ch08-structs-and-enums/src/lib.rs:struct_kinds}}
```

A named-field struct (`Point`) is the everyday case and reads exactly like
its C counterpart, minus the trailing semicolon after the closing brace.
A tuple struct (`Meters`) wraps a single value in a genuinely distinct
type — unlike `typedef double Meters;` in C, which is purely a naming
convenience the compiler discards immediately, `Meters(f64)` and a bare
`f64` are *not* interchangeable to Rust's type checker. A unit struct
(`Origin`) has no fields and no runtime size at all — a marker type with
no C equivalent, since C has no concept of a zero-sized type.

## `impl` blocks: associated functions vs. methods

```rust,ignore
{{#include ../../examples/ch08-structs-and-enums/src/lib.rs:impl_block}}
```

`Point::new(x, y)` is an **associated function** — it doesn't take `self`,
so you call it on the type itself, the way you'd call a C constructor
function like `point_create(x, y)`, except namespaced under `Point`
instead of living in the same flat global symbol table as everything
else in the program. `point.distance_from_origin()` is a **method** — it
takes `&self`, so it's called on a value.

> **C engineer's mental model.** There's no hidden vtable dispatch here
> (that's Chapter 13's `dyn Trait`). A method call on a concrete type like
> `Point` compiles to exactly the same kind of direct call as
> `point_distance_from_origin(&point)` would in C — `self` is just an
> implicit first argument.

## `#[derive(...)]`: the end of hand-written boilerplate

Every struct above carries `#[derive(Debug, Clone, Copy, PartialEq)]`.
Each one replaces a function you'd otherwise write by hand in C:

| Derive | Replaces (in C, by hand) |
|---|---|
| `Debug` | a `print_point(const struct Point *p)` you write and keep in sync |
| `Clone`/`Copy` | `memcpy`, or trusting the struct assignment `=` to do the right thing (it does, for a POD struct — but `Copy` makes "this type is safe to bitwise-duplicate" an explicit, checked property instead of an assumption) |
| `PartialEq` | a `points_equal(const struct Point *a, const struct Point *b)` comparing every field, that someone forgets to update when a field is added |

`derive` doesn't generate a runtime cost you didn't already have — it
generates exactly the function you'd have written, mechanically, and
keeps it in sync with the struct's fields forever.

## Enums: tagged unions, done right

```rust,ignore
{{#include ../../examples/ch08-structs-and-enums/src/lib.rs:shape_enum}}
```

In C, a shape that can be a circle, a rectangle, or a triangle is a tagged
union you build yourself:

```c
enum ShapeTag { CIRCLE, RECTANGLE, TRIANGLE };
struct Shape {
    enum ShapeTag tag;
    union {
        double radius;
        struct { double width, height; } rectangle;
        struct { double base, height; } triangle;
    } payload;
};
```

Nothing in that code stops you from reading `payload.rectangle` while
`tag == CIRCLE` — that's undefined behavior, not a caught bug, and it's a
real, recurring class of incident in C codebases that rely on this
pattern. Rust's `enum` makes it structurally impossible: each variant
carries exactly its own payload, and the only way to read one back out is
a `match` (Chapter 9) the compiler forces you to handle exhaustively —
there's no way to accidentally read a `Rectangle`'s fields while holding a
`Circle`, because the language doesn't let you name fields that don't
belong to the variant you're looking at.

One more difference worth flagging now: this `enum`'s in-memory layout
(`repr(Rust)`, the default) is *unspecified on purpose* — the compiler is
free to reorder fields and choose a tag representation to minimize size,
unlike a C `struct`, whose layout the language itself guarantees. When you
actually need C's layout guarantees — talking to a C library over FFI —
`#[repr(C)]` exists for exactly that. Chapter 26 covers it.

## Exercise

**Exercise 8.1**, in
[`exercises/ch08-structs-and-enums/ex01-shapes-and-vehicles/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch08-structs-and-enums/ex01-shapes-and-vehicles),
has you implement a struct's `impl` block and an enum's variant-dependent
logic. See its `README.md`.

Next: [Chapter 9 — Pattern Matching](09-pattern-matching.md).
