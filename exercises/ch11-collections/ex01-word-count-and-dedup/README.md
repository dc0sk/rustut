<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 11.1 — word count and dedup

## Task

1. `word_count(text: &str) -> HashMap<String, usize>` — tally how many
   times each whitespace-separated word appears, case-insensitively.
2. `dedup_preserve_order(items: Vec<i32>) -> Vec<i32>` — remove duplicate
   values, keeping each value's *first* occurrence, in original order.
   (This is different from `Vec::dedup`, which only removes *consecutive*
   duplicates — you need to remember every value you've already seen, not
   just the previous one.)

## Done when

- `cargo test -p ch11-ex01-word-count-and-dedup` passes
- `cargo clippy -p ch11-ex01-word-count-and-dedup --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `HashMap`'s `entry()` API (`*map.entry(key).or_insert(0) += 1`, or
  similar) is the idiomatic "increment a counter, inserting a default if
  it's the first time" pattern — no manual "check if present, then
  insert-or-update" two-step.
- A `HashSet` alongside a `Vec` gives you O(1) "have I seen this before?"
  membership tests without giving up the `Vec`'s insertion order — a
  combination you'd otherwise hand-roll as a hash table plus a separate
  ordered list in C.
