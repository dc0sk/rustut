<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 2.1 — wire a dependency

`src/lib.rs` in this crate already calls `stringy_lib::shout(...)` — the
sibling crate at `../stringy-lib` is finished and correct. But `Cargo.toml`
never told Cargo that dependency exists, so right now:

```sh
cargo test -p ch02-ex01-wire-a-dependency
# error[E0433]: failed to resolve: use of undeclared crate or module `stringy_lib`
```

## Task

Edit **`Cargo.toml` only** (don't touch `src/lib.rs`) and add a
`[dependencies]` section that points at `../stringy-lib` as a path
dependency, the same way you'd add another object file to a Makefile's link
line — except here Cargo resolves the whole dependency graph, and
`cargo doc`/`cargo test` follow it automatically.

## Done when

- `cargo test -p ch02-ex01-wire-a-dependency` passes
- `cargo clippy -p ch02-ex01-wire-a-dependency --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `Cargo.toml`'s `[dependencies]` table is the single source of truth for
  what a crate links against — no separate include-path flags, no manual
  link order.
- A `path = "..."` dependency is how you compose multiple crates in the
  same repository/workspace without publishing anything to crates.io.
