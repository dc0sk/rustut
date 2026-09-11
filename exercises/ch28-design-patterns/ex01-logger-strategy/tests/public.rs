// SPDX-License-Identifier: MIT OR Apache-2.0
use ch28_ex01_logger_strategy::{Logger, VecSink};

#[test]
fn logs_are_formatted_and_forwarded_to_the_sink() {
    let mut logger = Logger::new(VecSink::default());
    logger.log("starting up");
    logger.log("ready");
    assert_eq!(
        logger.into_sink().lines,
        vec!["[LOG] starting up", "[LOG] ready"]
    );
}
