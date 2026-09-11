// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 29 — serialization with serde.

// ANCHOR: config_struct
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub tags: Vec<String>,
}
// ANCHOR_END: config_struct

// ANCHOR: round_trip
/// Serialize `config` to JSON and immediately deserialize it back. A C
/// engineer's equivalent would be hand-writing a marshal/unmarshal pair
/// (or `memcpy`-ing a packed struct) with nothing checking, at compile
/// time, that what you wrote out is what you'll read back in — a field
/// added or reordered later silently breaks old saved data. Here, the
/// `#[derive]` above generates both directions from one struct
/// definition, so they can never drift apart from each other.
pub fn round_trip(config: &ServerConfig) -> serde_json::Result<ServerConfig> {
    let json = serde_json::to_string(config)?;
    serde_json::from_str(&json)
}
// ANCHOR_END: round_trip

// ANCHOR: untrusted_input
/// Parsing JSON from a source you don't fully trust (a network peer, a
/// config file a user handed you) should never `unwrap()` — a malformed
/// or malicious payload becomes an ordinary `Err` you handle, not a
/// panic. Chapter 31 covers the security posture for untrusted input in
/// general; this is the serialization-specific instance of the same
/// rule.
pub fn parse_config(input: &str) -> Result<ServerConfig, serde_json::Error> {
    serde_json::from_str(input)
}
// ANCHOR_END: untrusted_input

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_preserves_equality() {
        let config = ServerConfig {
            host: "localhost".to_string(),
            port: 8080,
            tags: vec!["dev".to_string()],
        };
        assert_eq!(round_trip(&config).unwrap(), config);
    }

    #[test]
    fn malformed_json_is_an_error_not_a_panic() {
        assert!(parse_config("{ not valid json").is_err());
    }

    #[test]
    fn missing_required_field_is_an_error() {
        assert!(parse_config(r#"{"host": "localhost"}"#).is_err());
    }
}
