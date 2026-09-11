// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch07-lifetimes/ex01-parser-struct.

pub struct Parser<'a> {
    remaining: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { remaining: input }
    }

    pub fn next_word(&mut self) -> Option<&'a str> {
        let trimmed = self.remaining.trim_start();
        if trimmed.is_empty() {
            self.remaining = trimmed;
            return None;
        }
        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        let (word, rest) = trimmed.split_at(end);
        self.remaining = rest;
        Some(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let mut p = Parser::new("  the quick  brown");
        assert_eq!(p.next_word(), Some("the"));
        assert_eq!(p.next_word(), Some("quick"));
        assert_eq!(p.next_word(), Some("brown"));
        assert_eq!(p.next_word(), None);
    }
}
