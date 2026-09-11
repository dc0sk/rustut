// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch25-networking/ex01-echo-line.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub fn bind_ephemeral() -> std::io::Result<(TcpListener, u16)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok((listener, port))
}

pub fn handle_one_uppercased(listener: &TcpListener) -> std::io::Result<()> {
    let (stream, _addr) = listener.accept()?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let mut writer = stream;
    writeln!(writer, "{}", line.trim_end().to_uppercase())?;
    Ok(())
}

pub fn send_and_receive(port: u16, message: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))?;
    writeln!(stream, "{message}")?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn matches_public_test_expectations() {
        let (listener, port) = bind_ephemeral().unwrap();
        let server = thread::spawn(move || handle_one_uppercased(&listener).unwrap());
        let response = send_and_receive(port, "hello").unwrap();
        server.join().unwrap();
        assert_eq!(response.trim_end(), "HELLO");
    }
}
