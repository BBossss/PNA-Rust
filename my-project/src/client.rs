use std::net::ToSocketAddrs;

use crate::Result;
/// Key-value store client
pub struct KvsClient {

}

impl KvsClient {
    // Connect to `addr` to access `KvsServer`
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<KvsClient> {
        
        Ok(KvsClient {})
    }

    // Get the value of a given key from the server
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        
        Ok(None)
    }

    // Set the value of a given key in the server
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        
        Ok(())
    }

    // Remove a given key in the server
    pub fn remove(&mut self, key: String) -> Result<()> {
        
        Ok(())
    }
}