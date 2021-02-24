use super::computer::{Computer, ComputerActions};
use super::error::Error;
use std::fs;

const FILENAME1: &str = "./inputs/day_two/input.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    Ok(initialise_intcode_program())
}

fn initialise_intcode_program() -> Vec<i32> {
    let mut results = Vec::new();
    let file_indices = get_file_input(FILENAME1);
    let mut computer = Computer::new(12, 2, file_indices);
    computer.restore_gravity_assist_program();
    computer.run();
    results.push(computer.computed_values[0] as i32);
    'outer: for x in 0..100 {
        for y in 0..100 {
            computer.reset(x, y);
            computer.run();
            let result = computer.computed_values[0];
            if result == 19_690_720 {
                results.push(result as i32);
                break 'outer;
            }
        }
    }
    results
}

fn get_file_input(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split(",")
        .map(|v| v.to_string().parse::<i32>().unwrap())
        .collect()
}
