use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "input.txt";

    let input = File::open(path)?;
    let lines = BufReader::new(input).lines();

    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;

    for result in lines {
        let line = result.unwrap();
        let vec: Vec<String> = line.split(' ').map(String::from).collect();

        let direction = &vec[0];
        let distance: i32 = vec[1].parse().unwrap();

        match direction.as_str() {
            "forward" => {
                horizontal_position += distance;
                depth += aim * distance;
            }
            "down" => {
                aim += distance;
            }
            "up" => {
                aim -= distance;
            }
            _ => {}
        }
    }

    println!(
        "You have travelled {} units horizontally and {} units vertically",
        horizontal_position, depth
    );
    println!(
        "Multiplied, your depth and horizontal position equal {}.",
        (horizontal_position * depth)
    );

    Ok(())
}
