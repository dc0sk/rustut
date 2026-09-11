// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md. Nothing to change in this file — the fix belongs in
//! Cargo.toml.

pub fn greet(name: &str) -> String {
    stringy_lib::shout(&format!("hello, {name}"))
}
