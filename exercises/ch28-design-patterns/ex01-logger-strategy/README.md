<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 28.1 — logger strategy

`Logger<S: LogSink>` should format a message and hand it to whichever
`LogSink` it was built with — the strategy pattern from this chapter,
statically dispatched via the generic `S` instead of a C-style function
pointer field.

## Task

1. Implement `VecSink::write` — push the given line onto `self.lines`.
2. Implement `Logger::log` — format `message` as `"[LOG] {message}"` and
   pass it to `self.sink.write(...)`.

## Done when

- `cargo test -p ch28-ex01-logger-strategy` passes
- `cargo clippy -p ch28-ex01-logger-strategy --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Swapping `Logger<VecSink>` for `Logger<SomeOtherSink>` changes behavior
  with no runtime cost and no `void*`/function-pointer plumbing — the
  compiler monomorphizes a separate `Logger` per concrete sink type.
- A sink that records into a `Vec` instead of printing is a normal,
  idiomatic way to make strategy-pattern code testable at all — you
  can't easily assert on stdout, but you can assert on a `Vec<String>`.
