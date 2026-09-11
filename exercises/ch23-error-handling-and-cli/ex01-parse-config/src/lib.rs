// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SettingsError {
    #[error("missing required field `{0}`")]
    MissingField(&'static str),
    #[error("field `port` must be a positive integer, got `{0}`")]
    InvalidPort(String),
}

#[derive(Debug, PartialEq)]
pub struct Settings {
    pub host: String,
    pub port: u16,
}

/// Looks up `key` in `input` (newline-separated "key=value" lines) and
/// returns its value, or `SettingsError::MissingField(key)` if absent.
/// Already implemented — build `parse_settings` on top of this with `?`.
fn find_field<'a>(input: &'a str, key: &'static str) -> Result<&'a str, SettingsError> {
    input
        .lines()
        .find_map(|line| {
            line.split_once('=')
                .filter(|(k, _)| *k == key)
                .map(|(_, v)| v)
        })
        .ok_or(SettingsError::MissingField(key))
}

/// Parses newline-separated "key=value" lines (`host` and `port` both
/// required) into a `Settings`, propagating any failure with `?`.
pub fn parse_settings(input: &str) -> Result<Settings, SettingsError> {
    todo!(
        "use find_field(input, \"host\") and find_field(input, \"port\") with `?`, parse port as u16, map a parse failure to SettingsError::InvalidPort"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_field_locates_a_key() {
        assert_eq!(
            find_field("host=example.com\nport=8080", "host"),
            Ok("example.com")
        );
    }
}
