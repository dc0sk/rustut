// SPDX-License-Identifier: MIT OR Apache-2.0
use ch09_ex01_classify_readings::{Alert, SensorReading, classify};

#[test]
fn temperature_critical_above_90() {
    assert_eq!(
        classify(&SensorReading::Temperature(91.0)),
        Alert::Critical("temperature over 90".into())
    );
}

#[test]
fn temperature_warning_between_75_and_90() {
    assert_eq!(
        classify(&SensorReading::Temperature(80.0)),
        Alert::Warning("temperature over 75".into())
    );
}

#[test]
fn temperature_nominal_at_or_below_75() {
    assert_eq!(classify(&SensorReading::Temperature(75.0)), Alert::Nominal);
    assert_eq!(classify(&SensorReading::Temperature(10.0)), Alert::Nominal);
}

#[test]
fn humidity_warning_above_95() {
    assert_eq!(
        classify(&SensorReading::Humidity(96.0)),
        Alert::Warning("humidity over 95".into())
    );
}

#[test]
fn humidity_nominal_at_or_below_95() {
    assert_eq!(classify(&SensorReading::Humidity(95.0)), Alert::Nominal);
}

#[test]
fn error_is_always_critical_with_formatted_message() {
    assert_eq!(
        classify(&SensorReading::Error {
            code: 7,
            message: "offline".into()
        }),
        Alert::Critical("error 7: offline".into())
    );
}

#[test]
fn idle_is_nominal() {
    assert_eq!(classify(&SensorReading::Idle), Alert::Nominal);
}
