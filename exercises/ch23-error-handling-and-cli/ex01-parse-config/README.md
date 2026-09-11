<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 23.1 — parse config

## Task

Implement `parse_settings`, which parses newline-separated `key=value`
lines into a `Settings { host, port }`. Both fields are required. Use the
already-implemented `find_field` helper and the `?` operator to propagate
its `SettingsError` — don't write your own error-checking `match` for each
field, that's exactly what `?` is for.

Requirements:
- `host` missing → `SettingsError::MissingField("host")`.
- `port` missing → `SettingsError::MissingField("port")`.
- `port` present but not a valid `u16` → `SettingsError::InvalidPort(<the
  offending string>)`.

## Done when

- `cargo test -p ch23-ex01-parse-config` passes
- `cargo clippy -p ch23-ex01-parse-config --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `?` propagates a `Result`'s error unchanged (or converts it via `From`,
  which this exercise doesn't need since both calls already return
  `SettingsError`) — the direct alternative to C's "check the return
  value, then `return` or `goto fail`" after every fallible call.
- `thiserror`'s `#[error("...")]` gives each error variant its own
  human-readable message with zero boilerplate `Display` impl.
