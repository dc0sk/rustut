<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 27.1 — typestate file handle

`src/lib.rs` has a `NaiveFileHandle` that tracks open/closed state with a
`bool` and panics at runtime if you call `.read()` while closed — the
same shape as a C handle with an `is_open` flag checked by convention at
the top of every function.

## Task

Delete `NaiveFileHandle` and replace it with a **typestate** design: two
marker types (`Closed`, `Open`) and a single generic `FileHandle<State>`,
such that calling `.read()` on a closed handle is a **compile error**, not
a panic. `tests/api_shape.rs` fixes the exact required shape (types,
method names, signatures) — match it exactly, since that file must
compile as-is. `tests/public.rs` checks the behavior.

This is an open-ended design exercise: the *shape* is graded mechanically
(does it compile against `api_shape.rs`, do the behavioral tests pass),
but there's no single "correct" internal representation.

## Design Rubric

Check each of these explicitly if you're reviewing this exercise (a
coding agent should evaluate each item and report pass/fail per item, per
`AGENTS.md`'s free-form-exercise protocol):

- [ ] `FileHandle<Closed>` has no `read` method at all — not one that
      panics, one that does not exist.
- [ ] `.open()` consumes `self` (takes `self` by value, not `&self` or
      `&mut self`) — the old `FileHandle<Closed>` is gone after opening,
      not just mutated in place.
- [ ] `.close()` likewise consumes `self` and returns `FileHandle<Closed>`.
- [ ] No `bool`/`enum` runtime flag anywhere tracking open/closed state —
      the state lives entirely in the type parameter.

## Done when

- `cargo test -p ch27-ex01-typestate-file` passes (this also compiles and
  runs `tests/api_shape.rs`, since Cargo treats every file under `tests/`
  as its own integration test target)
- `cargo clippy -p ch27-ex01-typestate-file --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Typestate turns a runtime invariant ("don't call this before that") into
  a compile-time one, with zero runtime representation cost (the marker
  types are zero-sized).
- A mechanical "shape" test (`api_shape.rs`) can grade the *interface* of
  an open-ended design exercise even when the *implementation* is free.
