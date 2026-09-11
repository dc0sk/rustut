// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 27 — builder, newtype, and typestate.

// ANCHOR: builder
/// A required field (`url`) is only checked at `build()` time here — the
/// simplest correct design. Compare to C's usual options for "construct a
/// complex value step by step": a giant struct literal with designated
/// initializers (nothing stops you from forgetting a field — it just
/// silently zero-initializes), or a `memset` + "set the fields you care
/// about" convention with no compiler help distinguishing "intentionally
/// left at its default" from "forgot to set this."
#[derive(Debug, Default)]
pub struct RequestBuilder {
    url: Option<String>,
    method: String,
    header: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            method: "GET".to_string(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    #[must_use]
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }

    #[must_use]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.header.push((key.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<Request, &'static str> {
        Ok(Request {
            url: self.url.ok_or("url is required")?,
            method: self.method,
            headers: self.header,
        })
    }
}
// ANCHOR_END: builder

// ANCHOR: newtype
/// Two newtypes wrapping the same primitive (`f64`) are still, to the
/// compiler, completely different types. Compare to C's `typedef double
/// Meters; typedef double Seconds;` — a `typedef` is *only* an alias, so a
/// `Seconds` value passes silently into a parameter typed `Meters`, zero
/// complaint from the compiler. These two cannot be swapped by accident:
///
/// ```rust,compile_fail
/// # use ch27_idiomatic_api_design_example::{Meters, Seconds, speed};
/// let distance = Seconds(9.58); // wrong newtype, right underlying type
/// let time = Meters(100.0);
/// speed(distance, time); // rejected: `speed` wants (Meters, Seconds)
/// ```
///
/// The real compiler output (rustc 1.98, stable):
///
/// ```text
/// error[E0308]: mismatched types
///   |
///   |     speed(distance, time);
///   |           ^^^^^^^^ expected `Meters`, found `Seconds`
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Meters(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Seconds(pub f64);

pub fn speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0
}
// ANCHOR_END: newtype

// ANCHOR: typestate
/// Typestate: protocol state lives in the *type*, not a runtime flag, so
/// calling a method in the wrong state is a compile error rather than a
/// panic or an `if !self.connected { return Err(...) }` check you have to
/// remember to write at the top of every method.
pub struct Disconnected;
pub struct Connected;

pub struct Connection<State> {
    address: String,
    _state: std::marker::PhantomData<State>,
}

impl Connection<Disconnected> {
    pub fn new(address: impl Into<String>) -> Self {
        Connection {
            address: address.into(),
            _state: std::marker::PhantomData,
        }
    }

    /// Consumes the disconnected connection, returns a connected one — the
    /// old, unusable-for-sending value is gone; there is no lingering
    /// handle still typed `Connection<Disconnected>` that a caller could
    /// mistakenly call `.send()` on.
    pub fn connect(self) -> Connection<Connected> {
        Connection {
            address: self.address,
            _state: std::marker::PhantomData,
        }
    }
}

impl Connection<Connected> {
    /// Only exists on `Connection<Connected>` — there is no `send` method
    /// to even attempt calling on a `Connection<Disconnected>`:
    ///
    /// ```rust,compile_fail
    /// # use ch27_idiomatic_api_design_example::Connection;
    /// let conn = Connection::new("localhost:9000");
    /// conn.send("hello"); // no method `send` on Connection<Disconnected>
    /// ```
    pub fn send(&self, message: &str) -> String {
        format!("sent {message:?} to {}", self.address)
    }

    pub fn disconnect(self) -> Connection<Disconnected> {
        Connection {
            address: self.address,
            _state: std::marker::PhantomData,
        }
    }
}
// ANCHOR_END: typestate

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_requires_url() {
        assert_eq!(RequestBuilder::new().build(), Err("url is required"));
    }

    #[test]
    fn builder_builds_a_complete_request() {
        let req = RequestBuilder::new()
            .url("https://example.com")
            .method("POST")
            .header("Content-Type", "text/plain")
            .build()
            .unwrap();
        assert_eq!(req.url, "https://example.com");
        assert_eq!(req.method, "POST");
        assert_eq!(
            req.headers,
            vec![("Content-Type".to_string(), "text/plain".to_string())]
        );
    }

    #[test]
    fn newtypes_keep_units_straight() {
        assert_eq!(speed(Meters(100.0), Seconds(10.0)), 10.0);
    }

    #[test]
    fn typestate_connection_round_trips() {
        let conn = Connection::new("localhost:9000").connect();
        assert_eq!(conn.send("hi"), "sent \"hi\" to localhost:9000");
        let _disconnected = conn.disconnect();
    }
}
