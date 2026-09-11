// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

use std::collections::HashMap;

/// Count how many times each whitespace-separated word appears in `text`,
/// case-insensitively (`"The The"` counts as `"the"` twice).
pub fn word_count(text: &str) -> HashMap<String, usize> {
    todo!("split on whitespace, lowercase each word, and tally occurrences into a HashMap")
}

/// Return `items` with duplicate values removed, keeping only each
/// value's *first* occurrence and preserving the original order — a
/// `Vec` alone can't check "have I seen this?" in O(1), so use a
/// `HashSet` alongside it to decide what to keep.
pub fn dedup_preserve_order(items: Vec<i32>) -> Vec<i32> {
    todo!("keep only the first occurrence of each value, in original order")
}
