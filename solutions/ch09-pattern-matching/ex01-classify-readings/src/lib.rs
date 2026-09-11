// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch09-pattern-matching/ex01-classify-readings.

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

pub fn classify(reading: &SensorReading) -> Alert {
    match reading {
        SensorReading::Temperature(t) if *t > 90.0 => Alert::Critical("temperature over 90".into()),
        SensorReading::Temperature(t) if *t > 75.0 => Alert::Warning("temperature over 75".into()),
        SensorReading::Temperature(_) => Alert::Nominal,
        SensorReading::Humidity(h) if *h > 95.0 => Alert::Warning("humidity over 95".into()),
        SensorReading::Humidity(_) => Alert::Nominal,
        SensorReading::Error { code, message } => {
            Alert::Critical(format!("error {code}: {message}"))
        }
        SensorReading::Idle => Alert::Nominal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        assert_eq!(
            classify(&SensorReading::Temperature(91.0)),
            Alert::Critical("temperature over 90".into())
        );
        assert_eq!(
            classify(&SensorReading::Error {
                code: 7,
                message: "offline".into()
            }),
            Alert::Critical("error 7: offline".into())
        );
        assert_eq!(classify(&SensorReading::Idle), Alert::Nominal);
    }
}
