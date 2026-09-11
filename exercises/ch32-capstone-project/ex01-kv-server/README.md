<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Capstone — a concurrent, persistent key-value server

## Task

`src/lib.rs` gives you a complete, working shell: CLI wiring (`src/main.rs`),
a type-state `Server<Unbound>`/`Server<Bound>` that accepts connections and
spawns one thread per client, and a `Store` that (de)serializes to JSON.
What's missing — three `todo!()`s — is the part actually specific to a
key-value store:

1. `Store::get`/`Store::set`/`Store::delete` — the core map operations.
2. `parse_command` — turn one line of client text (`GET foo`, `SET foo bar
   baz`, `DELETE foo`) into a `Command`, rejecting anything malformed with
   an `Err` (never a panic).
3. `handle_command` — apply a `Command` to the shared, `Mutex`-protected
   store, persist it to disk on any mutation, and produce the response
   line.

Every doc comment on the three `todo!()`s spells out exactly what's
expected. Don't touch `src/main.rs` or anything in `impl Server<...>` —
they're complete.

## Done when

- `cargo test -p ch32-capstone-project` passes (four tests: single-client
  round trip, malformed-input handling, two concurrent clients sharing one
  store, and persistence-to-disk)
- `cargo clippy -p ch32-capstone-project --all-targets --all-features -- -D warnings` is clean

## Design Rubric

This exercise integrates most of the book at once, so beyond the tests
passing, check these explicitly:

- [ ] The store's shared state is behind a `Mutex` (or `RwLock`) — not an
      unsynchronized `static mut`, not per-connection copies that silently
      diverge.
- [ ] The server genuinely handles **multiple concurrent clients** — two
      `TcpStream`s connected at once, each able to see the other's writes.
- [ ] Malformed client input produces an `ERROR ...` response, never a
      panic that kills the connection (or the whole process, if it
      happened outside the per-connection thread).
- [ ] A mutation (`SET`/successful `DELETE`) is durably persisted — reading
      the JSON file directly (independent of the running server) shows the
      write.
- [ ] `Server::run()` is only callable on `Server<Bound>` — i.e. you
      structurally cannot start accepting connections before binding a
      socket. (This is already true in the given scaffolding; understand
      *why* it's true, since it's the chapter's typestate example.)
- [ ] No unjustified `unsafe`.

## Learning goals

This is the synthesis exercise for the whole book. Concretely, it pulls
together: CLI parsing and error handling (Ch. 23), file I/O (Ch. 24), JSON
serialization (Ch. 29), threads and shared mutable state behind a lock
(Ch. 18-19), a simple protocol over `std::net` with ephemeral-port testing
(Ch. 25), the typestate pattern (Ch. 27), and clippy/test hygiene (Ch. 30-31).
