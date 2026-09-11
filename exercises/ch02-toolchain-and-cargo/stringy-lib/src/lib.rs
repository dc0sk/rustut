// SPDX-License-Identifier: MIT OR Apache-2.0
//! Tiny helper crate for Chapter 2's Cargo exercise.
//! Nothing to fill in here — this crate is already complete; the exercise
//! is in `../ex01-wire-a-dependency`.

pub fn shout(input: &str) -> String {
    format!("{}!", input.trim().to_uppercase())
}
