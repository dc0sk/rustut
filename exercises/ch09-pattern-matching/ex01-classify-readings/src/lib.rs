// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

#[derive(Debug, Clone, PartialEq)]
pub enum SensorReading {
    Temperature(f64),
    Humidity(f64),
    Error { code: u32, message: String },
    Idle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Alert {
    Critical(String),
    Warning(String),
    Nominal,
}

/// Classify a reading using `match`, with guards for the threshold checks:
///
/// - `Temperature(t)` with `t > 90.0` -> `Alert::Critical("temperature over 90".into())`
/// - `Temperature(t)` with `t > 75.0` -> `Alert::Warning("temperature over 75".into())`
/// - any other `Temperature` -> `Alert::Nominal`
/// - `Humidity(h)` with `h > 95.0` -> `Alert::Warning("humidity over 95".into())`
/// - any other `Humidity` -> `Alert::Nominal`
/// - `Error { code, message }` -> `Alert::Critical(format!("error {code}: {message}"))`
/// - `Idle` -> `Alert::Nominal`
pub fn classify(reading: &SensorReading) -> Alert {
    todo!("match on `reading`; use match guards (`if ...`) for the threshold checks")
}
