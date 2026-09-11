// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

use std::cell::RefCell;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<&'static str>>>;

pub struct RecordingGuard {
    name: &'static str,
    log: Log,
}

impl RecordingGuard {
    pub fn new(name: &'static str, log: Log) -> Self {
        RecordingGuard { name, log }
    }
}

impl Drop for RecordingGuard {
    fn drop(&mut self) {
        todo!("push self.name onto the shared log")
    }
}

/// Creates an "outer" guard, then inside a nested block creates an
/// "inner" guard. Don't add any explicit drop/cleanup calls — the point
/// of the exercise is that scope alone determines the order.
pub fn nested_scopes(log: Log) {
    todo!(
        "create a RecordingGuard named \"outer\" bound to `_outer`, then a \
         nested block containing a RecordingGuard named \"inner\" bound to `_inner`"
    )
}
