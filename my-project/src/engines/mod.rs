use crate::Result;

/// This module provides various key-value storage engines.

pub trait KvsEngine {
    /// Sets a value of a string key.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Gets a value of a string key.
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Removes a value of a string key.
    fn remove(&mut self, key: String) -> Result<()>;
}

mod kvs;
mod sled;

pub use kvs::KvStore;
pub use self::sled::SledKvsEngine;