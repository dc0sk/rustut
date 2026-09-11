<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Networking Basics

## Sockets, the same syscalls underneath

`std::net::{TcpListener, TcpStream, UdpSocket}` are not a new networking
model — under the hood, they're the same `socket()`/`bind()`/`listen()`/
`accept()`/`connect()` syscalls a C program would make against BSD
sockets. What Rust changes is what you write on top of them:

| C (BSD sockets) | Rust (`std::net`) |
|---|---|
| `socket()`, then `bind()`, then `listen()` | `TcpListener::bind(addr)` |
| `accept()` returning a raw `int` fd | `listener.accept()` returning a `(TcpStream, SocketAddr)` |
| manually filling a `struct sockaddr_in` | `"127.0.0.1:8080".parse()` or a `(host, port)` tuple |
| `close(fd)` — easy to forget on an error path | `Drop` closes the socket automatically (Ch. 16) |
| `read()`/`write()` on a raw fd, checking `errno` | `Read`/`Write` trait methods returning `io::Result<T>` |

> **C engineer's mental model.** Nothing about the underlying network
> protocol changed. What changed is that "did I close this socket on
> every exit path" and "did I fill in the sockaddr struct correctly" stop
> being your problem.

## A minimal blocking echo server

```rust,ignore
{{#include ../../examples/ch25-networking/src/lib.rs:bind_ephemeral}}
```

Binding to port `0` asks the OS to assign any free port — essential for
tests (and good practice generally): a hardcoded port number means your
test suite can't run twice at once, in CI or locally, without a collision.

```rust,ignore
{{#include ../../examples/ch25-networking/src/lib.rs:echo_one}}
```

```rust,ignore
{{#include ../../examples/ch25-networking/src/lib.rs:client_roundtrip}}
```

Notice there's no `close()` anywhere in either function — `TcpStream` and
`TcpListener` release their underlying file descriptor in their `Drop`
impl, the same RAII guarantee Chapter 16 covered for any other resource.

## Where async fits in

Chapter 22 covered `async`/`await` and `tokio`. `tokio::net::{TcpListener,
TcpStream}` are the async equivalents of exactly the types above, with the
same method names — the reason to reach for them is handling many
concurrent connections without spawning one OS thread per connection
(Ch. 18's thread-per-connection model works, but doesn't scale to tens of
thousands of idle connections the way a handful of async tasks on a small
thread pool does). This chapter deliberately stays on blocking `std::net`:
it's the right place to actually see what a socket *is*, before adding
async's scheduling model on top of it. Once you need it, swapping
`std::net::TcpListener` for `tokio::net::TcpListener` and adding `.await`
after `.accept()` is a small, mechanical change, not a redesign.

## Exercise

**Exercise 25.1**, in
[`exercises/ch25-networking/ex01-echo-line/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch25-networking/ex01-echo-line),
has you implement an uppercase-echo server building on the same
ephemeral-port pattern this chapter used. See its `README.md`.

Next: [Chapter 26 — Interop with C](26-interop-with-c.md).
