// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: pub_super
/// Visible only to `network` (this module's parent) — not reachable from
/// the crate root, and not reachable from outside the crate at all.
pub(super) fn parse_status_line(line: &str) -> Option<u16> {
    line.split_whitespace().nth(1)?.parse().ok()
}
// ANCHOR_END: pub_super

/// Fully public: reachable as `network::http::is_success` from anywhere
/// `network` itself is reachable from.
pub fn is_success(code: u16) -> bool {
    (200..300).contains(&code)
}
