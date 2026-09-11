<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 18.1 — parallel sum

Implement `parallel_sum(data, num_workers)`: split `data` into
`num_workers` chunks, sum each chunk on its own thread, and add up the
partial sums — for the same result `data.iter().sum()` would give
sequentially, on any number of workers (including more workers than
elements, or an empty slice).

## Task

Use `std::thread::scope` so your worker threads can borrow slices of
`data` directly, with no cloning and no `Arc`. `<[T]>::chunks(n)` (from
`std::slice`) is a convenient way to split `data` into pieces.

## Done when

- `cargo test -p ch18-ex01-parallel-sum` passes
- `cargo clippy -p ch18-ex01-parallel-sum --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `thread::scope` lets threads borrow non-`'static` data safely, because
  the compiler guarantees every scoped thread is joined before the scope
  returns.
- Splitting work across threads and joining every handle before reading
  the result is the basic shape of most parallel-sum/map-reduce code —
  worth having in muscle memory before Chapter 19 adds shared mutable
  state to the picture.
