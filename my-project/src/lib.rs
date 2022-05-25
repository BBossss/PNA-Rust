use std::{collections::HashMap, path::PathBuf, io};

use failure::Fail;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    hashmap: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> KvStore {
        Self {
            hashmap: HashMap::new(),
        }
    }

    /// Sets the value of the string key to a string.
    ///
    /// Return an error if the value is not written successfully.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        unimplemented!();
    }

    /// Get the string value of a given string key.
    ///
    /// Return `None` if the given key does not exist.
    pub fn get(&mut self, _key: String) -> Result<Option<String>> {
        unimplemented!();
    }

    /// Remove a given key.
    pub fn remove(&mut self, _key: String) -> Result<()> {
        unimplemented!();
    }

    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        unimplemented!();
    }
}


/// Error type for Kvstore
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Remove non-existing key error
    #[fail(display = "key not found")]
    KeyNotFound,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

/// Result type for Kvstore
type Result<T> = std::result::Result<T, KvsError>;