// SPDX-License-Identifier: MIT OR Apache-2.0
use ch02_ex01_wire_a_dependency::greet;

#[test]
fn greets_and_shouts() {
    assert_eq!(greet("world"), "HELLO, WORLD!");
}

#[test]
fn trailing_whitespace_in_the_name_is_trimmed() {
    assert_eq!(greet("ada  "), "HELLO, ADA!");
}
