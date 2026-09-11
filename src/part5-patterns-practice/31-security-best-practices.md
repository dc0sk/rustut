<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Security Best Practices

Rust's type system and borrow checker close off whole bug classes at
compile time (Ch. 1 onward). None of that automates away the parts of
security that were never about memory safety in the first place:
knowing what's in your dependency tree, keeping `unsafe` auditable,
handling arithmetic that can overflow, and knowing what tools exist for
the rest. This chapter is a tour of those tools.

## Dependency auditing

C's usual dependency story is "whatever got vendored into `third_party/`
or linked from the system," with no automated way to ask "does any of
this have a known CVE." Rust's answer is mechanical:

- **`cargo audit`** checks `Cargo.lock` — every dependency, transitive
  included, at its *exact* resolved version — against the
  [RustSec](https://rustsec.org/) advisory database, and fails if
  anything in your dependency tree has a published vulnerability.
- **`cargo-deny`** is broader policy enforcement: license compatibility,
  duplicate versions of the same crate bloating your binary, an explicit
  ban list.

This isn't hypothetical for this repository — `cargo audit` is already
wired into CI, running on every push and pull request, plus a weekly
schedule to catch newly-published advisories against dependencies that
haven't otherwise changed:

```yaml,ignore
{{#include ../../.github/workflows/exercises.yml:46:52}}
```

## Auditing `unsafe` code

Chapter 17 established the `// SAFETY: ...` comment convention: every
`unsafe` block should carry one explaining *why* the specific operation
is sound. That makes `unsafe` auditable, but someone still has to
remember to look. For a crate that has no legitimate reason to ever
contain `unsafe` at all, there's a stronger tool — make it a compile
error instead of a review question:

```rust,ignore
{{#include ../../examples/ch31-security-best-practices/src/lib.rs:forbid_unsafe}}
```

`#![forbid(unsafe_code)]` doesn't evaluate whether a given `unsafe` block
would have been sound — it categorically refuses the keyword, crate-wide,
before anyone gets a chance to review it. `forbid` (rather than `deny`)
also means nothing downstream in the same crate can `#[allow]` it back
in — the only way to add `unsafe` later is to remove this line, which
itself is a visible, reviewable diff.

## Integer overflow: choosing on purpose

Chapter 4 covered the default: debug builds panic on overflow, release
builds silently wrap. Both defaults are worse than C's *only* option for
signed integers — unconditional undefined behavior — but "the behavior
changes depending on your build profile" is still not something you want
load-bearing logic depending on by accident. When the specific behavior
matters, say so explicitly:

```rust,ignore
{{#include ../../examples/ch31-security-best-practices/src/lib.rs:overflow_tools}}
```

- **`checked_add`/`checked_mul`** return `None` on overflow — the answer
  is "impossible to compute," and you're forced to handle that.
- **`saturating_add`** clamps at the type's min/max — the right choice
  when "as much as will fit" is an acceptable answer.
- **`wrapping_add`** gives defined, silent wraparound — reach for this
  only when wraparound genuinely *is* the intended behavior (a hash mix,
  a ring-buffer index), never as a default choice.

This connects to a real, recurring C vulnerability class: a buffer-size
calculation like `malloc(header + count * size)` where the arithmetic
overflows, wraps to a small number, `malloc` returns a buffer far smaller
than the code believes it asked for, and whatever fills it in writes past
the end. Rust doesn't make that calculation impossible to get wrong — but
`checked_mul`/`checked_add` make it possible to get *right*, explicitly,
in a way the compiler and a reviewer can both see.

## Supply chain

Chapter 2 established that this repository commits `Cargo.lock`. The
security reason: it pins the *exact* version of every dependency,
transitive included, that a build actually used — not just the direct
dependencies your `Cargo.toml` names. `cargo audit` above is only
checking something meaningful because that exact-version record exists;
a range like `"1.0"` in `Cargo.toml` alone doesn't tell you which `1.0.x`
was actually built.

Worth knowing, briefly: crates.io has no walled-garden vetting — anyone
can publish a crate. The trust model is "you get automated tooling
(`cargo audit`/`cargo-deny`) plus your own judgment about what you
depend on," not "everything on crates.io has been reviewed for you."

## Fuzzing

`cargo-fuzz` (built on `libFuzzer`) generates random/mutated input and
throws it at a function you designate, looking for panics or — in
`unsafe` code — actual memory-safety violations that sanitizers can
catch. It's aimed squarely at code that parses untrusted input:
Chapter 29's deserialization code is exactly the kind of target it's
good at. It's also only as good as the fixtures/corpus it starts from
and the code paths those inputs actually reach — a fuzz target that
never got past the first `if` in a parser hasn't tested the rest of it.
Setting one up is a real, separate `fuzz/` crate with its own corpus
management; that's disproportionate machinery for one book exercise, so
this chapter leaves it at "know this exists and what it's for" rather
than building a toy fuzz target and overstating what a few seconds of
fuzzing one function actually proved.

## Exercise

**Exercise 31.1**, in
[`exercises/ch31-security-best-practices/ex01-checked-buffer-size/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch31-security-best-practices/ex01-checked-buffer-size),
has you implement the buffer-size calculation described above using
checked arithmetic, tested specifically at the overflow boundary. See its
`README.md`.

Next: [Chapter 32 — Capstone Project](32-capstone.md).
