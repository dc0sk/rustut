// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

/// Binds to an OS-assigned port on localhost. Returns the listener and the
/// port that was actually assigned.
pub fn bind_ephemeral() -> std::io::Result<(TcpListener, u16)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok((listener, port))
}

/// Accepts exactly one connection, reads one line, and writes back that
/// line converted to uppercase (with a trailing newline).
pub fn handle_one_uppercased(listener: &TcpListener) -> std::io::Result<()> {
    todo!("accept a connection, read one line, write back its uppercase form")
}

/// Connects to `port` on localhost, sends `message` (the caller does not
/// need to add a newline), and reads back one line of response.
pub fn send_and_receive(port: u16, message: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))?;
    writeln!(stream, "{message}")?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    Ok(response)
}
