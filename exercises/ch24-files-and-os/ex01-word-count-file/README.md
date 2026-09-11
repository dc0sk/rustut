<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 24.1 — word count file

## Task

Implement two functions that read a real file from disk:

1. `total_words(path)` — the total number of whitespace-separated words.
2. `most_frequent_word(path)` — the most frequent word, compared
   case-insensitively (`"Rust"` and `"rust"` count as the same word; return
   it lowercased), or `None` if the file has no words. Break ties by
   whichever word appeared **first** in the file.

Both return `anyhow::Result<...>` — propagate file-read errors with `?`
rather than `.unwrap()`ing them (a missing file should be a real,
inspectable error, not a panic — see Ch. 23).

## Done when

- `cargo test -p ch24-ex01-word-count-file` passes
- `cargo clippy -p ch24-ex01-word-count-file --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Reading a real file with `std::fs`, and propagating `io::Error` into
  `anyhow::Result` with `?` — no manual wrapping needed.
- Using a `HashMap` (Ch. 11) to count occurrences, then finding the
  maximum while preserving first-seen order on a tie.
- Tests that touch the filesystem use `tempfile::tempdir()` for an
  isolated, automatically-cleaned-up directory — never a fixed path,
  which could collide with another test running in parallel.
