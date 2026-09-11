// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 23 — application error handling & CLI
//! foundations.

// ANCHOR: thiserror_error
use thiserror::Error;

/// A library-level error type: one variant per distinct failure mode,
/// each with its own message. Contrast with C, where every fallible
/// function invents its own ad-hoc "what does -1 mean here" convention
/// that every caller has to remember and look up — often in a comment,
/// if you're lucky.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required field `{0}`")]
    MissingField(&'static str),
    #[error("field `{field}` must be a positive integer, got `{value}`")]
    InvalidPort { field: &'static str, value: String },
}

/// Parses a tiny "key=value" config line into a validated port number.
pub fn parse_port_line(line: &str) -> Result<u16, ConfigError> {
    let (key, value) = line
        .split_once('=')
        .ok_or(ConfigError::MissingField("port"))?;
    if key != "port" {
        return Err(ConfigError::MissingField("port"));
    }
    value
        .trim()
        .parse::<u16>()
        .map_err(|_| ConfigError::InvalidPort {
            field: "port",
            value: value.to_string(),
        })
}
// ANCHOR_END: thiserror_error

// ANCHOR: anyhow_context
use anyhow::{Context, Result};

/// Application-level code (a CLI's `main`, or anything close to it)
/// typically doesn't want a bespoke error enum for every possible
/// failure — `anyhow::Error` is a single catch-all type that can hold
/// *any* error, and `.context(...)` lets you attach a human-readable
/// explanation of what you were trying to do when it failed, without
/// writing a wrapper type for it.
pub fn load_port(config_line: &str) -> Result<u16> {
    parse_port_line(config_line).context("failed to read the `port` setting from configuration")
}
// ANCHOR_END: anyhow_context

// ANCHOR: clap_cli
use clap::Parser;

/// `#[derive(Parser)]` generates an entire `--help`/`--version`-capable
/// argument parser from this struct's fields — compare to C, where
/// `getopt`/`getopt_long` still leaves you hand-writing a switch
/// statement over option characters and manually converting each
/// argument string to its real type.
#[derive(Parser, Debug)]
#[command(name = "portcheck", about = "Validates a port config line")]
pub struct Cli {
    /// The "key=value" config line to validate, e.g. "port=8080"
    pub config_line: String,

    /// Print extra detail on failure
    #[arg(short, long)]
    pub verbose: bool,
}
// ANCHOR_END: clap_cli

// ANCHOR: exit_code_pattern
/// A typical CLI `main` shape: do the fallible work, and on failure print
/// a clean message to stderr and return a nonzero process exit code —
/// never a panic (a stack-trace dump) for an *expected* failure like bad
/// user input. Contrast with C, where `main`'s only two real options are
/// "return an int" or `abort()`/an unhandled signal — there's no
/// in-between "clean, structured failure" story built into the language.
///
/// This function itself never calls `std::process::exit` (that would
/// terminate the process immediately, making it untestable) — it returns
/// the exit code `main` should use. A real `fn main() -> ExitCode` would
/// call this and do `std::process::ExitCode::from(run(&line))`.
pub fn run(config_line: &str) -> u8 {
    match load_port(config_line) {
        Ok(port) => {
            println!("valid port: {port}");
            0
        }
        Err(e) => {
            // The `{e:#}` alternate form prints anyhow's full context
            // chain, not just the innermost message.
            eprintln!("error: {e:#}");
            1
        }
    }
}
// ANCHOR_END: exit_code_pattern

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_port() {
        assert_eq!(parse_port_line("port=8080").unwrap(), 8080);
    }

    #[test]
    fn rejects_a_non_numeric_port() {
        assert!(matches!(
            parse_port_line("port=not-a-number"),
            Err(ConfigError::InvalidPort { .. })
        ));
    }

    #[test]
    fn rejects_a_missing_field() {
        assert!(matches!(
            parse_port_line("nonsense"),
            Err(ConfigError::MissingField(_))
        ));
    }

    #[test]
    fn load_port_wraps_the_error_with_context() {
        let err = load_port("nonsense").unwrap_err();
        // anyhow's Display chains the context onto the underlying error.
        assert!(format!("{err:#}").contains("failed to read the `port` setting"));
    }

    #[test]
    fn cli_parses_from_argv() {
        let cli = Cli::parse_from(["portcheck", "port=80", "--verbose"]);
        assert_eq!(cli.config_line, "port=80");
        assert!(cli.verbose);
    }

    #[test]
    fn run_returns_zero_on_success() {
        assert_eq!(run("port=8080"), 0);
    }

    #[test]
    fn run_returns_nonzero_on_failure() {
        assert_eq!(run("garbage"), 1);
    }
}
