//! A simple key/value store.

pub use error::{KvsError, Result};
pub use engines::KvStore;

mod engines;
mod error;

pub struct KvsEngine;