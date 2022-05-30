use std::{process::exit, env::current_dir, fs};

use clap::{Command, arg};
use kvs::{Result, KvsServer, KvStore, SledKvsEngine};


fn main() -> Result<()> {
    let matches = Command::new("kvs-server")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Kvs server")
        .arg(
            arg!(--addr <VALUE>)
                .required(false)
                .default_value("127.0.0.1:8080")
        )
        .arg(
            arg!(--engine <VALUE>)
                .required(false)
        )
        .get_matches();

    let addr = matches.value_of("addr").unwrap();
    let addr = match addr.parse::<std::net::SocketAddr>() {
        Ok(addr) => addr,
        Err(_) => {
            eprintln!("Invalid address: {}", addr);
            exit(1);
        }
    };
    let mut new_engine = match matches.value_of("engine") {
        Some(engine) => Some(engine.to_owned()),
        None => None
    };
    let old_engine = match current_engine() {
        Ok(engine) => engine,
        Err(_) => exit(1)
    };

    if new_engine.is_none() {
        new_engine = old_engine;
    } else if old_engine.is_some() && old_engine != new_engine {
        exit(1);
    }

    let engine = new_engine.unwrap_or("kvs".to_string());
    fs::write(current_dir()?.join("engine"), format!("{}", engine))?;

    if engine == "kvs".to_string() {
        let mut server = KvsServer::new(KvStore::open(current_dir()?)?);
        server.run(addr)
    } else {
        let mut server = KvsServer::new(SledKvsEngine::new(sled::open(current_dir()?)?));
        server.run(addr)
    }
    
}

fn current_engine() -> Result<Option<String>> {
    let engine = current_dir()?.join("engine");
    
    if !engine.exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine) {
        Ok(engine) => Ok(Some(engine)),
        Err(_) => {
            Ok(None)
        }
    }
}