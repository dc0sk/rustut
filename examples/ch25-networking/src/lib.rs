// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 25 — a minimal blocking TCP echo server and
//! client, built entirely on `std::net`.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

// ANCHOR: bind_ephemeral
/// Binds to an OS-assigned port on localhost and returns the listener plus
/// the port that was actually assigned — never hardcode a port number in a
/// test, or two test runs (or two CI jobs) racing each other will collide.
pub fn bind_ephemeral() -> std::io::Result<(TcpListener, u16)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok((listener, port))
}
// ANCHOR_END: bind_ephemeral

// ANCHOR: echo_one
/// Accepts exactly one connection, reads one line, and writes it straight
/// back. `TcpStream` closes its socket when it's dropped (Ch. 16's RAII
/// guarantee again) — there is no `close(fd)` to forget, unlike raw BSD
/// sockets in C.
pub fn echo_one(listener: &TcpListener) -> std::io::Result<()> {
    let (stream, _addr) = listener.accept()?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let mut writer = stream;
    writer.write_all(line.as_bytes())?;
    Ok(())
}
// ANCHOR_END: echo_one

// ANCHOR: client_roundtrip
/// Connects to `port` on localhost, sends `message` (with a trailing
/// newline so the server's `read_line` has a terminator), and reads back
/// whatever the server sends in response.
pub fn client_roundtrip(port: u16, message: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))?;
    writeln!(stream, "{message}")?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    Ok(response)
}
// ANCHOR_END: client_roundtrip

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn echoes_the_line_sent_by_the_client() {
        let (listener, port) = bind_ephemeral().unwrap();
        let server = thread::spawn(move || echo_one(&listener).unwrap());

        let response = client_roundtrip(port, "hello, socket").unwrap();
        server.join().unwrap();

        assert_eq!(response.trim_end(), "hello, socket");
    }
}
