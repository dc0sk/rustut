<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Further Reading

This book is deliberately scoped: general-purpose Rust, taught through a
C engineer's existing mental model. Here's where to go for the topics it
touched briefly or skipped outright, organized by what you're looking for
rather than by source.

## The official documentation

- [The Rust Programming Language](https://doc.rust-lang.org/book/) (aka
  "the book") — the canonical from-scratch introduction. If any chapter
  here moved faster than you'd like, its equivalent chapter is a slower,
  more thorough second pass.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — the
  same material as the official book, indexed by runnable snippet rather
  than by narrative.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — the deep end of
  `unsafe`: what the compiler actually assumes you're upholding, aliasing
  rules for raw pointers, and the hazards Ch. 17 only introduced.
- [The Cargo Book](https://doc.rust-lang.org/cargo/) — everything Ch. 2
  and Ch. 14 covered about workspaces, profiles, and dependencies, in
  full.
- [The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
  — the checklist real published crates hold themselves to; a natural
  next step after Ch. 27's idiomatic-API-design chapter.
- [The Rust Reference](https://doc.rust-lang.org/reference/) — precise
  language semantics, for when a chapter's prose isn't specific enough.

## Embedded & `no_std` Rust

This book's introduction was explicit that it teaches general-purpose
Rust using an embedded/C background as a lens, not embedded Rust itself.
If the embedded side is actually what you're here for:

- [The Embedded Rust Book](https://docs.rust-embedded.org/book/) — the
  `no_std` equivalent of this book's Part 4: HALs, PACs, and running on
  bare metal.
- [Embedded HAL](https://github.com/rust-embedded/embedded-hal) — the
  trait ecosystem Ch. 26's FFI chapter gestured at but didn't build.
- [`probe-rs`](https://probe.rs/) — flashing and debugging real hardware
  from Rust tooling, without a vendor-specific debugger.

## Macros

Ch. 4 mentioned `macro_rules!` as the `#define`-with-hygiene replacement
and never built one. For that:

- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/) —
  declarative macros (`macro_rules!`) in depth.
- [The `syn`/`quote` crates](https://docs.rs/syn/) — the foundation of
  nearly every procedural (`#[derive(...)]`-style) macro in the ecosystem,
  for when you want to write your own `#[derive]`.

## Async, in depth

Ch. 22 covered why async needs a runtime and gave one worked example.
For the rest of the ecosystem:

- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
  — the `Future` trait, executors, and pinning, properly explained.
- [Tokio's own tutorial](https://tokio.rs/tokio/tutorial) — this book used
  a small slice of `tokio`; its own docs cover the rest (streams, sync
  primitives built for async code, `tracing` integration).

## Testing beyond what Ch. 30 covered

- [`proptest` book](https://proptest-rs.github.io/proptest/) — this book
  used `proptest` for one exercise; its docs cover strategies, shrinking,
  and regression files in full.
- [`cargo-fuzz`](https://rust-fuzz.github.io/book/) — Ch. 31 mentioned
  fuzzing without setting one up; this is the real thing.
- [`criterion`](https://bheisler.github.io/criterion.rs/book/) —
  statistically rigorous benchmarking, for when "is this actually faster"
  needs a real answer instead of a stopwatch.

## Staying current

- [This Week in Rust](https://this-week-in-rust.org/) — a weekly digest of
  the ecosystem, RFCs, and new crates.
- [users.rust-lang.org](https://users.rust-lang.org/) — the official
  community forum; good for "is this idiomatic" questions.
- [/r/rust](https://www.reddit.com/r/rust/) — higher-traffic, more
  variable signal-to-noise, but often first to discuss a new release.
- [crates.io](https://crates.io/) — before writing something from
  scratch, check whether a well-maintained crate already does it; this
  book depended on several (`anyhow`, `thiserror`, `clap`, `serde`,
  `tokio`, `proptest`) rather than reinventing them, and that's the norm,
  not a shortcut.

This closes the Appendix, and the book. If you worked through the
capstone (Ch. 32), you've already exercised most of what's linked above
in miniature — this list is for going deeper on any one piece of it.
