//! A simple key/value store.

pub use error::{KvsError, Result};
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use common::{Request, GetResponse, SetResponse, RemoveResponse};
pub use server::KvsServer;
pub use client::KvsClient;

mod engines;
mod error;
mod common;
mod server;
mod client;