// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.
//!
//! `src/inventory.rs` already exists and is complete — it just isn't
//! wired into this crate yet, and its public type isn't reachable the way
//! callers (and `tests/public.rs`) expect.

// TODO 1: declare the `inventory` module (its file already exists at
// src/inventory.rs — a `mod` declaration is what makes it part of this
// crate's module tree; the file existing on disk is not enough).

// TODO 2: re-export `inventory::Item` at the crate root, so callers can
// write `ch03_ex01_module_visibility::Item` instead of reaching into
// `ch03_ex01_module_visibility::inventory::Item`.
