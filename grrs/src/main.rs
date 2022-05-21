use std::fs;
#[allow(unused)]

use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap[parse(from_os_str)]]
    path: PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let contents = fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", args.path))?;

    for line in contents.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
