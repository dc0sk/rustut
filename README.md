<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Rust for C Engineers

A deep, example-driven Rust tutorial for engineers who already think in C:
ownership vs. `malloc`/`free`, borrowing vs. raw pointers, traits vs.
vtables, fearless concurrency vs. `pthreads`, and how to build real PC
applications idiomatically.

The reader's assumed background (C, systems/embedded, EE or CS) is a lens
for analogies throughout the book — **not the destination**. This is a
general-purpose Rust tutorial (libraries, CLI tools, networked services),
not an embedded/`no_std` guide. See
[`src/introduction.md`](src/introduction.md) for the full pitch.

## Reading the book

The book is written for [mdBook](https://rust-lang.github.io/mdBook/).
Once pushed, it's built and deployed to GitHub Pages by
[`.github/workflows/book.yml`](.github/workflows/book.yml) on every push to
`main`. To read or build it locally:

```sh
cargo install mdbook
mdbook serve   # live-reloading local server, or:
mdbook build   # static site in ./book
```

## Repository structure

```
src/                  Book prose (mdBook chapters), CC-BY-SA-4.0
examples/chNN-topic/  Guided examples the chapters {{#include}} verbatim —
                      real, compiled, tested code, never a hand-typed
                      paraphrase that can drift out of sync with the prose
exercises/chNN-topic/exNN-slug/
                      One Cargo crate per exercise: a README with the task
                      and a "Done when" checklist, a stub to fill in, and
                      a test suite that mechanically decides pass/fail
solutions/chNN-topic/exNN-slug/
                      Reference solutions — committed and visible; this is
                      a self-directed-learner tool, not a classroom trying
                      to prevent copying. Try an exercise yourself first.
```

Every chapter follows this same four-piece shape (prose + guided example +
exercise + solution) — see [`src/introduction.md`](src/introduction.md)
for the full explanation, and any already-written chapter for a concrete
example of the pattern. Chapters are grouped into six parts (see
[`src/SUMMARY.md`](src/SUMMARY.md) for the full table of contents):
Orientation, Core Language, Abstraction, Safety & Concurrency, Building PC
Applications, and Patterns & Practice (ending in a capstone project).

## Working with a coding agent

If you're using a coding agent (Claude Code or similar) to work through
exercises, or to review your own or a learner's solutions, point it at
[`AGENTS.md`](AGENTS.md) first. It's the contract: exact commands to run,
what counts as a pass vs. a style note, and how to grade the handful of
exercises that are open-ended design tasks (graded via a rubric) rather
than pure test pass/fail.

## Building and testing the code

This is a single Cargo workspace. Exercise stubs are *intentionally*
unsolved (`todo!()`, or in a few cases a deliberately-broken manifest/API
shape) until you fill them in, so the workspace's `default-members` are
scoped to only `examples/*` and `solutions/*/*` — the parts that are
always expected to be clean:

```sh
cargo build                                                    # examples + solutions only
cargo clippy --all-targets --all-features -- -D warnings       # same scope, must be clean
cargo test                                                      # same scope, must pass
cargo fmt --all -- --check

# Work on one specific exercise (ignores default-members):
cargo test -p <crate-name>
cargo clippy -p <crate-name> --all-targets --all-features -- -D warnings
```

CI (`.github/workflows/exercises.yml` and `book.yml`) runs the same
checks, plus `reuse lint` (license compliance), `cargo audit` (dependency
vulnerability scanning), and an anti-rot check that every code sample in
the book is a real `{{#include}}` of compiled code, never a hand-typed
snippet that could silently drift out of sync.

## License

This repository mixes two licenses by content type — see
[`LICENSING.md`](LICENSING.md) for the full explanation and
[`REUSE.toml`](REUSE.toml) for the machine-readable mapping (this repo is
[REUSE](https://reuse.software/)-compliant):

- **Book prose** (`src/**`): [CC BY-SA 4.0](LICENSES/CC-BY-SA-4.0.txt)
- **All code** (`examples/`, `exercises/`, `solutions/`, and this repo's
  own tooling/config): [MIT](LICENSES/MIT.txt) OR
  [Apache-2.0](LICENSES/Apache-2.0.txt) — the same dual license the Rust
  compiler itself uses, so every example, exercise, and solution is free
  to copy into your own projects.

## Status

All 32 chapters (Parts 0–5, including the capstone project) are written,
guided-example-backed, exercise-backed, and verified. The Appendix
(C-to-Rust glossary, cheat sheet, further reading) is scaffolded in
`src/appendix/` but not yet written — contributions welcome.
