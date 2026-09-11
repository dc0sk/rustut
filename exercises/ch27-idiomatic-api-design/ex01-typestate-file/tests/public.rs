// SPDX-License-Identifier: MIT OR Apache-2.0
use ch27_ex01_typestate_file::FileHandle;

#[test]
fn open_then_read_then_close() {
    let handle = FileHandle::new("notes.txt");
    let handle = handle.open();
    assert_eq!(handle.read(), "contents of notes.txt");
    let _handle = handle.close();
}

#[test]
fn reopening_after_close_works() {
    let handle = FileHandle::new("a.txt").open().close().open();
    assert_eq!(handle.read(), "contents of a.txt");
}
