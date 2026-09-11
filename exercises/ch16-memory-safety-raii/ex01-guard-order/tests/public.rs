// SPDX-License-Identifier: MIT OR Apache-2.0
use ch16_ex01_guard_order::{Log, nested_scopes};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn inner_scope_drops_before_the_outer_guard() {
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    nested_scopes(log.clone());
    assert_eq!(*log.borrow(), vec!["inner", "outer"]);
}

#[test]
fn each_call_gets_an_independent_log() {
    let log_a: Log = Rc::new(RefCell::new(Vec::new()));
    let log_b: Log = Rc::new(RefCell::new(Vec::new()));
    nested_scopes(log_a.clone());
    assert_eq!(log_b.borrow().len(), 0);
    assert_eq!(log_a.borrow().len(), 2);
}
