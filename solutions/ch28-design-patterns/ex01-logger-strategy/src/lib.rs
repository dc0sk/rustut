// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch28-design-patterns/ex01-logger-strategy.

pub trait LogSink {
    fn write(&mut self, line: String);
}

#[derive(Default)]
pub struct VecSink {
    pub lines: Vec<String>,
}

impl LogSink for VecSink {
    fn write(&mut self, line: String) {
        self.lines.push(line);
    }
}

pub struct Logger<S: LogSink> {
    sink: S,
}

impl<S: LogSink> Logger<S> {
    pub fn new(sink: S) -> Self {
        Logger { sink }
    }

    pub fn log(&mut self, message: &str) {
        self.sink.write(format!("[LOG] {message}"));
    }

    pub fn into_sink(self) -> S {
        self.sink
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_public_test_expectations() {
        let mut logger = Logger::new(VecSink::default());
        logger.log("starting up");
        logger.log("ready");
        assert_eq!(
            logger.into_sink().lines,
            vec!["[LOG] starting up", "[LOG] ready"]
        );
    }
}
