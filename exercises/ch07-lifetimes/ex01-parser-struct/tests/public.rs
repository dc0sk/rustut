// SPDX-License-Identifier: MIT OR Apache-2.0
use ch07_ex01_parser_struct::Parser;

#[test]
fn splits_on_whitespace() {
    let mut p = Parser::new("the quick brown fox");
    assert_eq!(p.next_word(), Some("the"));
    assert_eq!(p.next_word(), Some("quick"));
    assert_eq!(p.next_word(), Some("brown"));
    assert_eq!(p.next_word(), Some("fox"));
    assert_eq!(p.next_word(), None);
}

#[test]
fn handles_leading_and_repeated_whitespace() {
    let mut p = Parser::new("  a   b ");
    assert_eq!(p.next_word(), Some("a"));
    assert_eq!(p.next_word(), Some("b"));
    assert_eq!(p.next_word(), None);
}

#[test]
fn empty_input_yields_no_words() {
    let mut p = Parser::new("");
    assert_eq!(p.next_word(), None);
}

#[test]
fn calling_next_word_after_exhaustion_stays_none() {
    let mut p = Parser::new("one");
    assert_eq!(p.next_word(), Some("one"));
    assert_eq!(p.next_word(), None);
    assert_eq!(p.next_word(), None);
}

#[test]
fn returned_words_outlive_the_mutable_borrow_used_to_get_them() {
    // `next_word`'s return type borrows from the `'a` on `Parser`, not
    // from `&mut self` — so the word it returns is allowed to outlive
    // the `Parser` itself. If `next_word` were (incorrectly) written as
    // `fn next_word(&'a mut self) -> Option<&'a str>`, `word` would not
    // be usable past the end of this block, and this would fail to
    // compile.
    let text = String::from("alpha beta");
    let word = {
        let mut p = Parser::new(&text);
        p.next_word().unwrap()
    };
    assert_eq!(word, "alpha");
}
