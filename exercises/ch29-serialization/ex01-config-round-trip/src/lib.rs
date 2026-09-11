// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md. The `AppSettings` struct below is complete — the two
//! functions are the exercise.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub retries: u32,
    #[serde(default)]
    pub verbose: bool,
}

/// Parse `json` into an `AppSettings`. Must return `Err` on malformed or
/// incomplete (missing a required field) input rather than panicking.
pub fn load_settings(json: &str) -> Result<AppSettings, serde_json::Error> {
    todo!("deserialize `json` into an AppSettings, propagating any error with `?`")
}

/// Serialize `settings` back to a JSON string.
pub fn save_settings(settings: &AppSettings) -> String {
    todo!("serialize `settings` to a JSON string")
}
