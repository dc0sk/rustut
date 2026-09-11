// SPDX-License-Identifier: MIT OR Apache-2.0
use ch23_ex01_parse_config::{Settings, SettingsError, parse_settings};

#[test]
fn parses_valid_settings() {
    let input = "host=example.com\nport=8080\n";
    assert_eq!(
        parse_settings(input).unwrap(),
        Settings {
            host: "example.com".to_string(),
            port: 8080
        }
    );
}

#[test]
fn missing_host_is_an_error() {
    let input = "port=8080\n";
    assert_eq!(
        parse_settings(input),
        Err(SettingsError::MissingField("host"))
    );
}

#[test]
fn missing_port_is_an_error() {
    let input = "host=example.com\n";
    assert_eq!(
        parse_settings(input),
        Err(SettingsError::MissingField("port"))
    );
}

#[test]
fn invalid_port_is_an_error() {
    let input = "host=example.com\nport=notanumber\n";
    assert_eq!(
        parse_settings(input),
        Err(SettingsError::InvalidPort("notanumber".to_string()))
    );
}
