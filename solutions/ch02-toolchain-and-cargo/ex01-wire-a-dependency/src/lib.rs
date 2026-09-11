// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch02-toolchain-and-cargo/ex01-wire-a-dependency.
//! The only real "fix" is in Cargo.toml — this file is identical to the
//! exercise's.

pub fn greet(name: &str) -> String {
    stringy_lib_solution::shout(&format!("hello, {name}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(greet("world"), "HELLO, WORLD!");
        assert_eq!(greet("ada  "), "HELLO, ADA!");
    }
}
