// SPDX-License-Identifier: MIT OR Apache-2.0
//! Reference solution for exercises/ch32-capstone-project.
//! See that exercise's README.md for the full spec.

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
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Returns `true` if `key` was present (and is now removed).
    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}

// ANCHOR: command_enum
/// One parsed client command.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
}
// ANCHOR_END: command_enum

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
/// newline) into a [`Command`]. Never panics on malformed input — always
/// returns `Err` instead, which the caller turns into an `ERROR` response
/// line rather than dropping the connection.
pub fn parse_command(line: &str) -> Result<Command, ProtocolError> {
    let mut parts = line.split_whitespace();
    let verb = parts.next().ok_or(ProtocolError::Empty)?;
    match verb.to_ascii_uppercase().as_str() {
        "GET" => {
            let key = parts
                .next()
                .ok_or_else(|| ProtocolError::MissingKey(line.to_string()))?;
            if parts.next().is_some() {
                return Err(ProtocolError::MissingKey(line.to_string()));
            }
            Ok(Command::Get {
                key: key.to_string(),
            })
        }
        "DELETE" => {
            let key = parts
                .next()
                .ok_or_else(|| ProtocolError::MissingKey(line.to_string()))?;
            if parts.next().is_some() {
                return Err(ProtocolError::MissingKey(line.to_string()));
            }
            Ok(Command::Delete {
                key: key.to_string(),
            })
        }
        "SET" => {
            let key = parts
                .next()
                .ok_or_else(|| ProtocolError::MissingValue(line.to_string()))?;
            let rest: Vec<&str> = parts.collect();
            if rest.is_empty() {
                return Err(ProtocolError::MissingValue(line.to_string()));
            }
            Ok(Command::Set {
                key: key.to_string(),
                value: rest.join(" "),
            })
        }
        other => Err(ProtocolError::UnknownCommand(other.to_string())),
    }
}

// ANCHOR: handle_command
/// Apply one command to the shared store, persisting to `persist_path`
/// after any mutation, and produce the wire response line (without a
/// trailing newline — the caller adds that).
pub fn handle_command(store: &Mutex<Store>, persist_path: &Path, command: Command) -> String {
    let mut guard = store
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match command {
        Command::Get { key } => match guard.get(&key) {
            Some(value) => format!("VALUE {value}"),
            None => "NOT_FOUND".to_string(),
        },
        Command::Set { key, value } => {
            guard.set(key, value);
            match guard.save(persist_path) {
                Ok(()) => "OK".to_string(),
                Err(e) => format!("ERROR failed to persist: {e}"),
            }
        }
        Command::Delete { key } => {
            let existed = guard.delete(&key);
            if existed {
                match guard.save(persist_path) {
                    Ok(()) => "OK".to_string(),
                    Err(e) => format!("ERROR failed to persist: {e}"),
                }
            } else {
                "NOT_FOUND".to_string()
            }
        }
    }
}
// ANCHOR_END: handle_command

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Server configuration — deliberately just a plain, directly-constructible
/// struct. A builder would add ceremony with no real payoff here (there are
/// no optional fields, no validation beyond what `TcpListener::bind` itself
/// already does); see the chapter for why this capstone reaches for the
/// *typestate* pattern instead, on [`Server`] itself.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub listen_addr: String,
    pub data_file: PathBuf,
}

// ANCHOR: typestate
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
// ANCHOR_END: typestate

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_round_trips_through_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("store.json");

        let mut store = Store::new();
        store.set("a".to_string(), "1".to_string());
        store.set("b".to_string(), "2".to_string());
        store.save(&path).unwrap();

        let reloaded = Store::load(&path).unwrap();
        assert_eq!(reloaded.get("a"), Some(&"1".to_string()));
        assert_eq!(reloaded.get("b"), Some(&"2".to_string()));
    }

    #[test]
    fn load_of_missing_file_is_an_empty_store() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("does-not-exist.json");
        let store = Store::load(&path).unwrap();
        assert_eq!(store.get("anything"), None);
    }

    #[test]
    fn parse_command_rejects_malformed_input_without_panicking() {
        assert_eq!(parse_command(""), Err(ProtocolError::Empty));
        assert!(matches!(
            parse_command("FROBNICATE x"),
            Err(ProtocolError::UnknownCommand(_))
        ));
        assert!(matches!(
            parse_command("GET"),
            Err(ProtocolError::MissingKey(_))
        ));
        assert!(matches!(
            parse_command("SET onlykey"),
            Err(ProtocolError::MissingValue(_))
        ));
    }

    #[test]
    fn parse_command_accepts_well_formed_input() {
        assert_eq!(
            parse_command("GET a"),
            Ok(Command::Get {
                key: "a".to_string()
            })
        );
        assert_eq!(
            parse_command("SET a hello world"),
            Ok(Command::Set {
                key: "a".to_string(),
                value: "hello world".to_string()
            })
        );
        assert_eq!(
            parse_command("delete a"),
            Ok(Command::Delete {
                key: "a".to_string()
            })
        );
    }
}
