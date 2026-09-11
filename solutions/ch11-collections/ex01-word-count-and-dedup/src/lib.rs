// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch11-collections/ex01-word-count-and-dedup.

use std::collections::{HashMap, HashSet};

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

pub fn dedup_preserve_order(items: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    items.into_iter().filter(|v| seen.insert(*v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(word_count("").len(), 0);
        assert_eq!(
            dedup_preserve_order(vec![3, 1, 3, 2, 1, 4]),
            vec![3, 1, 2, 4]
        );
    }
}
