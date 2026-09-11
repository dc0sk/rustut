// SPDX-License-Identifier: MIT OR Apache-2.0
//! Exercise: redesign this runtime-checked file handle as a typestate API.
//! See README.md for the full task and Design Rubric.
//!
//! This naive version compiles and "works," but every method that cares
//! about open/closed state needs an `if` guard and a panic message that
//! only a test (or a user, in production) discovers by running it —
//! exactly the C pattern of a `bool is_open` flag checked by convention
//! at the top of every function that touches the handle.

pub struct NaiveFileHandle {
    name: String,
    open: bool,
}

impl NaiveFileHandle {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            open: false,
        }
    }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn read(&self) -> String {
        if !self.open {
            panic!("cannot read a closed file handle");
        }
        format!("contents of {}", self.name)
    }

    pub fn close(&mut self) {
        self.open = false;
    }
}

// TODO (this is the exercise): delete `NaiveFileHandle` above and replace
// it with a typestate design exposing exactly this shape (tests/api_shape.rs
// checks it mechanically):
//
//   pub struct Closed;
//   pub struct Open;
//   pub struct FileHandle<State> { /* ... */ }
//
//   impl FileHandle<Closed> {
//       pub fn new(name: &str) -> Self { /* ... */ }
//       pub fn open(self) -> FileHandle<Open> { /* ... */ }
//   }
//
//   impl FileHandle<Open> {
//       pub fn read(&self) -> String { /* ... */ } // "contents of {name}"
//       pub fn close(self) -> FileHandle<Closed> { /* ... */ }
//   }
//
// `FileHandle<Closed>` must NOT have a `read` method at all — not one
// that panics, one that does not exist — so that calling `.read()` before
// `.open()` is a compile error. See README.md's Design Rubric.
