// SPDX-License-Identifier: MIT OR Apache-2.0
//! See README.md.
//!
//! Task: implement the strategy pattern — `Logger<S>` formats a message
//! and hands it to whichever `LogSink` it was built with, without knowing
//! or caring which one.

pub trait LogSink {
    fn write(&mut self, line: String);
}

/// A sink that records lines in memory instead of printing them, so tests
/// can inspect exactly what was logged.
#[derive(Default)]
pub struct VecSink {
    pub lines: Vec<String>,
}

impl LogSink for VecSink {
    fn write(&mut self, line: String) {
        todo!("push `line` onto self.lines")
    }
}

pub struct Logger<S: LogSink> {
    sink: S,
}

impl<S: LogSink> Logger<S> {
    pub fn new(sink: S) -> Self {
        Logger { sink }
    }

    /// Format `message` as `"[LOG] {message}"` and hand it to the sink.
    pub fn log(&mut self, message: &str) {
        todo!("format message as \"[LOG] {{message}}\" and call self.sink.write(...)")
    }

    /// Consumes the logger, returning its sink — used by tests to inspect
    /// what was logged. Already implemented; not part of the task.
    pub fn into_sink(self) -> S {
        self.sink
    }
}
