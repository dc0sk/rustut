// SPDX-License-Identifier: MIT OR Apache-2.0
use ch11_ex01_word_count_and_dedup::{dedup_preserve_order, word_count};
use std::collections::HashMap;

#[test]
fn word_count_tallies_case_insensitively() {
    let counts = word_count("The quick fox the FOX jumps");
    let expected: HashMap<String, usize> = [
        ("the".to_string(), 2),
        ("quick".to_string(), 1),
        ("fox".to_string(), 2),
        ("jumps".to_string(), 1),
    ]
    .into_iter()
    .collect();
    assert_eq!(counts, expected);
}

#[test]
fn word_count_empty_text_is_empty_map() {
    assert!(word_count("").is_empty());
}

#[test]
fn dedup_keeps_first_occurrence_order() {
    assert_eq!(
        dedup_preserve_order(vec![3, 1, 3, 2, 1, 4]),
        vec![3, 1, 2, 4]
    );
}

#[test]
fn dedup_empty_input_is_empty() {
    assert_eq!(dedup_preserve_order(vec![]), Vec::<i32>::new());
}

#[test]
fn dedup_no_duplicates_is_unchanged() {
    assert_eq!(dedup_preserve_order(vec![1, 2, 3]), vec![1, 2, 3]);
}
