<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 31.1 — checked buffer size

Implement `buffer_size`, which computes `header_bytes + record_count *
record_size` — the classic "how big a buffer do I need" calculation.

## Task

Use `checked_mul` and `checked_add` (not plain `*`/`+`) so that an overflow
at *either* step returns `None` instead of silently wrapping to a small,
wrong number. This is not a hypothetical: `malloc(header + count * size)`
overflowing exactly this way, in exactly this shape, is a real, recurring
CVE pattern in C — the multiplication wraps, `malloc` returns a buffer far
smaller than the code believes it asked for, and whatever fills the buffer
in writes past the end of it.

## Done when

- `cargo test -p ch31-ex01-checked-buffer-size` passes
- `cargo clippy -p ch31-ex01-checked-buffer-size --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `checked_mul`/`checked_add` turn "does this overflow" into an `Option`
  you're forced to handle, instead of a silent wraparound (`wrapping_*`) or
  an undefined-behavior-in-C footgun.
- Chaining two checked operations (`checked_mul(...).and_then(|product|
  product.checked_add(...))`, or an equivalent `?`-based form) composes
  cleanly — no need to check "did it overflow" with a separate branch after
  every arithmetic operation the way you would in C.
