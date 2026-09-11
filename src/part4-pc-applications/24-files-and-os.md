<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Files & OS Interaction

## Reading and writing files

```rust,ignore
{{#include ../../examples/ch24-files-and-os/src/lib.rs:write_and_read}}
```

`fs::write`/`fs::read_to_string` don't hand you a `File` at all for the
simple case — they open one, use it, and let it go out of scope
internally. That's not hiding anything dangerous: Chapter 16 already
established that a value's `Drop` impl runs the instant it goes out of
scope, on every path, including an early `?` return. A `std::fs::File`'s
`Drop` closes the underlying OS file descriptor exactly the same way — so
"did I remember to close this on every exit path" simply isn't a category
of bug here.

> **C engineer's mental model.** `fopen`/`fread`/`fwrite`/`fclose` map
> directly onto `File::open`/`Read`/`Write` methods/(automatic close) —
> the syscalls underneath are the same ones. What changes is that
> `fclose` stops being something *you* have to remember to call on every
> return path; it becomes something the compiler guarantees happens,
> the same RAII story as everywhere else in this book.

## Buffered I/O

```rust,ignore
{{#include ../../examples/ch24-files-and-os/src/lib.rs:buffered_io}}
```

`BufReader`/`BufWriter` batch the underlying `read()`/`write()` syscalls
into larger chunks — the same idea as a C `FILE*`'s internal buffer,
just spelled out as an explicit wrapper type instead of being baked
invisibly into `fopen`.

## Composing with Chapter 23's error handling

Notice `roundtrip` and `count_nonempty_lines` both return
`anyhow::Result<_>` and use `.with_context(...)` — `std::io::Error`
converts into `anyhow::Error` at the `?` site with zero glue code, exactly
as Chapter 23 described for any other error type. A missing file, a
permissions error, a disk-full condition — all become one ordinary `?`
away from a clean, contextual error message instead of a manual `errno`
check after every call.

## Environment variables

```rust,ignore
{{#include ../../examples/ch24-files-and-os/src/lib.rs:env_vars}}
```

## Testing code that touches the filesystem

Every test in this chapter's example uses `tempfile::tempdir()` rather
than a fixed path like `/tmp/test.txt`. This isn't extra caution for its
own sake: `cargo test` runs tests in parallel by default, so two tests
writing to the same fixed path *will* race — one truncating or
overwriting the file mid-read by the other, an intermittent failure
that's miserable to track down precisely because it depends on scheduling
luck. A fresh `tempdir()` per test gives each one its own isolated
directory, automatically deleted when it goes out of scope (`Drop`
again) — no collision possible, and no litter left behind either.

## Exercise

**Exercise 24.1**, in
[`exercises/ch24-files-and-os/ex01-word-count-file/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch24-files-and-os/ex01-word-count-file),
has you implement a small word-frequency counter over a real file on
disk. See its `README.md`.

Next: [Chapter 25 — Networking Basics](25-networking.md).
