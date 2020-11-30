// Two wires are connected centrally and extend outward
// Trace path each wire takes, which is stored as text
// Aim is to find the point that both wires cross closest to the central port
// For each of the instruction vectors, carry out the instructions and
use super::error::Error;
use std::fs;

const FILENAME1: &str = "./inputs/day_three/input.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    let file_input = get_file_input(FILENAME1);
    let (first_wire, second_wire) = create_instruction_arrays(file_input);
    let first_coordinates = get_coordinates(first_wire);
    println!("{:?}", first_coordinates);
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

fn create_instruction_arrays(file_input: Vec<String>) -> (Vec<(String, i32)>, Vec<(String, i32)>) {
    let result: Vec<Vec<(String, i32)>> = file_input
        .into_iter()
        .map(|v| {
            v.split(",")
                .map(|s| {
                    let inner_val = s.to_string();
                    let split_val = inner_val.split_at(1);
                    let parsed_val: i32 = split_val.1.parse().unwrap();
                    (split_val.0.to_string(), parsed_val)
                })
                .collect::<Vec<(String, i32)>>()
        })
        .collect();

    // println!("{:?}", result);

    (result[0].clone(), result[1].clone())
}

fn get_coordinates(input: Vec<(String, i32)>) {
    let move_coordinates = input.into_iter().fold(vec![(0, 0)], |acc, val| {
        let move_amount = val.1;
        let previous_index = acc.len() - 1;
        let x = acc[previous_index].0;
        let y = acc[previous_index].1;

        let instruction = val.0;
        match instruction.as_str() {
            "U" => acc.push((acc[previous_index].0, y + move_amount)),
            "D" => acc.push((acc[previous_index].0, y - move_amount)),
            "R" => acc.push((x + move_amount, acc[previous_index].0)),
            "L" => acc.push((x - move_amount, acc[previous_index].0)),
            _ => panic!("Shit"),
        };
        acc
    });
}
