// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: nested_and_visibility
pub mod http;

pub struct Connection {
    pub host: String,
}

impl Connection {
    pub fn new(host: impl Into<String>) -> Self {
        Connection { host: host.into() }
    }

    /// Calls into `http`'s `pub(super)` helper — allowed because this
    /// module (`network`) *is* `http`'s parent. There's no real C
    /// analogue for this granularity: `static` is all-or-nothing per
    /// translation unit, with nothing between "this file only" and
    /// "every file that links against it."
    pub fn parse_status(&self, line: &str) -> Option<u16> {
        http::parse_status_line(line)
    }
}
// ANCHOR_END: nested_and_visibility

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_via_pub_super_helper() {
        let c = Connection::new("example.com");
        assert_eq!(c.parse_status("HTTP/1.1 200 OK"), Some(200));
    }
}
