// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: module_tree
// `network.rs` alongside a `network/` directory: the modern (2018+) way to
// give a module children without needing a `mod.rs` file. `network` itself
// is `pub`, so outside code can dig into `network::http` directly if it
// wants to — Chapter 3's crate used a private `mod`; this one shows the
// public alternative.
pub mod network;

// Re-exporting the common type at the crate root alongside a `pub` module
// is a very common combination in real crates: casual callers use
// `Connection` directly, power users can still reach `network::http`.
pub use network::Connection;
// ANCHOR_END: module_tree

// ANCHOR: prelude
/// The one place a glob import (`use ...::prelude::*;`) is idiomatic: a
/// curated re-export module a caller opts into explicitly by name, rather
/// than an anonymous `use whatever::*;` that silently pulls in anything a
/// dependency adds later.
///
/// Note this re-export works even though it only re-exports items that
/// are *already* `pub` through `network` — visibility of a `pub use` is
/// governed by whether the re-exporting code can see the item, not by
/// adding any new access. It's purely an ergonomic flattening.
pub mod prelude {
    pub use crate::network::Connection;
    pub use crate::network::http::is_success;
}
// ANCHOR_END: prelude

// ANCHOR: pub_super_boundary
/// ```
/// use ch14_modules_and_visibility_example::prelude::*;
///
/// let c = Connection::new("example.com");
/// assert!(is_success(200));
/// assert_eq!(c.host, "example.com");
/// ```
///
/// `http::parse_status_line` is `pub(super)` (visible to `network`, its
/// parent module, only) — reachable from inside this crate's `network`
/// module, but not from the crate root, and this does NOT compile:
///
/// ```compile_fail
/// let _ = ch14_modules_and_visibility_example::network::http::parse_status_line("x");
/// ```
fn _doc_anchor() {}
// ANCHOR_END: pub_super_boundary
