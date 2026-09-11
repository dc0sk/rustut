// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: sensor_reading
#[derive(Debug, Clone, PartialEq)]
pub enum SensorReading {
    Temperature(f64),
    Humidity(f64),
    Error { code: u32, message: String },
    Idle,
}
// ANCHOR_END: sensor_reading

// ANCHOR: match_exhaustive
/// `match` must handle every variant of an enum, or the crate doesn't
/// compile — there is no equivalent of C's `switch`, where a missing
/// `case` (or a forgotten `default`) just silently does nothing at
/// runtime, a real bug class that has caused real incidents:
///
/// ```rust,compile_fail
/// enum Light { Red, Yellow, Green }
///
/// fn describe(l: Light) -> &'static str {
///     match l {
///         Light::Red => "stop",
///         Light::Green => "go",
///     } // error[E0004]: non-exhaustive patterns: `Light::Yellow` not covered
/// }
/// ```
pub fn describe(reading: &SensorReading) -> String {
    match reading {
        SensorReading::Temperature(t) => format!("temperature: {t:.1}"),
        SensorReading::Humidity(h) => format!("humidity: {h:.1}%"),
        SensorReading::Error { code, message } => format!("error {code}: {message}"),
        SensorReading::Idle => "idle".to_string(),
    }
}
// ANCHOR_END: match_exhaustive

// ANCHOR: match_guard
/// A match guard (`if <condition>` after the pattern) adds a boolean
/// condition on top of the shape match already checked — something a C
/// `switch` can't express at all (`case` labels must be constants), which
/// is why C code reaches for an `if`/`else if` chain the moment a
/// condition depends on the payload's value, not just which variant it is.
pub fn temperature_alert(reading: &SensorReading) -> Option<&'static str> {
    match reading {
        SensorReading::Temperature(t) if *t > 90.0 => Some("critical"),
        SensorReading::Temperature(t) if *t > 75.0 => Some("warning"),
        _ => None,
    }
}
// ANCHOR_END: match_guard

// ANCHOR: if_let
/// `if let` is `match` for the common case of caring about exactly one
/// pattern and ignoring the rest — no `_ => {}` arm needed.
pub fn error_code(reading: &SensorReading) -> Option<u32> {
    if let SensorReading::Error { code, .. } = reading {
        Some(*code)
    } else {
        None
    }
}
// ANCHOR_END: if_let

// ANCHOR: while_let
/// `while let` repeats for as long as a pattern keeps matching — the
/// common use is draining a collection, here a stack of pending readings,
/// picking out just the error codes as we go.
pub fn drain_error_codes(mut pending: Vec<SensorReading>) -> Vec<u32> {
    let mut codes = Vec::new();
    while let Some(reading) = pending.pop() {
        if let SensorReading::Error { code, .. } = reading {
            codes.push(code);
        }
    }
    codes
}
// ANCHOR_END: while_let

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_covers_every_variant() {
        assert_eq!(
            describe(&SensorReading::Temperature(21.4)),
            "temperature: 21.4"
        );
        assert_eq!(describe(&SensorReading::Humidity(55.0)), "humidity: 55.0%");
        assert_eq!(
            describe(&SensorReading::Error {
                code: 7,
                message: "sensor offline".into()
            }),
            "error 7: sensor offline"
        );
        assert_eq!(describe(&SensorReading::Idle), "idle");
    }

    #[test]
    fn temperature_alert_thresholds() {
        assert_eq!(
            temperature_alert(&SensorReading::Temperature(95.0)),
            Some("critical")
        );
        assert_eq!(
            temperature_alert(&SensorReading::Temperature(80.0)),
            Some("warning")
        );
        assert_eq!(temperature_alert(&SensorReading::Temperature(50.0)), None);
        assert_eq!(temperature_alert(&SensorReading::Idle), None);
    }

    #[test]
    fn if_let_extracts_error_code() {
        assert_eq!(
            error_code(&SensorReading::Error {
                code: 42,
                message: "x".into()
            }),
            Some(42)
        );
        assert_eq!(error_code(&SensorReading::Idle), None);
    }

    #[test]
    fn while_let_drains_only_error_codes() {
        let pending = vec![
            SensorReading::Temperature(10.0),
            SensorReading::Error {
                code: 1,
                message: "a".into(),
            },
            SensorReading::Idle,
            SensorReading::Error {
                code: 2,
                message: "b".into(),
            },
        ];
        let mut codes = drain_error_codes(pending);
        codes.sort_unstable();
        assert_eq!(codes, vec![1, 2]);
    }
}
