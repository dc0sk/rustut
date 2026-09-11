<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# C-to-Rust Glossary

A fast lookup from "the C thing I already know" to "the Rust thing that
plays the same role," with a pointer to the chapter that covers it
properly. This is a map, not a tutorial — if a row surprises you, follow
the chapter link rather than taking the one-liner as the whole story.

## Memory & ownership

| C | Rust | Chapter |
|---|---|---|
| `malloc`/`free` | An owned value; heap allocation via `Box<T>`, freed automatically when it's dropped | [Ch. 5](../part1-core-language/05-ownership.md), [Ch. 16](../part3-safety-concurrency/16-memory-safety-raii.md) |
| Manually calling `free()`/`fclose()` on every exit path | `Drop`, run automatically on scope exit, on every path including panics | [Ch. 1](../part0-orientation/01-why-rust.md), [Ch. 16](../part3-safety-concurrency/16-memory-safety-raii.md) |
| A pointer that outlives what it points to | Rejected at compile time by lifetime checking | [Ch. 7](../part1-core-language/07-lifetimes.md) |
| Two pointers to the same memory, no aliasing rule | The borrow checker: many `&T` *or* one `&mut T`, never both | [Ch. 6](../part1-core-language/06-borrowing-and-references.md) |
| `NULL` pointer | `Option<T>` — there's no way to use the `T` without unwrapping it first | [Ch. 10](../part1-core-language/10-option-result.md) |
| `memcpy`-based deep copy | `.clone()` (explicit opt-in); trivial types copy implicitly via `Copy` | [Ch. 5](../part1-core-language/05-ownership.md) |
| `void*` + manual casting for "any type" | Generics (`fn foo<T>(...)`), monomorphized, no runtime cost | [Ch. 12](../part2-abstraction/12-traits-and-generics.md) |
| A hand-rolled vtable (`struct { void (*run)(void*); }`) | `dyn Trait` — the same mechanism, but the compiler builds and checks the vtable for you | [Ch. 13](../part2-abstraction/13-trait-objects.md) |

## Types & control flow

| C | Rust | Chapter |
|---|---|---|
| `typedef double Meters;` (just an alias — no type safety) | A newtype (`struct Meters(f64);`) — a distinct type the compiler won't let you mix up with another | [Ch. 27](../part5-patterns-practice/27-idiomatic-api-design.md) |
| `struct` | `struct` (named-field, tuple, or unit) | [Ch. 8](../part1-core-language/08-structs-and-enums.md) |
| `union` + a separate tag field, tracked by convention | `enum` — the compiler tracks the active variant; reading the wrong one is impossible in safe code | [Ch. 8](../part1-core-language/08-structs-and-enums.md) |
| `switch` (a missing `case`/`default` silently does nothing) | `match` (exhaustive — the compiler rejects an unhandled variant) | [Ch. 9](../part1-core-language/09-pattern-matching.md) |
| errno / a negative return code | `Result<T, E>`, propagated with `?` | [Ch. 10](../part1-core-language/10-option-result.md), [Ch. 23](../part4-pc-applications/23-error-handling-and-cli.md) |
| `int arr[N]` (decays to a pointer, `sizeof` tricks for length) | `[T; N]` (fixed size, `.len()` always available) or `Vec<T>` (growable) | [Ch. 4](../part1-core-language/04-variables-mutability-types.md), [Ch. 11](../part1-core-language/11-collections.md) |
| A pointer + a separately-tracked length, by convention | A slice `&[T]` — a single fat-pointer value that carries its own length | [Ch. 6](../part1-core-language/06-borrowing-and-references.md), [Ch. 11](../part1-core-language/11-collections.md) |
| `char*` (bytes, no encoding guarantee) | `&str`/`String` — guaranteed valid UTF-8 | [Ch. 11](../part1-core-language/11-collections.md) |
| A function pointer field for "swappable behavior" | A generic with a trait bound (static dispatch) or `Box<dyn Trait>` (dynamic dispatch) — the strategy pattern | [Ch. 28](../part5-patterns-practice/28-design-patterns.md) |
| Implementing your own hash table | `HashMap<K, V>` | [Ch. 11](../part1-core-language/11-collections.md) |

## Build, linking & tooling

| C | Rust | Chapter |
|---|---|---|
| `make`/`CMakeLists.txt` | `cargo build` | [Ch. 2](../part0-orientation/02-toolchain-and-cargo.md) |
| `#include "foo.h"` (textual paste, no compiler-checked visibility) | `mod`/`use` — a real, compiler-checked module tree | [Ch. 3](../part0-orientation/03-crate-anatomy.md), [Ch. 14](../part2-abstraction/14-modules-and-visibility.md) |
| `static` at file scope (visibility by convention) | Private by default (no keyword needed); `pub(crate)`/`pub` to widen it, enforced by the compiler | [Ch. 3](../part0-orientation/03-crate-anatomy.md) |
| Vendoring a `.tar.gz` into `third_party/`, or apt/vcpkg | `[dependencies]` in `Cargo.toml`, fetched from crates.io | [Ch. 2](../part0-orientation/02-toolchain-and-cargo.md) |
| Pinning a dependency version by hand and hoping | `Cargo.lock` — the exact resolved version of every dependency, committed | [Ch. 2](../part0-orientation/02-toolchain-and-cargo.md) |
| `doxygen` | `///` doc comments + `cargo doc` (and its examples are `cargo test`-verified) | [Ch. 2](../part0-orientation/02-toolchain-and-cargo.md), [Ch. 30](../part5-patterns-practice/30-testing-and-documentation.md) |
| `clang-format` | `cargo fmt` | [Ch. 2](../part0-orientation/02-toolchain-and-cargo.md) |
| `clang-tidy`/`cppcheck` | `cargo clippy` | [Ch. 2](../part0-orientation/02-toolchain-and-cargo.md) |
| Unity/CMock/Check | `#[test]`, `cargo test` (built into the toolchain, not bolted on) | [Ch. 30](../part5-patterns-practice/30-testing-and-documentation.md) |
| Manually cross-referencing CVEs against vendored code | `cargo audit`/`cargo-deny` against `Cargo.lock` | [Ch. 31](../part5-patterns-practice/31-security-best-practices.md) |

## Concurrency

| C | Rust | Chapter |
|---|---|---|
| `pthread_create`/`pthread_join` | `std::thread::spawn`/`JoinHandle::join` | [Ch. 18](../part3-safety-concurrency/18-threads.md) |
| `pthread_mutex_t` + a shared struct, linked only by convention | `Mutex<T>` — the data is *inside* the mutex; you cannot reach it without locking | [Ch. 19](../part3-safety-concurrency/19-send-sync.md) |
| A hand-rolled queue + `pthread_cond_t` for producer/consumer | A channel (`std::sync::mpsc`) | [Ch. 20](../part3-safety-concurrency/20-channels.md) |
| `<stdatomic.h>` (`_Atomic`, `memory_order_*`) | `std::sync::atomic` — the same C11/C++11 memory model, same ordering names | [Ch. 21](../part3-safety-concurrency/21-atomics.md) |
| A hand-rolled `epoll`/`select` event loop | `async`/`await` + a runtime (e.g. `tokio`) | [Ch. 22](../part3-safety-concurrency/22-async-await.md) |
| A data race caught (if you're lucky) by TSan at runtime | `Send`/`Sync`, checked at compile time | [Ch. 19](../part3-safety-concurrency/19-send-sync.md) |

## Not covered in this book

A few C-adjacent topics this book's general-purpose-Rust scope doesn't
reach — see [Further Reading](further-reading.md) for where to go next:

- `volatile` reads/writes, memory-mapped I/O, and interrupt handlers (the
  embedded/`no_std` side of Rust).
- Declarative macros (`macro_rules!`) as a `#define`-with-hygiene
  replacement — mentioned in passing, never built.
- `#ifdef`-style conditional compilation in depth (`#[cfg(...)]` is used
  throughout for `#[cfg(test)]`, but never explained as its own topic).
