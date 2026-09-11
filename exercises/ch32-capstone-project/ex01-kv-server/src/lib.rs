// SPDX-License-Identifier: MIT OR Apache-2.0
//! Core store and protocol logic for the capstone key-value server.
//! See README.md for the full spec and the Design Rubric.
//!
//! `Server`, `ServerConfig`, and `handle_client` below are complete —
//! the networking/threading/CLI wiring isn't the point of this exercise.
//! Your task is the three `todo!()`s: the `Store`'s core operations, the
//! wire-protocol parser, and the command dispatcher that ties them
//! together behind the shared lock.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use thiserror::Error;

/// The key-value data, in memory and (via [`Store::load`]/[`Store::save`])
/// on disk as JSON.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    data: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("I/O error accessing store file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to (de)serialize store: {0}")]
    Serde(#[from] serde_json::Error),
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    /// Load a store from `path`, or start empty if the file doesn't exist
    /// yet (e.g. first run).
    pub fn load(path: &Path) -> Result<Self, StoreError> {
        if !path.exists() {
            return Ok(Store::new());
        }
        let bytes = std::fs::read(path)?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Persist the whole store to `path` as JSON.
    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        let bytes = serde_json::to_vec_pretty(self)?;
        std::fs::write(path, bytes)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        todo!("look up `key` in `self.data`")
    }

    pub fn set(&mut self, key: String, value: String) {
        todo!("insert `key` -> `value` into `self.data` (overwriting any existing value)")
    }

    /// Returns `true` if `key` was present (and is now removed).
    pub fn delete(&mut self, key: &str) -> bool {
        todo!("remove `key` from `self.data`; return whether it was present")
    }
}

/// One parsed client command.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("empty command")]
    Empty,
    #[error("unknown command {0:?}")]
    UnknownCommand(String),
    #[error("GET/DELETE need exactly one key, got: {0:?}")]
    MissingKey(String),
    #[error("SET needs a key and a value, got: {0:?}")]
    MissingValue(String),
}

/// Parse one line of client input (already stripped of its trailing
/// newline) into a [`Command`].
///
/// Must **never panic** on malformed input — always return `Err` instead
/// (the caller turns that into an `ERROR` response line rather than
/// dropping the connection). The wire format: whitespace-separated words,
/// verb is case-insensitive (`GET`/`SET`/`DELETE`), `GET`/`DELETE` take
/// exactly one key and nothing else, `SET` takes a key followed by one or
/// more words that get rejoined with single spaces as the value (so
/// `SET greeting hello world` has value `"hello world"`).
pub fn parse_command(line: &str) -> Result<Command, ProtocolError> {
    todo!(
        "split on whitespace; dispatch on the first word (case-insensitively) \
         to GET/SET/DELETE; return the matching ProtocolError variant for anything malformed"
    )
}

/// Apply one command to the shared store, persisting to `persist_path`
/// after any mutation (SET/DELETE), and produce the wire response line
/// (without a trailing newline — the caller adds that): `"VALUE <v>"` or
/// `"NOT_FOUND"` for `Get`, `"OK"` (or `"ERROR ..."` if persisting fails)
/// for `Set`, `"OK"`/`"NOT_FOUND"` for `Delete`.
pub fn handle_command(store: &Mutex<Store>, persist_path: &Path, command: Command) -> String {
    todo!(
        "lock `store` (recover from poisoning with `.unwrap_or_else(|p| p.into_inner())` \
         rather than propagating a panic from one client into every other connection), \
         match on `command`, and call Store::get/set/delete plus Store::save as needed"
    )
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Server configuration — deliberately just a plain, directly-constructible
/// struct; see the chapter for why this capstone reaches for the
/// *typestate* pattern on [`Server`] itself instead of a builder here.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub listen_addr: String,
    pub data_file: PathBuf,
}

/// Type-state marker: configured, not yet bound to a socket.
pub struct Unbound;

/// Type-state marker: bound to a real socket, ready to accept connections
/// (or to have its assigned port read back, if `listen_addr` used port 0).
pub struct Bound {
    listener: TcpListener,
}

/// A server, tracked through its `Unbound -> Bound` lifecycle in the type
/// system: [`Server::run`] only exists on `Server<Bound>`, so calling it
/// before [`Server::bind`] is a compile error, not a runtime "not bound
/// yet" panic or `Option` you have to check.
pub struct Server<State = Unbound> {
    config: ServerConfig,
    state: State,
}

impl Server<Unbound> {
    pub fn new(config: ServerConfig) -> Self {
        Server {
            config,
            state: Unbound,
        }
    }

    pub fn bind(self) -> Result<Server<Bound>, ServerError> {
        let listener = TcpListener::bind(&self.config.listen_addr)?;
        Ok(Server {
            config: self.config,
            state: Bound { listener },
        })
    }
}

impl Server<Bound> {
    /// The actual address this server is listening on — in particular,
    /// the OS-assigned port when `listen_addr` ended in `:0`. Only
    /// meaningful (and only callable) once bound.
    pub fn local_addr(&self) -> Result<SocketAddr, ServerError> {
        Ok(self.state.listener.local_addr()?)
    }

    /// Accept connections forever, handling each on its own thread. Blocks
    /// the calling thread; callers that want to keep doing other things
    /// (e.g. tests) should run this on a spawned thread.
    pub fn run(self) -> Result<(), ServerError> {
        let store = Arc::new(Mutex::new(
            Store::load(&self.config.data_file).unwrap_or_default(),
        ));
        for stream in self.state.listener.incoming() {
            let stream = stream?;
            let store = Arc::clone(&store);
            let data_file = self.config.data_file.clone();
            thread::spawn(move || {
                if let Err(e) = handle_client(stream, &store, &data_file) {
                    eprintln!("client error: {e}");
                }
            });
        }
        Ok(())
    }
}

fn handle_client(stream: TcpStream, store: &Mutex<Store>, data_file: &Path) -> std::io::Result<()> {
    let mut writer = stream.try_clone()?;
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let line = line?;
        let response = match parse_command(&line) {
            Ok(command) => handle_command(store, data_file, command),
            Err(e) => format!("ERROR {e}"),
        };
        writeln!(writer, "{response}")?;
    }
    Ok(())
}
