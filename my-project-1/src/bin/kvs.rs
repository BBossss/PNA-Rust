use std::process;

use clap::{Arg, Command};

fn main() {
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
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::new("KEY").help("a string key").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove the given key")
                .arg(Arg::new("KEY").help("a string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", _matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        Some(("get", _matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        Some(("rm", _matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        _ => {
            unreachable!();
        }
    }
}
