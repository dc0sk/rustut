// SPDX-License-Identifier: MIT OR Apache-2.0
//! Backing code for the Appendix cheat sheet. Every multi-line snippet the
//! cheat sheet shows as a code block is a real, compiled, tested anchor
//! here — the same anti-rot guarantee every chapter's guided examples use.
//! Single-line idioms are shown as inline code spans in the cheat sheet
//! text itself and don't need a backing anchor.

// ANCHOR: struct_and_derive
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
// ANCHOR_END: struct_and_derive

// ANCHOR: enum_and_match
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

pub fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
    }
}
// ANCHOR_END: enum_and_match

// ANCHOR: error_enum
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing field: {0}")]
    MissingField(String),
    #[error("invalid value for {field}: {value}")]
    InvalidValue { field: String, value: String },
}
// ANCHOR_END: error_enum

// ANCHOR: option_result_chain
pub fn first_even_doubled(values: &[i32]) -> Option<i32> {
    values.iter().find(|&&v| v % 2 == 0).map(|v| v * 2)
}

pub fn parse_and_validate(input: &str) -> Result<u32, String> {
    let n: u32 = input
        .parse()
        .map_err(|_| format!("not a number: {input}"))?;
    if n == 0 {
        return Err("must be nonzero".to_string());
    }
    Ok(n)
}
// ANCHOR_END: option_result_chain

// ANCHOR: iterator_chain
pub fn sum_of_squares_of_evens(values: &[i32]) -> i32 {
    values.iter().filter(|&&v| v % 2 == 0).map(|v| v * v).sum()
}
// ANCHOR_END: iterator_chain

// ANCHOR: builder_skeleton
#[derive(Default)]
pub struct RequestBuilder {
    url: Option<String>,
    timeout_ms: u32,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            url: None,
            timeout_ms: 5000,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn timeout_ms(mut self, ms: u32) -> Self {
        self.timeout_ms = ms;
        self
    }

    pub fn build(self) -> Result<(String, u32), &'static str> {
        Ok((self.url.ok_or("url is required")?, self.timeout_ms))
    }
}
// ANCHOR_END: builder_skeleton

// ANCHOR: thread_and_mutex
use std::sync::{Arc, Mutex};
use std::thread;

pub fn increment_concurrently(times: usize) -> u64 {
    let counter = Arc::new(Mutex::new(0u64));
    let handles: Vec<_> = (0..times)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                *counter.lock().unwrap() += 1;
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    *counter.lock().unwrap()
}
// ANCHOR_END: thread_and_mutex

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_and_derive_work() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = p1.clone();
        assert_eq!(p1, p2);
        assert_eq!(Point::default(), Point { x: 0, y: 0 });
    }

    #[test]
    fn enum_and_match_compute_area() {
        assert!((area(&Shape::Circle { radius: 2.0 }) - 12.566_370_614_359_172).abs() < 1e-9);
        assert_eq!(
            area(&Shape::Rectangle {
                width: 3.0,
                height: 4.0
            }),
            12.0
        );
    }

    #[test]
    fn error_enum_messages() {
        let e = ConfigError::MissingField("port".to_string());
        assert_eq!(e.to_string(), "missing field: port");
    }

    #[test]
    fn option_result_chain_works() {
        assert_eq!(first_even_doubled(&[1, 3, 4, 5]), Some(8));
        assert_eq!(first_even_doubled(&[1, 3, 5]), None);
        assert_eq!(parse_and_validate("42"), Ok(42));
        assert!(parse_and_validate("0").is_err());
        assert!(parse_and_validate("abc").is_err());
    }

    #[test]
    fn iterator_chain_sums_correctly() {
        assert_eq!(sum_of_squares_of_evens(&[1, 2, 3, 4, 5]), 20);
    }

    #[test]
    fn builder_skeleton_builds() {
        let (url, timeout) = RequestBuilder::new()
            .url("https://example.com")
            .build()
            .unwrap();
        assert_eq!(url, "https://example.com");
        assert_eq!(timeout, 5000);
        assert!(RequestBuilder::new().build().is_err());
    }

    #[test]
    fn thread_and_mutex_increments_exactly_n_times() {
        assert_eq!(increment_concurrently(50), 50);
    }
}
