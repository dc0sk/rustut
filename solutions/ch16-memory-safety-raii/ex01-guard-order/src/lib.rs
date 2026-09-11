// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch16-memory-safety-raii/ex01-guard-order.

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
        self.log.borrow_mut().push(self.name);
    }
}

pub fn nested_scopes(log: Log) {
    let _outer = RecordingGuard::new("outer", log.clone());
    {
        let _inner = RecordingGuard::new("inner", log.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        nested_scopes(log.clone());
        assert_eq!(*log.borrow(), vec!["inner", "outer"]);
    }
}
