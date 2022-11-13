use std::{fs::File, io};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Direction {
    Front,
    Back,
    Left,
    Rigth,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Move {
    direction: Direction,
    steps: usize,
}

impl Move {
    fn new(direction: Direction, steps: usize) -> Move {
        Move{direction, steps}
    }
}

fn main() -> Result<(), io::Error> {
    let a = Move::new(Direction::Front, 3);
    
    let json_writer = File::create("move.json")?;
    serde_json::to_writer(&json_writer, &a)?;
    
    let json_reader = File::open("move.json")?;
    let b: Move = serde_json::from_reader(&json_reader)?;

    assert_eq!(a,b);
    Ok(())
}
