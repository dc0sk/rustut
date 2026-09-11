// SPDX-License-Identifier: MIT OR Apache-2.0
//! Copy of exercises/ch02-toolchain-and-cargo/stringy-lib, kept alongside
//! the solution so the solution crate is self-contained.

pub fn shout(input: &str) -> String {
    format!("{}!", input.trim().to_uppercase())
}
