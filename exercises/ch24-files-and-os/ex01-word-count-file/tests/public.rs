// SPDX-License-Identifier: MIT OR Apache-2.0
use ch24_ex01_word_count_file::{most_frequent_word, total_words};
use std::fs;
use tempfile::tempdir;

#[test]
fn total_words_counts_across_multiple_lines() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("sample.txt");
    fs::write(&path, "the quick brown fox\njumps over the lazy dog\n").unwrap();
    assert_eq!(total_words(&path).unwrap(), 9);
}

#[test]
fn total_words_is_zero_for_an_empty_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("empty.txt");
    fs::write(&path, "").unwrap();
    assert_eq!(total_words(&path).unwrap(), 0);
}

#[test]
fn most_frequent_word_is_case_insensitive() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("words.txt");
    fs::write(&path, "Rust rust RUST is fun\n").unwrap();
    assert_eq!(most_frequent_word(&path).unwrap(), Some("rust".to_string()));
}

#[test]
fn most_frequent_word_breaks_ties_by_first_occurrence() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("tie.txt");
    fs::write(&path, "alpha beta\n").unwrap();
    assert_eq!(
        most_frequent_word(&path).unwrap(),
        Some("alpha".to_string())
    );
}

#[test]
fn most_frequent_word_is_none_for_an_empty_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("empty.txt");
    fs::write(&path, "   \n  \n").unwrap();
    assert_eq!(most_frequent_word(&path).unwrap(), None);
}

#[test]
fn missing_file_is_a_real_error_not_a_panic() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("does-not-exist.txt");
    assert!(total_words(&path).is_err());
}
