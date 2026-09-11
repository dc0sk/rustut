// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch23-error-handling-and-cli/ex01-parse-config.

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

pub fn parse_settings(input: &str) -> Result<Settings, SettingsError> {
    let host = find_field(input, "host")?;
    let port_str = find_field(input, "port")?;
    let port = port_str
        .parse::<u16>()
        .map_err(|_| SettingsError::InvalidPort(port_str.to_string()))?;
    Ok(Settings {
        host: host.to_string(),
        port,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(
            parse_settings("host=example.com\nport=8080").unwrap(),
            Settings {
                host: "example.com".to_string(),
                port: 8080
            }
        );
        assert_eq!(
            parse_settings("port=8080"),
            Err(SettingsError::MissingField("host"))
        );
        assert_eq!(
            parse_settings("host=example.com\nport=x"),
            Err(SettingsError::InvalidPort("x".to_string()))
        );
    }
}
