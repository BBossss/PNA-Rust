use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Move {
    steps: u32,
    direction: String,
}
fn main() {
    let m = Move {
        steps: 5,
        direction: String::from("right")
    };

    let s = ron::to_string(&m).unwrap();
    println!("{:?}", s);

    let b: Move = ron::from_str(&s).unwrap();
    println!("{:?}", b);
}
