// SPDX-License-Identifier: MIT OR Apache-2.0
use ch25_ex01_echo_line::{bind_ephemeral, handle_one_uppercased, send_and_receive};
use std::thread;

#[test]
fn uppercases_a_simple_word() {
    let (listener, port) = bind_ephemeral().unwrap();
    let server = thread::spawn(move || handle_one_uppercased(&listener).unwrap());

    let response = send_and_receive(port, "hello").unwrap();
    server.join().unwrap();

    assert_eq!(response.trim_end(), "HELLO");
}

#[test]
fn uppercases_mixed_case_with_punctuation() {
    let (listener, port) = bind_ephemeral().unwrap();
    let server = thread::spawn(move || handle_one_uppercased(&listener).unwrap());

    let response = send_and_receive(port, "Rust, meet C!").unwrap();
    server.join().unwrap();

    assert_eq!(response.trim_end(), "RUST, MEET C!");
}
