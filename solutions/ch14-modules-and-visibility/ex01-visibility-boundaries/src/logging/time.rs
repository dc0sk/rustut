// SPDX-License-Identifier: MIT OR Apache-2.0
pub(super) fn format_timestamp(seconds: u64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    format!("{h:02}:{m:02}:{s:02}")
}
