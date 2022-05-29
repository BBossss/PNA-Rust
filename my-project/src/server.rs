use std::{net::{ToSocketAddrs, TcpListener, TcpStream}, io::{BufReader, BufWriter, Write}};
use crate::{KvsEngine, KvsError, Result, Request, GetResponse, SetResponse, RemoveResponse};
use log::error;
use serde_json::Deserializer;

/// The server of a key value store
pub struct KvsServer<E: KvsEngine> {
    engine: E
}

impl<E: KvsEngine> KvsServer<E> {
    /// Create a `KvsServer` with a given storage engine
    pub fn new(e: E) -> KvsServer<E> {
        KvsServer {
            engine: e
        }
    }

    /// Run the server listening on the given address
    pub fn run<A: ToSocketAddrs>(&mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.serve(stream) {
                        error!("Error on serving client, {}", e)
                    }
                },
                Err(e) => error!("Connection failed, {}", e),
            }
        }
        
        Ok(())
    } 

    fn serve(&mut self, stream: TcpStream) -> Result<()> {
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();
        
        for req in req_reader {
            let req = req?;
            
            match req {
                Request::Get { key } => {
                    let resp = match self.engine.get(key) {
                        Ok(value) => GetResponse::Ok(value),
                        Err(e) => GetResponse::Err(e.to_string()),
                    };
                    serde_json::to_writer(&mut writer, &resp)?;
                    writer.flush()?;
                },
                Request::Set { key, value } => {
                    let resp = match self.engine.set(key, value) {
                        Ok(_) => SetResponse::Ok(()),
                        Err(e) => SetResponse::Err(e.to_string()),
                    };
                    serde_json::to_writer(&mut writer, &resp)?;
                    writer.flush()?;
                },
                Request::Remove { key } => {
                    let resp = match self.engine.remove(key) {
                        Ok(_) => RemoveResponse::Ok(()),
                        Err(e) => RemoveResponse::Err(e.to_string()),
                    };
                    serde_json::to_writer(&mut writer, &resp)?;
                    writer.flush()?;
                }
            }
        }

        Ok(())
    }
}
