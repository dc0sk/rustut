<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Serialization & Data Interchange

## The problem, in C

Saving a struct to disk or sending it over the wire in C usually means one
of two things: hand-writing a marshal/unmarshal function pair for every
struct (tedious, and the two halves can drift apart if one is edited and
the other isn't), or `memcpy`-ing the struct's raw bytes directly (fast,
but now you're at the mercy of struct padding, field order, and
endianness — and if you add a field or the compiler changes how it lays
out the struct, old saved data silently means something different, with
nothing at compile time to warn you). Neither approach gives the compiler
any way to check that what you serialize is what you'll get back out.

## `serde`: one derive, both directions

`serde` (+ a format crate like `serde_json`) generates both the
serializer and the deserializer from a single struct definition, so they
cannot drift apart from each other the way two hand-written functions can:

```rust,ignore
{{#include ../../examples/ch29-serialization/src/lib.rs:config_struct}}
```

```rust,ignore
{{#include ../../examples/ch29-serialization/src/lib.rs:round_trip}}
```

> **C engineer's mental model.** Think of `#[derive(Serialize,
> Deserialize)]` as generating the marshal/unmarshal pair you'd otherwise
> hand-write — except both halves come from the same source of truth (the
> struct definition itself), so a field you add is automatically part of
> both directions, and a version that doesn't have the field simply
> doesn't compile if the format doesn't have it either (or falls back to a
> default, if you say so — see below).

## Format choice

This chapter uses JSON (via `serde_json`) because it's human-readable and
easy to eyeball while learning. `serde`'s trait-based design means the
same `#[derive]` works with many other formats too — a compact binary
format like `bincode` or `postcard` is a drop-in swap of the format crate,
not a rewrite of your data types, when you need smaller messages or faster
encoding and don't need humans reading the wire format.

## Untrusted input is still `Result`, not `unwrap()`

```rust,ignore
{{#include ../../examples/ch29-serialization/src/lib.rs:untrusted_input}}
```

Deserializing a payload from a network peer or a file a user handed you is
exactly the "untrusted input" scenario every parser in this book has been
careful about — a malformed or malicious JSON blob becomes an ordinary
`Err` you handle, never a panic. Chapter 31 covers untrusted-input handling
as a general security posture; this is `serde`'s specific instance of the
same discipline.

## Exercise

**Exercise 29.1**, in
[`exercises/ch29-serialization/ex01-config-round-trip/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch29-serialization/ex01-config-round-trip),
has you implement a load/save pair around a complete, `#[serde(default)]`-annotated
struct. See its `README.md`.

Next: [Chapter 30 — Testing & Documentation](30-testing-and-documentation.md).
