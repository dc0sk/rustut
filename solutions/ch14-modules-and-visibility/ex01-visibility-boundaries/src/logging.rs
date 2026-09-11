// SPDX-License-Identifier: MIT OR Apache-2.0
pub mod time;

use time::format_timestamp;

pub fn log_line(seconds: u64, message: &str) -> String {
    format!("[{}] {message}", format_timestamp(seconds))
}
