<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 14.1 — visibility boundaries

`logging::log_line` is complete and calls a `pub(super)` helper,
`logging::time::format_timestamp`, that isn't implemented yet. Separately,
nothing at the crate root re-exports `log_line` through a `prelude` module.

## Task

1. In `src/logging/time.rs`, implement `format_timestamp` — format
   `seconds` (elapsed since midnight) as zero-padded `HH:MM:SS`. **Don't
   change its `pub(super)` visibility**: it should stay reachable from
   `logging` only, not from the crate root.
2. In `src/lib.rs`, add a `pub mod prelude` that re-exports
   `logging::log_line` so it's reachable as
   `ch14_ex01_visibility_boundaries::prelude::log_line`.

## Done when

- `cargo test -p ch14-ex01-visibility-boundaries` passes
- `cargo clippy -p ch14-ex01-visibility-boundaries --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `pub(super)` scopes visibility to a specific ancestor module, not just
  "crate" or "everywhere" — there's no C equivalent this precise.
- A `pub use` re-export's visibility depends on whether the re-exporting
  code can see the item, not on adding any new access — a `prelude` module
  is a curated, flattened view onto an already-public API surface.
