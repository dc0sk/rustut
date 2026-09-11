<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Application Error Handling & CLI Foundations

Welcome to Part 4. Everything from here on targets **general-purpose Rust
applications** — command-line tools, networked services, libraries you'd
ship to run on a PC or a server — not embedded targets. Your C/embedded
background is still the lens for analogies (it doesn't stop being useful
just because the destination changed), but `no_std`/bare-metal programming
isn't where this part is headed.

Chapter 10 introduced `Option`/`Result` and the `?` operator at the
language level. This chapter is about the *application-level* conventions
built on top of them — the equivalent of moving from "what does a function
signature that can fail look like" to "what does a whole program's error
handling strategy look like."

## Two crates, two different jobs

```toml
{{#include ../../examples/ch23-error-handling-and-cli/Cargo.toml}}
```

**`thiserror`** is for **libraries**: it derives a real `enum` error type
with one variant per distinct failure, each carrying its own
`#[error("...")]` message:

```rust,ignore
{{#include ../../examples/ch23-error-handling-and-cli/src/lib.rs:thiserror_error}}
```

> **C engineer's mental model.** In C, every fallible function invents its
> own convention for what failure looks like — a negative `int`, a NULL
> pointer, an out-parameter plus a boolean, `errno` after the fact — and
> every caller has to know (usually from a comment, if you're lucky) which
> convention applies *here*. `ConfigError` is a single, closed set of
> everything that can go wrong in this one operation, checked exhaustively
> by `match` (Chapter 9) if you want to handle each case differently.

**`anyhow`** is for **applications** — a CLI's `main`, or anything close to
it — where you usually don't want to design a bespoke error enum for every
possible failure; you just want *some* error, with enough human-readable
context to explain what you were trying to do:

```rust,ignore
{{#include ../../examples/ch23-error-handling-and-cli/src/lib.rs:anyhow_context}}
```

`.context("...")` wraps the underlying error with an explanation, and `?`
still works exactly as it does everywhere else — `anyhow::Error` can hold
*any* error type, so a `ConfigError` converts into it automatically at the
`?` site.

## `clap`: derive-based CLI parsing

```rust,ignore
{{#include ../../examples/ch23-error-handling-and-cli/src/lib.rs:clap_cli}}
```

`#[derive(Parser)]` reads this struct's fields and generates an entire
argument parser from them — positional arguments, `--flags`, `--help`,
`--version`, type conversion (a `u16` field would just fail to parse with a
clean error message if given `"abc"`) — all without you writing a parsing
loop.

> **C engineer's mental model.** `getopt`/`getopt_long` gets you *most* of
> the way to "parse `argv`," but you still hand-write the `switch` over
> option characters, still manually call `atoi`/`strtol` on every argument
> string, and still hand-write the usage message. `clap`'s derive macro
> generates all of that from the struct definition itself — the struct
> *is* the specification.

## Panic vs. `Result` vs. `std::process::exit`

A C `main` really has two modes: return an `int` exit code, or crash
(`abort()`, an unhandled signal, a segfault). Rust gives you a real third
option, and the trick is picking the right one for the failure you're
looking at:

```rust,ignore
{{#include ../../examples/ch23-error-handling-and-cli/src/lib.rs:exit_code_pattern}}
```

- **`Result` + a clean exit code** for *expected* failures — bad user
  input, a missing file, a network timeout. The user did something the
  program can legitimately reject; don't dump a stack trace at them.
- **`panic!`** for a *programmer* bug — an invariant you believed could
  never be violated actually was. A panic is closer to what an `assert()`
  failure or an unhandled signal gets you in C: a loud, immediate stop,
  because continuing would mean operating on a state your code doesn't
  actually know how to handle.
- Never use a panic where a clean error message and a nonzero exit code
  would do — that distinction is the main judgment call this chapter is
  asking you to make.

## Exercise

**Exercise 23.1**, in
[`exercises/ch23-error-handling-and-cli/ex01-parse-config/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch23-error-handling-and-cli/ex01-parse-config),
has you implement a small settings parser using `?` to propagate a
`thiserror`-derived error type. See its `README.md`.

Next: [Chapter 24 — Files & OS Interaction](24-files-and-os.md).
