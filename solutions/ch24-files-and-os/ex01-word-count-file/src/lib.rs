// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch24-files-and-os/ex01-word-count-file.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn total_words(path: &Path) -> Result<usize> {
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(text.split_whitespace().count())
}

pub fn most_frequent_word(path: &Path) -> Result<Option<String>> {
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;

    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut first_seen_order: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        let lower = word.to_lowercase();
        if !counts.contains_key(&lower) {
            first_seen_order.push(lower.clone());
        }
        *counts.entry(lower).or_insert(0) += 1;
    }

    let mut best: Option<(&str, usize)> = None;
    for word in &first_seen_order {
        let count = counts[word];
        let is_new_best = match best {
            None => true,
            Some((_, best_count)) => count > best_count,
        };
        if is_new_best {
            best = Some((word, count));
        }
    }
    Ok(best.map(|(word, _)| word.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn matches_public_test_expectations() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("sample.txt");
        fs::write(&path, "Rust rust RUST is fun\n").unwrap();
        assert_eq!(total_words(&path).unwrap(), 5);
        assert_eq!(most_frequent_word(&path).unwrap(), Some("rust".to_string()));
    }
}
