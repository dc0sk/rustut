// SPDX-License-Identifier: MIT OR Apache-2.0
use ch10_ex01_lookup_and_parse::{first_positive, parse_pair};

#[test]
fn first_positive_finds_it() {
    assert_eq!(first_positive(&[-3, -1, 0, 2, 5]), Some(2));
}

#[test]
fn first_positive_none_when_all_non_positive() {
    assert_eq!(first_positive(&[-3, -1, 0]), None);
}

#[test]
fn first_positive_none_on_empty_slice() {
    assert_eq!(first_positive(&[]), None);
}

#[test]
fn parse_pair_parses_both_sides() {
    assert_eq!(parse_pair("3,4"), Ok((3, 4)));
    assert_eq!(parse_pair(" 10 , -7 "), Ok((10, -7)));
}

#[test]
fn parse_pair_reports_missing_comma() {
    assert!(parse_pair("34").is_err());
}

#[test]
fn parse_pair_reports_bad_number() {
    assert!(parse_pair("3,x").is_err());
    assert!(parse_pair("x,4").is_err());
}
