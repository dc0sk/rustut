<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 16.1 — guard order

## Task

1. Implement `Drop for RecordingGuard` so dropping a guard pushes its
   `name` onto the shared log.
2. Implement `nested_scopes`: bind a `RecordingGuard` named `"outer"` to
   `_outer`, then open a nested block that binds a second `RecordingGuard`
   named `"inner"` to `_inner`. Add no explicit cleanup calls — the guards
   dropping themselves, in the right order, purely from scope, is the
   entire point.

## Done when

- `cargo test -p ch16-ex01-guard-order` passes
- `cargo clippy -p ch16-ex01-guard-order --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Values drop in reverse declaration order, and a nested block's locals
  are always gone before the enclosing scope continues — there is no
  separate "cleanup step" to write or forget, the scope structure of the
  code *is* the cleanup order.
