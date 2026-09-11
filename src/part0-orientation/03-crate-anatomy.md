<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Crate & Project Anatomy

## Crates: the unit of compilation

A **crate** is Rust's unit of compilation — the closest analogue is a
static or dynamic library (or an executable) as a whole, not a single
translation unit. Where C compiles each `.c` file separately and lets the
linker reconcile them, `rustc` compiles an entire crate as one unit,
which is *why* Rust can afford whole-crate type inference and monomorphized
generics (Chapter 12) without the "the linker can't see across translation
units" limitations C tooling works around with link-time optimization.

There are two crate kinds you'll create constantly:

- A **binary crate** has a `src/main.rs` with a `fn main()` and produces an
  executable — the Rust equivalent of the `.c` file with your `main()`.
- A **library crate** has a `src/lib.rs` (no `main()`) and produces
  something other crates depend on — the equivalent of a `.a`/`.so` plus
  its public header.

A single package can have both: one `src/lib.rs` plus one or more binaries
under `src/bin/`, each sharing the library's code — exactly the shape this
book's own `examples/` crates use, and the shape a CLI tool that's also
usable as a library (Chapter 23) takes.

## Modules: the tree that replaces `#include`

C's "what's visible where" is a purely textual, preprocessor-driven
story: `#include "foo.h"` pastes declarations in, and anything with
external linkage is fair game to any `.c` file that includes the right
header — visibility is a *convention* (`static`, leading underscores),
not something the compiler enforces across files.

Rust replaces both the textual-paste step and the convention with a real,
compiler-checked **module tree**. `mod warehouse;` in a crate root doesn't
paste text — it declares that `src/warehouse.rs` (or
`src/warehouse/mod.rs`) is part of this crate's module tree, and every
item in it is reachable at the path `warehouse::whatever`, subject to
visibility rules the compiler actually enforces:

```rust,ignore
{{#include ../../examples/ch03-crate-anatomy/src/lib.rs:module_tree}}
```

> **C engineer's mental model.** A C header advertises what a translation
> unit will let you link against, but says nothing enforceable about what
> stays internal — a non-`static` function is fair game from anywhere,
> header or no header. Rust's module system is the part of "public API"
> that C has always had to fake with comments and hope.

## Visibility: three levels the compiler actually checks

```rust,ignore
{{#include ../../examples/ch03-crate-anatomy/src/warehouse.rs:visibility}}
```

| Visibility | Reachable from | Closest C analogue |
|---|---|---|
| (nothing, default) | only the current module and its children | `static` at file scope — but scoped to a *module*, which can span multiple files |
| `pub(crate)` | anywhere in the same crate | no real equivalent — closest is "declared in an internal header nothing outside the project ships" |
| `pub` | anywhere the crate is used from | a symbol with external linkage, declared in a public header |

The default (no keyword) being *private* is deliberate and inverted from
C's default (external linkage unless you say `static`): in Rust, you have
to opt in to exposing something, so a crate's true public API is exactly
its `pub` items — nothing is accidentally exported because someone forgot
`static`.

## Re-exports: the public API doesn't have to mirror the module tree

`pub use warehouse::Crate;` at the crate root means callers write
`ch03_crate_anatomy_example::Crate`, never needing to know `Crate` is
really implemented in a `warehouse` submodule. This is how real crates
keep an internal module structure organized for the maintainers while
presenting a flat, curated public surface to everyone else —
`std::collections::HashMap`, for instance, is itself a re-export from a
deeper internal path.

## Workspaces, briefly revisited

Chapter 2 introduced workspaces as Cargo's multi-crate build coordinator.
The anatomy point worth adding here: a workspace member is still a fully
independent crate with its own `Cargo.toml`, `src/lib.rs`/`src/main.rs`,
and module tree — the workspace just gives them a shared `Cargo.lock` and
(in this repository) shared lint settings via `[lints] workspace = true`.
Nothing about modules or visibility changes because a crate happens to
live inside a workspace.

## Exercise

**Exercise 3.1**, in
[`exercises/ch03-crate-anatomy/ex01-module-visibility/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch03-crate-anatomy/ex01-module-visibility),
gives you a complete, correct `inventory.rs` that simply isn't wired into
the crate yet. See its `README.md`.

This closes Part 0. Part 1 starts where the real language begins: [Chapter
4 — Variables, Mutability & Types](../part1-core-language/04-variables-mutability-types.md).
