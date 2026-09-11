// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: module_tree
// `mod warehouse;` pulls in src/warehouse.rs as a submodule — the module
// tree is defined by these declarations, not by which files happen to
// exist on disk (unlike a C build where every .c the Makefile lists gets
// compiled in).
mod warehouse;

// A private submodule's *type* can still be re-exported publicly: outside
// code sees `ch03_crate_anatomy_example::Crate` and never needs to know
// it's actually implemented in `warehouse.rs`.
pub use warehouse::Crate;
// ANCHOR_END: module_tree
