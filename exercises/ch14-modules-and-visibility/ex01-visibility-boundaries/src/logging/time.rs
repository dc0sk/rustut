// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

/// Kept `pub(super)` on purpose: only `logging` (this module's parent)
/// should format timestamps directly — callers outside `logging` use
/// `log_line` instead. Don't change this visibility; implement the body.
pub(super) fn format_timestamp(seconds: u64) -> String {
    todo!("format `seconds` since midnight as zero-padded HH:MM:SS")
}
