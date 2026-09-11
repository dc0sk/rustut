// SPDX-License-Identifier: MIT OR Apache-2.0
use ch29_ex01_config_round_trip::{AppSettings, load_settings, save_settings};

#[test]
fn round_trip_preserves_equality() {
    let settings = AppSettings {
        name: "svc".to_string(),
        retries: 3,
        verbose: true,
    };
    let json = save_settings(&settings);
    let loaded = load_settings(&json).unwrap();
    assert_eq!(loaded, settings);
}

#[test]
fn missing_optional_field_defaults_to_false() {
    let loaded = load_settings(r#"{"name": "svc", "retries": 3}"#).unwrap();
    assert!(!loaded.verbose);
}

#[test]
fn missing_required_field_is_an_error() {
    assert!(load_settings(r#"{"retries": 3}"#).is_err());
}

#[test]
fn malformed_json_is_an_error_not_a_panic() {
    assert!(load_settings("not json").is_err());
}
