use std::{net::{ToSocketAddrs, TcpStream}, io::{BufWriter, BufReader, Write}};

use serde::Deserialize;
use serde_json::{Deserializer, de::IoRead};

use crate::{Result, Request, GetResponse, KvsError, SetResponse, RemoveResponse};
/// Key-value store client
pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>
}

impl KvsClient {
    // Connect to `addr` to access `KvsServer`
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?;

        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(tcp_reader)),
            writer: BufWriter::new(tcp_writer)
        })
    }

    // Get the value of a given key from the server
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;

        let resp = GetResponse::deserialize(&mut self.reader)?;
        match resp {
            GetResponse::Ok(value) => Ok(value),
            GetResponse::Err(_) => Err(KvsError::StringError("Key not found".to_string())),
        }
    }

    // Set the value of a given key in the server
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;

        let resp = SetResponse::deserialize(&mut self.reader)?;
        match resp {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(_) => Err(KvsError::StringError("Key not found".to_string())),
        }
    }

    // Remove a given key in the server
    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove{ key })?;
        self.writer.flush()?;

        let resp = RemoveResponse::deserialize(&mut self.reader)?;
        match resp {
            RemoveResponse::Ok(_) => Ok(()),
            RemoveResponse::Err(_) => Err(KvsError::StringError("Key not found".to_string())),
        }
    }
}