use std::{process::exit};

use clap::{Arg, Command, arg};
use kvs::{ Result, KvsError, KvsClient };

fn main() -> Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::new("KEY").help("a string key").required(true))
                .arg(
                    Arg::new("VALUE")
                        .help("the string value of the key")
                        .required(true),
                )
                .arg(
                    arg!(--addr <VALUE>)
                        .required(false)
                        .default_value("127.0.0.1:4000"),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::new("KEY").help("a string key").required(true))
                .arg(
                    arg!(--addr <VALUE>)
                        .required(false)
                        .default_value("127.0.0.1:4000"),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove the given key")
                .arg(Arg::new("KEY").help("a string key").required(true))
                .arg(
                    arg!(--addr <VALUE>)
                        .required(false)
                        .default_value("127.0.0.1:4000"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let value = matches.value_of("VALUE").unwrap();
            let addr = matches.value_of("addr").unwrap();
            let addr = match addr.parse::<std::net::SocketAddr>() {
                Ok(addr) => addr,
                Err(_) => {
                    eprintln!("Invalid address: {}", addr);
                    exit(1);
                }
            };

            let mut client = KvsClient::connect(addr)?;
            client.set(key.to_string(), value.to_string())?;
        }
        Some(("get", matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let addr = matches.value_of("addr").unwrap();
            let addr = match addr.parse::<std::net::SocketAddr>() {
                Ok(addr) => addr,
                Err(_) => {
                    eprintln!("Invalid address: {}", addr);
                    exit(1);
                }
            };

            let mut client = KvsClient::connect(addr)?;
            if let Some(value) = client.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
                exit(0);
            }
        }
        Some(("rm", matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let addr = matches.value_of("addr").unwrap();
            let addr = match addr.parse::<std::net::SocketAddr>() {
                Ok(addr) => addr,
                Err(_) => {
                    eprintln!("Invalid address: {}", addr);
                    exit(1);
                }
            };

            let mut client = KvsClient::connect(addr)?; 
            match client.remove(key.to_string()) {
                Ok(()) => {},
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                },
                Err(err) => return Err(err)
            }
        }
        _ => {
            unreachable!();
        }
    }
    Ok(())
}
