// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md for the task.

/// Splits `remaining` into whitespace-delimited words, one at a time,
/// without allocating — every word returned by `next_word` borrows
/// directly from the `&'a str` originally passed to `new`.
pub struct Parser<'a> {
    remaining: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        todo!("store `input` as `remaining`")
    }

    /// Returns the next whitespace-delimited word, advancing past it and
    /// any whitespace before it, or `None` once nothing but whitespace
    /// (or nothing at all) remains.
    pub fn next_word(&mut self) -> Option<&'a str> {
        todo!(
            "trim leading whitespace from `self.remaining`, split off the \
             next word, update `self.remaining` to what's left, and return \
             the word — or None if nothing remains"
        )
    }
}
