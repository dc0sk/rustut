// SPDX-License-Identifier: MIT OR Apache-2.0
//! CLI wiring for the capstone key-value server. All the interesting logic
//! lives in `lib.rs`; this binary just parses arguments and drives it.

use anyhow::Context;
use ch32_capstone_project::{Server, ServerConfig};
use clap::Parser;
use std::path::PathBuf;

/// A tiny concurrent, persistent key-value store server.
///
/// Protocol (newline-delimited, connect with e.g. `nc`):
///   SET <key> <value...>  -> OK / ERROR <msg>
///   GET <key>              -> VALUE <value> / NOT_FOUND
///   DELETE <key>            -> OK / NOT_FOUND
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Address to listen on. Use port 0 to let the OS pick one.
    #[arg(long, default_value = "127.0.0.1:7878")]
    listen: String,

    /// Path to the JSON file the store is persisted to/loaded from.
    #[arg(long, default_value = "store.json")]
    data_file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = ServerConfig {
        listen_addr: args.listen.clone(),
        data_file: args.data_file,
    };

    let server = Server::new(config)
        .bind()
        .with_context(|| format!("binding to {}", args.listen))?;
    println!("listening on {}", server.local_addr()?);
    server.run().context("server loop failed")?;
    Ok(())
}
