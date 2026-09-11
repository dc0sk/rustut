// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md for the full task description.

use std::cell::Cell;

/// Return `Some(value)` if `index` is in bounds for `data`, `None`
/// otherwise. Must never panic and must never read out-of-bounds memory.
pub fn safe_get(data: &[i32], index: usize) -> Option<i32> {
    todo!("look up `index` in `data` without panicking or indexing directly")
}

/// A stand-in for a C resource (a file handle, a lock, a malloc'd buffer)
/// that must be released exactly once when it goes out of scope.
pub struct Resource<'a> {
    releases: &'a Cell<u32>,
}

impl<'a> Resource<'a> {
    pub fn new(releases: &'a Cell<u32>) -> Self {
        Resource { releases }
    }
}

impl Drop for Resource<'_> {
    fn drop(&mut self) {
        todo!(
            "increment *self.releases by one — this runs automatically, on every return path below"
        )
    }
}

/// Acquires a `Resource`, then either returns early or completes normally.
/// Either way, the `Resource` must be released exactly once.
pub fn use_resource(releases: &Cell<u32>, take_early_exit: bool) -> &'static str {
    let _guard = Resource::new(releases);
    if take_early_exit {
        return "early";
    }
    "normal"
}
