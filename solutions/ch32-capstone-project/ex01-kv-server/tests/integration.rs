// SPDX-License-Identifier: MIT OR Apache-2.0
//! End-to-end test: real TCP connections against a real running server.

use ch32_capstone_project_solution::{Server, ServerConfig};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn start_test_server() -> (std::net::SocketAddr, std::path::PathBuf, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let data_file = dir.path().join("store.json");
    let config = ServerConfig {
        listen_addr: "127.0.0.1:0".to_string(),
        data_file: data_file.clone(),
    };
    let server = Server::new(config)
        .bind()
        .expect("bind should succeed on an ephemeral port");
    let addr = server
        .local_addr()
        .expect("bound server has a local address");
    thread::spawn(move || {
        // Test-only: run() blocks forever: fine, the process exits at the
        // end of the test binary and takes this thread with it.
        let _ = server.run();
    });
    (addr, data_file, dir)
}

fn send(stream: &mut TcpStream, line: &str) -> String {
    writeln!(stream, "{line}").unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    response.trim_end().to_string()
}

#[test]
fn single_client_set_get_delete_round_trip() {
    let (addr, _data_file, _dir) = start_test_server();
    let mut client = TcpStream::connect(addr).expect("connect to freshly-bound server");

    assert_eq!(send(&mut client, "GET missing"), "NOT_FOUND");
    assert_eq!(send(&mut client, "SET greeting hello world"), "OK");
    assert_eq!(send(&mut client, "GET greeting"), "VALUE hello world");
    assert_eq!(send(&mut client, "DELETE greeting"), "OK");
    assert_eq!(send(&mut client, "GET greeting"), "NOT_FOUND");
    assert_eq!(send(&mut client, "DELETE greeting"), "NOT_FOUND");
}

#[test]
fn malformed_input_gets_an_error_response_not_a_dropped_connection() {
    let (addr, _data_file, _dir) = start_test_server();
    let mut client = TcpStream::connect(addr).unwrap();

    let response = send(&mut client, "FROBNICATE something");
    assert!(
        response.starts_with("ERROR"),
        "expected an ERROR response, got {response:?}"
    );

    // The connection must still be alive and usable after a bad command.
    assert_eq!(send(&mut client, "SET k v"), "OK");
    assert_eq!(send(&mut client, "GET k"), "VALUE v");
}

#[test]
fn two_concurrent_clients_share_one_store() {
    let (addr, _data_file, _dir) = start_test_server();
    let mut client_a = TcpStream::connect(addr).unwrap();
    let mut client_b = TcpStream::connect(addr).unwrap();

    assert_eq!(send(&mut client_a, "SET shared from-a"), "OK");
    // Client B, a separate connection/thread, must see client A's write —
    // this is only true because the store is behind one shared Mutex, not
    // one-per-connection state.
    assert_eq!(send(&mut client_b, "GET shared"), "VALUE from-a");

    assert_eq!(send(&mut client_b, "SET shared from-b"), "OK");
    assert_eq!(send(&mut client_a, "GET shared"), "VALUE from-b");
}

#[test]
fn mutations_are_persisted_to_disk() {
    let (addr, data_file, _dir) = start_test_server();
    let mut client = TcpStream::connect(addr).unwrap();
    assert_eq!(send(&mut client, "SET durable yes"), "OK");

    // Read the persisted file directly, independent of the live server, to
    // confirm SET actually reached disk rather than only living in memory.
    let on_disk = std::fs::read_to_string(&data_file).expect("store file should exist after a SET");
    assert!(
        on_disk.contains("durable"),
        "persisted file should contain the written key: {on_disk}"
    );
    assert!(
        on_disk.contains("yes"),
        "persisted file should contain the written value: {on_disk}"
    );
}
