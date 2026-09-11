// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 24 — files & OS interaction.

// ANCHOR: write_and_read
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Writes `contents` to `path`, then reads it back — demonstrating that
/// `std::io::Error` converts into `anyhow::Error` via `?` exactly like any
/// other error type, no wrapping code needed (Chapter 23's `anyhow`
/// section carries over unchanged).
pub fn roundtrip(path: &Path, contents: &str) -> Result<String> {
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))?;
    let read_back =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(read_back)
    // `File` isn't even visible here: `fs::write`/`fs::read_to_string`
    // open, use, and drop a `File` internally — its `Drop` impl (Chapter
    // 16) closes the underlying OS handle the instant it goes out of
    // scope, on every path out of this function, including an early `?`
    // return — no `fclose` to remember.
}
// ANCHOR_END: write_and_read

// ANCHOR: buffered_io
use std::io::{BufRead, BufReader, Write};

/// Counts non-empty lines in a file using a `BufReader` rather than
/// reading the whole file into memory first — the Rust equivalent of a
/// C `FILE*` opened in `"r"` mode: `BufReader` batches the underlying
/// `read()` syscalls into larger chunks instead of one syscall per line.
pub fn count_nonempty_lines(path: &Path) -> Result<usize> {
    let file =
        fs::File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            count += 1;
        }
    }
    Ok(count)
}

/// Writes each of `lines` to `path`, one per line, via a `BufWriter`.
pub fn write_lines(path: &Path, lines: &[&str]) -> Result<()> {
    let file =
        fs::File::create(path).with_context(|| format!("failed to create {}", path.display()))?;
    let mut writer = std::io::BufWriter::new(file);
    for line in lines {
        writeln!(writer, "{line}")?;
    }
    Ok(())
}
// ANCHOR_END: buffered_io

// ANCHOR: env_vars
use std::env;

/// `std::env::var` vs. C's `getenv`: `getenv` returns `NULL` for "unset"
/// and gives no way to distinguish that from "set, but not valid text";
/// `env::var` returns a `Result` whose `Err` distinguishes exactly those
/// two cases (`VarError::NotPresent` vs. `VarError::NotUnicode`).
pub fn resolve_output_dir() -> String {
    env::var("RUSTUT_OUTPUT_DIR").unwrap_or_else(|_| "./output".to_string())
}
// ANCHOR_END: env_vars

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn roundtrip_reads_back_what_was_written() {
        // A fresh tempdir per test: two tests writing to a fixed path like
        // "/tmp/test.txt" would race or clobber each other, especially
        // under `cargo test`'s default parallel test threads.
        let dir = tempdir().unwrap();
        let path = dir.path().join("scratch.txt");
        let result = roundtrip(&path, "hello, file").unwrap();
        assert_eq!(result, "hello, file");
    }

    #[test]
    fn counts_only_nonempty_lines() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("lines.txt");
        write_lines(&path, &["a", "", "b", "  ", "c"]).unwrap();
        assert_eq!(count_nonempty_lines(&path).unwrap(), 3);
    }

    #[test]
    fn missing_file_produces_a_contextual_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("does-not-exist.txt");
        let err = count_nonempty_lines(&path).unwrap_err();
        assert!(format!("{err:#}").contains("failed to open"));
    }

    #[test]
    fn resolve_output_dir_has_a_default() {
        // Not asserting a specific value since the real environment may
        // or may not have RUSTUT_OUTPUT_DIR set; just confirm this never
        // panics and always returns *something*.
        let _ = resolve_output_dir();
    }
}
