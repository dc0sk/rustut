<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Introduction

You already know how to program close to the machine. You've written code
where a stray pointer corrupts a struct three allocations away, where a
`free()` twice crashes only in the release build under load, where a
`switch` without a `default` silently does nothing when someone adds an
enum value. You've debugged a data race with a logic analyzer because `gdb`
changed the timing enough to hide it. That experience is the entire premise
of this book: **it does not re-teach you what a pointer, the stack, the
heap, a linker, or a race condition is.** It teaches you Rust *in terms of*
that knowledge — what problem each Rust feature solves that you have
personally been burned by, and what it costs you in return.

## Who this is for

Electronics/embedded/systems engineers with solid C and a CS background,
who are **complete beginners to Rust**. If you've never written C and don't
know what a dangling pointer is, this book will still mostly work, but it
will spend less time than a from-scratch beginner book on things like "what
is a stack frame" — it assumes you already have that model and just needs
to map it onto Rust's.

## What this book is not

This is **not** an embedded/`no_std` tutorial. Your background is the lens
we use for analogies — malloc/free, pointers, linkers, volatile,
interrupts — not the destination. The destination is general-purpose Rust:
libraries, command-line tools, and networked PC applications. Part 4 does
cover FFI into C libraries, because "you already know C" is directly
useful there, but it is interop with existing C code, not bare-metal
programming.

## How each chapter is built

Every chapter has the same four pieces:

1. **Prose** — the concept, explained, with an explicit side-by-side
   comparison to the closest C idiom wherever one exists. Look for boxes
   marked **C engineer's mental model**.
2. **A guided example** — real, compiling code under `examples/chNN-topic/`
   that the chapter's prose `{{#include}}`s directly. If you run
   `cargo run --bin <name> -p chNN-topic-example` (or `cargo test -p
   chNN-topic-example`, for library examples), you're running the exact
   code you just read — never a hand-copied paraphrase that can drift out
   of sync.
3. **One or more exercises** — under `exercises/chNN-topic/exNN-slug/`,
   each a small real Cargo crate with a `README.md` prompt, a stub
   `src/lib.rs` containing `todo!()`, and a test suite. An exercise is
   "done" when `cargo test -p <crate>` and `cargo clippy -p <crate>
   --all-targets --all-features -- -D warnings` both pass — not when it
   subjectively "looks right."
4. **A reference solution** — under `solutions/chNN-topic/exNN-slug/`,
   committed and visible. This book assumes you're a self-directed learner
   working alone or with a coding agent, not a classroom trying to prevent
   copying — so solutions aren't hidden. Try the exercise yourself first;
   the point is the struggle, not the answer.

## Working with a coding agent

If you're using a coding agent (Claude Code or similar) to work through
exercises or to check your solutions, point it at [`AGENTS.md`](https://github.com/simonkeimer/rustut/blob/main/AGENTS.md)
in the repository root first. It defines exactly what commands to run, what
counts as a pass, and how to grade the handful of exercises that are
open-ended design tasks rather than pure test-pass/fail. An agent that
hasn't read it will invent its own grading criteria, which won't match this
book's.

## Conventions

- Every code sample you see in the book text is real, compiled code — see
  point 2 above. If you ever see a snippet that *isn't* runnable, it will
  say so explicitly (e.g. "pseudocode" or "does not compile, and here's
  why").
- `// C:` comments inside a Rust snippet mark the line where the C
  equivalent would do something meaningfully different or dangerous.
- Chapters are numbered by the order they build on each other, but Part 4
  and Part 5 chapters are largely independent of each other once you've
  finished Parts 0–3 — skip around if you have a specific application
  itch to scratch.

## License

This book's prose (everything under `src/`) is licensed
[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). All code —
examples, exercises, and solutions — is licensed MIT OR Apache-2.0, the
same dual license used by the Rust compiler and the overwhelming majority
of published crates, so you can copy it into your own projects freely. See
[`LICENSING.md`](https://github.com/simonkeimer/rustut/blob/main/LICENSING.md)
for the exact split.

Let's start with *why* — [Chapter 1](part0-orientation/01-why-rust.md).
