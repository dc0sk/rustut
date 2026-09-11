// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: vec_growth
/// A `Vec<T>` grows itself: no `realloc`, no manually-tracked `len`
/// vs. `capacity` pair to keep in sync by hand. `.push()` either fits in
/// the existing allocation or grows it — either way, the caller never
/// touches an allocator directly.
pub fn running_totals(values: &[i32]) -> Vec<i32> {
    let mut totals = Vec::new();
    let mut sum = 0;
    for &v in values {
        sum += v;
        totals.push(sum);
    }
    totals
}
// ANCHOR_END: vec_growth

// ANCHOR: string_vs_str
/// `String` is owned and growable; `&str` is a borrowed view into UTF-8
/// bytes someone else owns (a `String`, a `&'static str` literal, ...).
/// Neither is ever invalid UTF-8 — there's no equivalent of a C `char*`
/// that happens to hold bytes nobody validated.
///
/// A single visible character can take more than one byte: `.len()`
/// counts bytes, `.chars().count()` counts Unicode scalar values. Mixing
/// the two up is exactly the class of bug `strlen`/manual byte-indexing
/// causes on non-ASCII C strings — except here it can't silently corrupt
/// a character, because Rust refuses to let you index a `String` at an
/// arbitrary byte offset at all (slicing at a non-boundary panics rather
/// than returning a truncated code point).
pub fn byte_len_vs_char_count(s: &str) -> (usize, usize) {
    (s.len(), s.chars().count())
}
// ANCHOR_END: string_vs_str

// ANCHOR: hashmap
use std::collections::HashMap;

/// A `HashMap<K, V>` where you would otherwise hand-roll open addressing
/// or a chained hash table, plus a hash function, by hand.
pub fn word_lengths(text: &str) -> HashMap<&str, usize> {
    text.split_whitespace().map(|w| (w, w.len())).collect()
}
// ANCHOR_END: hashmap
