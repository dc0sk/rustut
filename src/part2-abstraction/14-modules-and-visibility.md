<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Modules, Crates & Visibility

Chapter 3 gave you a light preview of modules — enough to read and write
straightforward code. This chapter is the full treatment: nested module
trees, the finer-grained visibility levels between "private" and
"everywhere," `use` in depth, and workspace organization as a design
question rather than just Cargo mechanics.

## Nested modules and the filesystem

A module can have child modules, and (since the 2018 edition) you don't
need a `mod.rs` file to do it. A module named `network` can be either a
single `network.rs` file, or — the moment it needs children — a
`network.rs` file *alongside* a `network/` directory holding its
submodules:

```rust,ignore
{{#include ../../examples/ch14-modules-and-visibility/src/lib.rs:module_tree}}
```

`network.rs` declares its own child:

```rust,ignore
{{#include ../../examples/ch14-modules-and-visibility/src/network.rs:nested_and_visibility}}
```

...which lives at `network/http.rs`:

```rust,ignore
{{#include ../../examples/ch14-modules-and-visibility/src/network/http.rs:pub_super}}
```

## `pub(super)` and `pub(in path)`: visibility between "private" and "everywhere"

Chapter 3 covered private (default), `pub(crate)`, and `pub`. There's a
finer level: `pub(super)` makes an item visible to the module's **parent**
specifically — not the whole crate, not "everywhere," just one specific
ancestor. `parse_status_line` above is `pub(super)`: `network` (its
parent) can call it directly, as `Connection::parse_status` does, but the
crate root cannot reach it at all — a doc-tested `compile_fail` example
proves it:

```rust,ignore
{{#include ../../examples/ch14-modules-and-visibility/src/lib.rs:pub_super_boundary}}
```

`pub(in some::path)` generalizes this further — visible to one
*named* ancestor module, not necessarily the immediate parent. There is no
real C analogue for this granularity: `static` at file scope is
all-or-nothing, with nothing between "this translation unit only" and
"every translation unit that links against it."

## `use`, glob imports, and re-exports

Beyond the simple `use a::b::C;` Chapter 3 showed:

- `use a::b::{C, D};` imports several items from the same path at once.
- `use a::b::C as D;` renames on import — handy when two dependencies
  export a type with the same name.
- `use a::b::*;` (a **glob import**) pulls in everything public at that
  path. This is usually discouraged in ordinary code, for the same reason
  an unqualified `#include` of a huge header full of macros is risky in
  C: you can't see at the call site where a name came from, and a later
  addition to the glob-imported module can silently start shadowing
  something. The one place it's idiomatic is a **prelude** — a module a
  crate author curates specifically to be glob-imported:

```rust,ignore
{{#include ../../examples/ch14-modules-and-visibility/src/lib.rs:prelude}}
```

Note what this re-export does *not* do: it doesn't grant any new access.
`Connection` and `is_success` were already fully `pub` through `network`;
`prelude` just offers a flatter, curated path to the same items — the
same relationship Chapter 3's single re-export example showed, just
demonstrated here alongside a module that's itself `pub` (so both the
"dig into the real module" and "use the flat prelude" paths work).

## Workspace organization as a design question

Chapter 2 covered the *mechanics* of a Cargo workspace. The *design*
question — when do you split one crate into several workspace members,
versus just adding more modules to one crate — has a direct C analogue:
it's the same judgment call as deciding whether a growing C project needs
a second static library, or just another `.c` file in the existing one.

Reach for a separate workspace member when:

- **A binary needs the logic, but shouldn't *be* the logic** — a `core`
  library crate holding the actual behavior, plus a thin `cli` binary
  crate that depends on it. This is exactly this book's own `examples/`
  layout: every guided example is its own crate specifically so the
  book's prose can `{{#include}}` real, independently-compiled files.
- **Two consumers need the same code but shouldn't need each other** — a
  shared crate two unrelated binaries both depend on, where neither binary
  should have to link against the other.
- **You want independent compile units for iteration speed** — changing
  one workspace member only forces a rebuild of that member and whatever
  depends on it, not everything.

Reach for "just add a module" when the code has exactly one consumer and
no reason yet to be built or tested independently — which is most code,
most of the time. Splitting too early costs you real ceremony (a new
`Cargo.toml`, a new visibility boundary to maintain) for no compile-time or
reuse benefit yet.

## Exercise

**Exercise 14.1**, in
[`exercises/ch14-modules-and-visibility/ex01-visibility-boundaries/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch14-modules-and-visibility/ex01-visibility-boundaries),
has you implement a `pub(super)`-scoped helper and add a `prelude`
re-export. See its `README.md`.

Next: [Chapter 15 — Closures & Iterators](15-closures-and-iterators.md).
