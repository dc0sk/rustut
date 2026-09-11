// SPDX-License-Identifier: MIT OR Apache-2.0
use ch14_ex01_visibility_boundaries::logging::log_line;
use ch14_ex01_visibility_boundaries::prelude::log_line as log_line_via_prelude;

#[test]
fn formats_hh_mm_ss_zero_padded() {
    assert_eq!(log_line(0, "boot"), "[00:00:00] boot");
    assert_eq!(
        log_line(3661, "one hour, one minute, one second"),
        "[01:01:01] one hour, one minute, one second"
    );
}

#[test]
fn prelude_reexport_reaches_the_same_function() {
    assert_eq!(
        log_line_via_prelude(59, "almost a minute"),
        "[00:00:59] almost a minute"
    );
}
