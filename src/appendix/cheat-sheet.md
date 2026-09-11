<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Cheat Sheet

A fast-lookup reference for after you've finished the book. Short idioms
are shown as inline code; anything multi-line is a real, compiled,
tested snippet from `examples/appendix-cheat-sheet/` — same anti-rot
guarantee as every chapter.

## Bindings & types

- `let x = 5;` — immutable by default. `let mut x = 5;` for mutable.
- `const MAX: u32 = 100;` — a compile-time constant, no storage, inlined
  at every use site (Ch. 4).
- Integer types: `i8 i16 i32 i64 i128 isize` / `u8 u16 u32 u64 u128 usize`
  — width is always explicit, never platform-dependent like C's `int`.
- Floats: `f32`, `f64`. Text: `char` (a Unicode scalar value, 4 bytes),
  `&str`/`String` (UTF-8). Boolean: `bool`. The empty type: `()`.
- `x as i32` — an explicit, possibly-lossy cast. `x.try_into()` — a
  checked conversion returning `Result`, the one you usually want instead
  (Ch. 4, Ch. 31).

## Ownership & borrowing, the rules

1. Each value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. You may have many `&T` (shared) *or* one `&mut T` (exclusive) into a
   value at a time, never both — enforced at compile time (Ch. 5, 6).

## Structs, enums, pattern matching

```rust,ignore
{{#include ../../examples/appendix-cheat-sheet/src/lib.rs:struct_and_derive}}
```

```rust,ignore
{{#include ../../examples/appendix-cheat-sheet/src/lib.rs:enum_and_match}}
```

Common derives: `Debug` (`{:?}` formatting), `Clone`/`Copy`, `PartialEq`/
`Eq`, `Hash` (usable as a `HashMap` key), `Default` (Ch. 8).

## `Option` and `Result`

```rust,ignore
{{#include ../../examples/appendix-cheat-sheet/src/lib.rs:option_result_chain}}
```

Reach for by name: `.map()` (transform the `Some`/`Ok` value),
`.and_then()` (chain another fallible step), `.unwrap_or(default)`,
`.unwrap_or_else(|| ...)`, `.ok_or("msg")` (turn `Option` into `Result`),
`?` (propagate an `Err`/`None` out of the current function) (Ch. 10).

## Iterators

```rust,ignore
{{#include ../../examples/appendix-cheat-sheet/src/lib.rs:iterator_chain}}
```

The core combinators: `.map()`, `.filter()`, `.filter_map()`, `.fold()`,
`.sum()`, `.collect()` (into `Vec`, `HashMap`, `String`, ...), `.zip()`,
`.enumerate()`, `.take()`/`.skip()`, `.rev()` (Ch. 15).

## Error types

```rust,ignore
{{#include ../../examples/appendix-cheat-sheet/src/lib.rs:error_enum}}
```

Rule of thumb: `thiserror` for a library's own error enum (Ch. 23), a
single `anyhow::Result<T>` (with `.context("...")`) at the application/
`main` boundary where you don't need callers to match on a specific
variant (Ch. 23).

## Builder pattern

```rust,ignore
{{#include ../../examples/appendix-cheat-sheet/src/lib.rs:builder_skeleton}}
```

Consuming `self` (not `&mut self`) on each setter lets calls chain
(`Builder::new().url(..).timeout_ms(..).build()`) without an intermediate
variable (Ch. 27).

## Concurrency quick reference

```rust,ignore
{{#include ../../examples/appendix-cheat-sheet/src/lib.rs:thread_and_mutex}}
```

| Need | Reach for | Chapter |
|---|---|---|
| Run work on another OS thread | `std::thread::spawn` | 18 |
| Share one mutable value across threads | `Arc<Mutex<T>>` (or `Arc<RwLock<T>>` for read-heavy access) | 19 |
| Hand ownership of values between threads | A channel (`std::sync::mpsc`) | 20 |
| A single counter/flag, lock-free | `std::sync::atomic::Atomic*` | 21 |
| Many concurrent I/O-bound tasks, not one-thread-each | `async`/`.await` + a runtime (`tokio`) | 22 |

## Cargo commands

| Command | Does |
|---|---|
| `cargo new`/`cargo init` | Scaffold a crate |
| `cargo build` / `cargo build --release` | Compile (debug / optimized) |
| `cargo run` | Build and run the binary |
| `cargo test` | Run unit, integration, and doc tests |
| `cargo check` | Type-check only, faster than a full build |
| `cargo clippy -- -D warnings` | Lint, treating warnings as errors |
| `cargo fmt` | Format |
| `cargo doc --open` | Build and view API docs |
| `cargo audit` | Check `Cargo.lock` against known vulnerabilities |

Next: this closes the Appendix. See [Further Reading](further-reading.md)
for where to go from here.
