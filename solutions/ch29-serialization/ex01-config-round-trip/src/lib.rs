// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch29-serialization/ex01-config-round-trip.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub retries: u32,
    #[serde(default)]
    pub verbose: bool,
}

pub fn load_settings(json: &str) -> Result<AppSettings, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn save_settings(settings: &AppSettings) -> String {
    // AppSettings is built entirely out of String/u32/bool — serde_json
    // cannot fail serializing it (the only serde_json::Error cases for
    // `to_string` are things like non-finite floats or non-string map
    // keys, neither of which this type has).
    serde_json::to_string(settings).expect("AppSettings always serializes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let settings = AppSettings {
            name: "svc".to_string(),
            retries: 3,
            verbose: true,
        };
        let json = save_settings(&settings);
        assert_eq!(load_settings(&json).unwrap(), settings);
        assert!(load_settings("not json").is_err());
    }
}
