// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Reads `path` and returns the total number of whitespace-separated
/// words in it.
pub fn total_words(path: &Path) -> Result<usize> {
    todo!(
        "read the file (with `.with_context` or `?` — Ch. 23/24's anyhow pattern), then count words with .split_whitespace()"
    )
}

/// Reads `path` and returns the most frequent word (case-insensitively),
/// or `None` if the file contains no words. On a tie, return whichever
/// word appears first in the file.
///
/// A `HashMap<String, usize>` (Ch. 11) is one reasonable way to count
/// occurrences per lowercased word before finding the maximum.
pub fn most_frequent_word(path: &Path) -> Result<Option<String>> {
    todo!(
        "count occurrences per lowercased word, then find the max — preserve first-seen order on a tie"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn total_words_counts_correctly_for_a_known_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("sample.txt");
        fs::write(&path, "the quick brown fox").unwrap();
        assert_eq!(total_words(&path).unwrap(), 4);
    }
}
