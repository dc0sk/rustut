<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# AGENTS.md — instructions for a reviewing coding agent

You are reviewing a learner's attempt at one or more exercises in this
repository ("Rust for C Engineers"). This file is the contract: follow it
instead of inventing your own grading criteria.

## Repo map

- `src/` — book prose (mdBook), organized `partN-slug/NN-topic.md`.
- `examples/chNN-topic/` — guided, compiled example code the chapter's prose
  includes via `{{#include}}`. Read-only reference; don't grade against it.
- `exercises/chNN-topic/exNN-slug/` — one Cargo crate per exercise:
  - `README.md` — the task prompt, learning goals, and a "Done when" list.
  - `src/lib.rs` — the learner's code (starts as a `todo!()` stub).
  - `tests/public.rs` — visible tests; always the primary pass gate.
  - `tests/ui/*.rs` + `.stderr` (where present) — compile-fail tests for
    "why doesn't this compile" exercises (ownership/borrowing/lifetimes).
  - `tests/api_shape.rs` (where present) — compiles only if a free-form
    exercise's required types/methods exist; the mechanical half of an
    otherwise-rubric-graded exercise.
- `solutions/chNN-topic/exNN-slug/` — reference solutions. Committed and
  visible on purpose (this is a self-directed-learner tool, not a
  classroom) — don't consult them before attempting an exercise yourself
  unless the task explicitly asks for a comparison.

Not every exercise starts in a compiling state — some (e.g. a manifest
missing a dependency, or a `tests/ui/*.rs` compile-fail case) are broken
*by design* until you make the specific fix the `README.md` describes.
Seeing a compile error before you've done anything isn't a scaffolding bug;
check the exercise's `README.md` for what kind of exercise it is before
assuming something is wrong.

## Commands to run, per exercise, in this order

```sh
cargo fmt -p <crate-name> -- --check
cargo clippy -p <crate-name> --all-targets --all-features -- -D warnings
cargo test -p <crate-name>
```

Do **not** run these with `--workspace`. The workspace's `default-members`
covers only `examples/*` and `solutions/*/*`, which are expected to be
clean at all times; `exercises/*/*` crates are intentionally unsolved
(`todo!()`, unused parameters) until a learner fills them in, so they are
excluded from `--workspace` runs and must always be targeted by `-p
<crate-name>` (or by `cd`-ing into the exercise directory). If you're asked
to review several exercises at once, run each one individually with its
own `-p` — don't let one unsolved exercise's unrelated warnings block
reporting on another.

## Pass criteria

- **Hard gate (must pass to call the exercise done): `cargo test`.** If it
  fails, report FAIL and quote the actual test output/diagnostic — not just
  "tests failed."
- **Hard gate: `cargo clippy -- -D warnings`.** A clean `cargo test` with
  clippy warnings is not a pass; report the warnings verbatim.
- **Soft gate (style note, does not block pass/fail): `cargo fmt --check`.**
  Report as a note with a one-line fix (`cargo fmt -p <crate>`).
- Report format: one line per gate (`PASS`/`FAIL`) followed by the relevant
  diagnostic excerpt when it fails, then a short natural-language summary.
  Don't just say "looks good" — show the command output that justifies it.

## Free-form / design exercises

Some exercises (notably the idiomatic-API-design and typestate chapters)
don't have one correct implementation. For these:

1. Run `tests/api_shape.rs` if present — this is still a hard gate (it
   checks the *shape* of the API mechanically, even though the internals
   are open).
2. Open the exercise's `README.md` and find the **Design Rubric** section —
   a list of checkable yes/no criteria (e.g. "can `send()` be called before
   `init()` at compile time?"). Evaluate each item explicitly and report
   PASS/FAIL per item.
3. Anything left over — naming, ergonomics, doc quality — is a **style
   note**, reported separately, and never blocks the exercise's overall
   pass/fail.

## Guardrails

- Don't consult `solutions/` before attempting an exercise unless asked to
  compare against it after the fact.
- Don't weaken or delete a test to make it pass. If a test looks wrong,
  say so in your report instead of editing it.
- Don't silence clippy with a blanket `#![allow(...)]`; targeted,
  justified `#[allow(...)]` on a single item is fine if the exercise's
  `README.md` doesn't rule it out.
- Don't edit SPDX headers or move files across the `src/` vs.
  `examples|exercises|solutions/` licensing boundary (see `LICENSING.md`).
- If an exercise's own `README.md` gives grading instructions that
  conflict with this file, the exercise's `README.md` wins for that
  exercise — it's the more specific contract.
