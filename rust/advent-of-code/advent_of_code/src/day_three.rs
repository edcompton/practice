// Two wires are connected centrally and extend outward
// Trace path each wire takes, which is stored as text
// Aim is to find the point that both wires cross closest to the central port
use super::error::Error;
use std::fs;

const FILENAME1: &str = "./inputs/day_three/input.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    let file_input = get_file_input(FILENAME1);
    println!("{:?}", file_input);
    let (first_wire, second_wire) = create_instruction_arrays(file_input);

    println!("{:?}", first_wire);
    Ok(vec![2345 as i32])
}

fn get_file_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split("\n")
        .map(|v| v.to_string())
        .collect()
}

fn create_instruction_arrays(
    file_input: Vec<String>,
) -> (Vec<(String, String)>, Vec<(String, String)>) {
    let result: Vec<Vec<(String, String)>> = file_input
        .into_iter()
        .map(|v| {
            v.split(",")
                .map(|s| {
                    let inner_val = s.to_string();
                    let split_val = inner_val.split_at(1);
                    (split_val.0.to_string(), split_val.1.to_string())
                })
                .collect::<Vec<(String, String)>>()
        })
        .collect();

    // println!("{:?}", result);

    (result[0].clone(), result[1].clone())
}
