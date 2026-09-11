<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 13.1 — shape collection

`Circle` and `Rectangle` are unrelated types that only share the `HasArea`
trait. Implement `total_area` so it sums the area of a heterogeneous
`Vec<Box<dyn HasArea>>` — a collection that could never exist as a plain
`Vec<Circle>` or `Vec<Rectangle>`, the same way a C array holding a mix of
`struct shape_vtable *` pointers could hold different concrete shapes
behind a shared interface.

## Task

Fill in `total_area` in `src/lib.rs`. Don't match on the concrete type or
downcast — dispatch purely through `.area()`.

## Done when

- `cargo test -p ch13-ex01-shape-collection` passes
- `cargo clippy -p ch13-ex01-shape-collection --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `Box<dyn Trait>` is how you store genuinely different concrete types
  behind one shared interface in a single collection — something a
  generic function bounded by a trait (Chapter 12) cannot do, since a
  generic gets monomorphized to exactly one concrete type per call site.
- Each `.area()` call here costs one indirect (vtable) call at runtime —
  the price of not knowing the concrete type until runtime, in exchange
  for being able to mix types at all.
