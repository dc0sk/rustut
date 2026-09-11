<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 8.1 — shapes and vehicles

## Task 1 — `Rectangle`

Implement `Rectangle::new`, `area`, and `perimeter`.

## Task 2 — `Vehicle`

Implement `Vehicle::wheel_count`:

- `Car` has 4 wheels, regardless of `doors`.
- `Truck` has 2 wheels per axle.
- `Motorcycle` has 2 wheels.

## Done when

- `cargo test -p ch08-ex01-shapes-and-vehicles` passes
- `cargo clippy -p ch08-ex01-shapes-and-vehicles --all-targets --all-features -- -D warnings` is clean

## Learning goals

- Associated functions (`Self::new(...)`) vs. methods (`&self`).
- `#[derive(Debug, PartialEq)]` gets you a working `{:?}` and `==` for free
  — no hand-written print/compare function per struct, the way C requires.
- An enum's variants can carry entirely different payloads (`{ doors: u8
  }` vs. `{ axles: u8 }` vs. nothing at all), each type-checked
  independently — a tagged union the compiler enforces you handle
  exhaustively when you read it back with `match`.
