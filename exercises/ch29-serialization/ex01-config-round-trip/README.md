<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 29.1 — config round-trip

`AppSettings` is a complete, derive-annotated struct. Implement the two
functions around it:

## Task

- `load_settings(json: &str) -> Result<AppSettings, serde_json::Error>` —
  deserialize `json`. Must **not** panic on malformed or incomplete input;
  return the `Err` instead.
- `save_settings(settings: &AppSettings) -> String` — serialize back to
  JSON.

`verbose` is annotated `#[serde(default)]`, so JSON that omits it should
still deserialize successfully, with `verbose` set to `false`.

## Done when

- `cargo test -p ch29-ex01-config-round-trip` passes
- `cargo clippy -p ch29-ex01-config-round-trip --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `serde`'s derive macros generate both serialize and deserialize from one
  struct definition — they cannot silently drift apart from each other the
  way hand-written C marshal/unmarshal code can.
- `#[serde(default)]` is how a field becomes optional in the wire format
  without becoming `Option<T>` in your struct.
- Deserializing untrusted input is a `Result`, not a `panic` — the same
  discipline as any other untrusted-input parsing (Ch. 31 covers this
  more generally).
