// SPDX-License-Identifier: MIT OR Apache-2.0
//! Complete — nothing to fix in this file. The exercise is in
//! `src/logging/time.rs` (implement the body) and `src/lib.rs` (add the
//! prelude re-export).

pub mod time;

use time::format_timestamp;

pub fn log_line(seconds: u64, message: &str) -> String {
    format!("[{}] {message}", format_timestamp(seconds))
}
