// SPDX-License-Identifier: MIT OR Apache-2.0
//! Compiles only if the typestate API has exactly the shape the README's
//! Design Rubric requires — the mechanical half of an otherwise
//! open-ended design exercise. Never executed as a meaningful test (the
//! function is never called); its only job is to fail to *compile* if the
//! shape is wrong.
#![allow(dead_code)]

use ch27_ex01_typestate_file::{Closed, FileHandle, Open};

fn shape_check() {
    let h: FileHandle<Closed> = FileHandle::new("x");
    let h: FileHandle<Open> = h.open();
    let _s: String = h.read();
    let _h2: FileHandle<Closed> = h.close();
}
