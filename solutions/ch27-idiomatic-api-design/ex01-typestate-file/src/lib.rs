// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch27-idiomatic-api-design/ex01-typestate-file.

pub struct Closed;
pub struct Open;

pub struct FileHandle<State> {
    name: String,
    _state: std::marker::PhantomData<State>,
}

impl FileHandle<Closed> {
    pub fn new(name: &str) -> Self {
        FileHandle {
            name: name.to_string(),
            _state: std::marker::PhantomData,
        }
    }

    pub fn open(self) -> FileHandle<Open> {
        FileHandle {
            name: self.name,
            _state: std::marker::PhantomData,
        }
    }
}

impl FileHandle<Open> {
    pub fn read(&self) -> String {
        format!("contents of {}", self.name)
    }

    pub fn close(self) -> FileHandle<Closed> {
        FileHandle {
            name: self.name,
            _state: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let handle = FileHandle::new("notes.txt").open();
        assert_eq!(handle.read(), "contents of notes.txt");
        let _handle = handle.close();
    }
}
