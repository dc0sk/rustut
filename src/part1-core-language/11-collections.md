<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Collections

## `Vec<T>`: a growable array, without the bookkeeping

A dynamic array in C means owning the bookkeeping yourself: a pointer, a
`len`, a `capacity`, and the discipline to call `realloc` (and update
`capacity`) before `len` would exceed it — and the classic bug is
forgetting one side of that: writing past `capacity` because `len` was
updated without checking, or leaking the old block because `realloc`'s
return value replaced the pointer without freeing on the failure path.

```rust,ignore
{{#include ../../examples/ch11-collections/src/lib.rs:vec_growth}}
```

`Vec::push` handles all of that internally. There's no `capacity`
variable for you to forget to update, because there's no `capacity`
variable in your code at all — `.capacity()` exists if you want to
*inspect* it, but nothing about correctness depends on you tracking it.

> **C engineer's mental model.** A `Vec<T>` is a `(pointer, length,
> capacity)` triple on the stack pointing at a heap allocation — the same
> shape you'd hand-write in C. The difference is that `push`/`pop`/index
> access are the *only* way to touch it, so the triple can never get out
> of sync with the allocation the way three independently-updated C
> variables can.

## `String` vs. `&str` vs. `char*`

C's `char*` is just a pointer to bytes; nothing about the type says
whether those bytes are valid text, in what encoding, or how long the
buffer actually is versus how long `strlen` will report before it walks
off the end looking for a `\0` that isn't there. `strcpy`/`gets`/`sprintf`
without a bound are exactly this ambiguity turned into a buffer overflow.

Rust splits the same "owned vs. borrowed" distinction Chapter 5
introduced for data in general into two string types: `String` (owned,
growable, heap-allocated) and `&str` (a borrowed view into UTF-8 bytes
someone else owns — a `String`, a `&'static str` literal, or a slice of
either). Both are **guaranteed valid UTF-8**, always, or the program
doesn't compile/panics constructing them from raw bytes — there is no
"just a pointer, hope the bytes are sensible."

```rust,ignore
{{#include ../../examples/ch11-collections/src/lib.rs:string_vs_str}}
```

One consequence of UTF-8: `.len()` counts *bytes*, not visible
characters, because a single character like `é` can take more than one
byte. `.chars().count()` counts Unicode scalar values instead. Mixing
these up is precisely the class of bug manual byte-indexing into a C
string causes on non-ASCII text — except Rust refuses to let a `String`
be indexed at an arbitrary byte offset at all (`s[3]` doesn't compile;
slicing at a non-character-boundary byte offset panics rather than
silently returning a corrupted partial character).

## `HashMap<K, V>`: no hand-rolled hash table

```rust,ignore
{{#include ../../examples/ch11-collections/src/lib.rs:hashmap}}
```

Where a C project reaches for a hand-written open-addressing or
chaining hash table (plus a hash function, plus resize logic), `HashMap`
is in the standard library, generic over any `K: Hash + Eq`, and
resizes itself the same way `Vec` does.

## Iterating any of them

`for x in &vec`, `for (k, v) in &map`, and `for c in s.chars()` all use
the same `Iterator` trait underneath — Chapter 15 covers iterators and
closures in depth; for now, treat `for` over a collection as the direct,
zero-cost replacement for a C `for (i = 0; i < len; i++)` loop, minus the
off-by-one class of bug that an explicit index invites.

## Bounds checking, once more

Chapter 1 opened with `.get()` returning `Option` instead of reading
out-of-bounds memory. `Vec` and slices share that guarantee — indexing
with `[]` panics (loudly, immediately, never silently) rather than
reading adjacent memory, and `.get()` sidesteps even the panic. Every
collection in this chapter inherits that property; it isn't a special
case for arrays.

## Exercise

**Exercise 11.1**, in
[`exercises/ch11-collections/ex01-word-count-and-dedup/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch11-collections/ex01-word-count-and-dedup),
has you build a word-frequency `HashMap` and an order-preserving dedup
using a `HashSet` alongside a `Vec`. See its `README.md`.

This closes Part 1 — the core language. You now have enough to read and
write straightforward Rust; Part 2 covers the abstraction tools (traits,
generics, closures) that make Rust code composable rather than just
correct.

Next: [Chapter 12 — Traits & Generics](../part2-abstraction/12-traits-and-generics.md).
