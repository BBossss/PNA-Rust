use std::process::exit;

use clap::{Command, arg};
use kvs::Result;

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
    let engine = match matches.value_of("engine") {
        Some("kvs") => "kvs",
        Some("sled") => "sled",
        None => "kvs",
        Some(engine) =>  {
            eprintln!("Invalid engine: {}", engine);
            exit(1);
        }
    };
    
    
    Ok(())
}