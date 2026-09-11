// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: option_vs_null
/// No sentinel value, no null pointer: if there's no match, you get
/// `None`, and the compiler will not let you reach for a `&str` that
/// isn't there.
pub fn find_user<'a>(id: u32, users: &'a [(u32, &'a str)]) -> Option<&'a str> {
    users
        .iter()
        .find(|&&(uid, _)| uid == id)
        .map(|&(_, name)| name)
}
// ANCHOR_END: option_vs_null

// ANCHOR: combinators
/// `.map()` transforms the value inside a `Some`, leaving `None` alone.
/// `.unwrap_or_else()` supplies a fallback for the `None` case — together
/// they replace the C pattern of "check for NULL, then use, else use a
/// default" with one expression that can't skip the check.
pub fn greeting_for(id: u32, users: &[(u32, &str)]) -> String {
    find_user(id, users)
        .map(|name| format!("Hello, {name}!"))
        .unwrap_or_else(|| "Hello, stranger!".to_string())
}
// ANCHOR_END: combinators

// ANCHOR: question_mark
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub struct ConfigEntry {
    pub key: String,
    pub value: i64,
}

/// Parses one already-split `key`/`value` pair. Assumes well-formed input
/// (an exercise coming up hardens this) — the point here is just `?`:
/// `value.parse()` returns `Result<i64, ParseIntError>`, and `?` either
/// unwraps the `Ok` or returns the `Err` from this function immediately,
/// propagating it one level up without an explicit `if` at every call site.
pub fn parse_entry(key: &str, value: &str) -> Result<ConfigEntry, ParseIntError> {
    let value: i64 = value.trim().parse()?;
    Ok(ConfigEntry {
        key: key.trim().to_string(),
        value,
    })
}
// ANCHOR_END: question_mark
