<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Exercise 25.1 — uppercase-echo server

Implement `handle_one_uppercased`: accept exactly one connection on the
given listener, read one line from it, and write back that line converted
to uppercase (with a trailing newline so the client's `read_line` gets a
terminator).

## Done when

- `cargo test -p ch25-ex01-echo-line` passes
- `cargo clippy -p ch25-ex01-echo-line --all-targets --all-features -- -D warnings` is clean

## Learning goals

- `TcpListener`/`TcpStream` are ordinary `std` types with RAII cleanup — no
  `close(fd)` to remember, unlike raw BSD sockets in C.
- Tests bind to an OS-assigned ephemeral port (`127.0.0.1:0`) and read back
  the real port with `.local_addr()` — never hardcode a port number, or
  concurrent test runs will collide on it.
