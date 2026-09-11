<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 3.1 — module visibility

`src/inventory.rs` is complete: it defines a public `Item` type with a
private `format_sku` helper. But nothing in `src/lib.rs` (the crate root)
declares `inventory` as a module, and even once it's declared, `Item`
still lives at the path `inventory::Item`, not at the crate root — so
`tests/public.rs`, which does `use ch03_ex01_module_visibility::Item;`,
currently fails to compile.

## Task

Edit only `src/lib.rs`:

1. Declare the `inventory` module.
2. Re-export `Item` at the crate root (`pub use`) so it's reachable as
   `ch03_ex01_module_visibility::Item`.

Don't touch `src/inventory.rs` — `format_sku` must stay private; the
exercise is about wiring the module tree and its public surface, not about
loosening anyone's visibility.

## Done when

- `cargo test -p ch03-ex01-module-visibility` passes
- `cargo clippy -p ch03-ex01-module-visibility --all-targets --all-features -- -D warnings` is clean

## Learning goals

- A `mod` declaration, not a file's mere existence on disk, is what makes
  a `.rs` file part of a crate's module tree.
- `pub use` re-exports an item at a new path — the mechanism behind "the
  public API is flatter than the internal module structure," which real
  crates use constantly (`std::collections::HashMap` is itself a
  re-export).
- A private item (`format_sku`) can be used freely *inside* the module
  that defines it while staying completely invisible outside it — no
  header/implementation split needed to get that separation.
