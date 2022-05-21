use std::{fs::File, io::{Write, self, Read}};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    steps: u32,
    direction: String,
}

fn main() -> io::Result<()> {
    let a = Move {
        steps: 5,
        direction: "right".to_owned(),
    };

    let j = serde_json::to_string(&a).unwrap();

    let mut file = File::create("test.json")?;
    file.write(j.as_bytes()).unwrap();

    let mut file = File::open("test.json").unwrap();
    let mut buffer = Vec::new();
    file.read(&mut buffer).unwrap();
    println!("{:?}", &buffer);
    let b: Move = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
    println!("{b:?}");
    Ok(())
}
