use std::env;
use std::fs;

fn main() {
    // Prints each argument on a separate line
    let arguments: Vec<String> = env::args().collect();
    let filename = arguments[1].clone();
    println!("Opening File: {}", filename);
    let contents = fs::read_to_string(filename);
    match contents {
        Ok(contents) => {
            let lines: Vec<&str> = contents.split("\n").filter(|line| line.len() > 0).collect();
            let commands: Vec<Vec<&str>> = lines.iter().map(|line| line.split(" ").collect()).collect();
            let mut horizontal_position = 0;
            let mut depth = 0;
            for command in commands.clone() {
                match command[0] {
                    "up" => {
                        depth -= command[1].parse::<i32>().unwrap();
                    },
                    "down" => {
                        depth += command[1].parse::<i32>().unwrap();
                    },
                    "forward" => {
                        horizontal_position += command[1].parse::<i32>().unwrap();
                    },
                    _ => {
                        println!("Unknown command: {}", command[0]);
                    }
                }
            }
            println!("Part 1 horizontal position: {}", horizontal_position);
            println!("Part 1 depth: {}", depth);
            println!("Part 1 answer: {}", horizontal_position * depth);
            let mut horizontal_position = 0;
            let mut depth = 0;
            let mut aim = 0;
            for command in commands {
                match command[0] {
                    "up" => {
                        aim -= command[1].parse::<i32>().unwrap();
                    },
                    "down" => {
                        aim += command[1].parse::<i32>().unwrap();
                    },
                    "forward" => {
                        let distance = command[1].parse::<i32>().unwrap();
                        horizontal_position += distance;
                        depth += aim * distance;
                    },
                    _ => {
                        println!("Unknown command: {}", command[0]);
                    }
                }
            }
            println!("Part 2 horizontal position: {}", horizontal_position);
            println!("Part 2 depth: {}", depth);
            println!("Part 2 answer: {}", horizontal_position * depth);
        }
        Err(error) => {
            println!("Error: {}", error)
        }
    }
}
