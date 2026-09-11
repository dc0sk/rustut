<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Toolchain & Cargo vs. Makefiles

## The toolchain

`rustup` is the equivalent of a version manager for `rustc` (the compiler)
plus its standard components. Where you might juggle several GCC/Clang
versions with `update-alternatives` or a container per target, `rustup`
does it per-project with a single file:

```toml
{{#include ../../rust-toolchain.toml}}
```

Any `cargo`/`rustc` invocation inside this repository transparently uses
exactly that channel — no "works on my machine" from a stray system
compiler.

## Cargo replaces your Makefile, your package manager, and your linker
## invocation, in one file

| You reach for in C | Cargo equivalent |
|---|---|
| `make` / `CMakeLists.txt` | `cargo build` |
| `make test` (if you wired one up) | `cargo test` |
| `gcc -c foo.c -o foo.o && ...link...` | handled by `cargo build`, transitively, for every dependency |
| vendoring a `.tar.gz` into `third_party/`, or apt/vcpkg | `[dependencies]` in `Cargo.toml`, fetched from crates.io |
| pinning a library version by hand, hoping it never silently changes | `Cargo.lock` — the exact resolved version of every dependency, transitively, committed to the repo |
| `doxygen Doxyfile` | `cargo doc --open` |
| `clang-format` | `cargo fmt` |
| `clang-tidy` / `cppcheck` | `cargo clippy` |
| `-O0` vs `-O2` | `cargo build` (dev profile) vs `cargo build --release` |

The single-file-does-everything design isn't an accident: it's what makes
"give this exercise to a coding agent" work at all. There's no build
system to reverse-engineer — `cargo test -p <crate>` is the same command
whether the crate has zero dependencies or twenty.

> **C engineer's mental model.** `Cargo.toml` is simultaneously your
> Makefile's dependency list, your linker's `-l`/`-L` flags, and your
> `configure.ac` — but declarative, and resolved transitively by Cargo
> instead of by you tracking include paths by hand.

## The manifest, for real

This is a real, working `Cargo.toml` from this book's own example
crates — not a paraphrase:

```toml
{{#include ../../examples/ch02-toolchain-and-cargo/Cargo.toml:manifest}}
```

`edition.workspace = true` and `license.workspace = true` **inherit** those
fields from the workspace root's `Cargo.toml` instead of repeating them in
every crate — the Rust equivalent of a shared `common.mk` your per-target
Makefiles `include`. `[lints] workspace = true` does the same for lint
configuration: one place decides "what counts as a warning," every crate
inherits it.

## Cargo commands you'll use constantly

- `cargo new <name>` / `cargo init` — scaffold a new binary or library crate.
- `cargo build` — compile (dev profile: fast to compile, unoptimized,
  includes debug info — think `-O0 -g`).
- `cargo build --release` — optimized build (`-O2`-ish, no debug assertions).
- `cargo run` — build and run the crate's binary.
- `cargo test` — run all `#[test]` functions, integration tests under
  `tests/`, and doc-tests.
- `cargo check` — type-check without producing a binary; much faster than
  `cargo build` for a tight edit-check loop, the way `gcc -fsyntax-only`
  is faster than a full compile+link.
- `cargo clippy` — Rust's linter, far more opinionated than the compiler's
  own warnings; this book treats `cargo clippy -- -D warnings` as a hard
  gate, not a suggestion.
- `cargo fmt` — canonical formatting, no bikeshedding, no `.clang-format`
  to argue about.
- `cargo doc --open` — generate and open API documentation from doc
  comments (`///`), including *running* the code in fenced examples as
  tests:

```rust,ignore
{{#include ../../examples/ch02-toolchain-and-cargo/src/lib.rs:doc_comment}}
```

## `Cargo.lock`: the pin you don't have to maintain by hand

`Cargo.toml` says "I need `serde`, version `^1.0`" — a *range*. `Cargo.lock`
records the *exact* version (and every transitive dependency's exact
version) that was actually resolved and built against, the same way you
might vendor an exact tarball into `third_party/` to stop a library from
changing out from under you — except Cargo generates and maintains this
file for you, and it's meant to be committed for anything that produces a
binary (which is every crate in this repository).

## Multi-crate composition: workspaces and path dependencies

A C project splits into multiple translation units linked into one binary,
usually coordinated by one Makefile that knows about all of them. Rust
splits into multiple **crates**, coordinated by a **workspace** — this
repository's root `Cargo.toml` is exactly that: a `[workspace]` with a
`members` list of glob patterns, so every example, exercise, and solution
crate is built and tested with one shared `Cargo.lock` and one shared lint
configuration, without needing to be published anywhere.

Two crates in the same workspace (or even outside one) can depend on each
other directly by filesystem path:

```toml
[dependencies]
some-helper-crate = { path = "../some-helper-crate" }
```

This is the direct analogue of `#include "../common/foo.h"` plus adding
`../common/foo.o` to your link line — except Cargo handles the "and
recompile it if it changed, and make its public items visible" part for
you.

## Exercise

**Exercise 2.1**, in
[`exercises/ch02-toolchain-and-cargo/ex01-wire-a-dependency/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch02-toolchain-and-cargo/ex01-wire-a-dependency),
is deliberately broken: its `src/lib.rs` already calls a finished sibling
crate, but `Cargo.toml` never declares the dependency, so it won't even
compile yet. Fix the manifest, not the code — see the exercise's
`README.md`.

Next: [Chapter 3 — Crate & Project Anatomy](03-crate-anatomy.md).
