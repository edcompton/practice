use super::error::Error;
use std::fs;

const FILENAME1: &str = "./inputs/day_two/input.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    Ok(initialise_intcode_program())
}

fn initialise_intcode_program() -> Vec<i32> {
    let mut results = Vec::new();
    results.push(run_intcode_program(12, 2));
    'outer: for x in 0..100 {
        let mut result = run_intcode_program(x, 1);
        if result != 19690720 {
            for y in 0..100 {
                result = run_intcode_program(x, y);
                if result == 19690720 {
                    results.push(result);
                    break 'outer;
                }
            }
        }
    }
    results
}

fn run_intcode_program(noun: usize, verb: usize) -> i32 {
    let mut file_indices = get_file_input(FILENAME1);
    restore_gravity_assist_program(noun, verb, &mut file_indices);
    run_computer(&mut file_indices, 0);
    file_indices[0] as i32
}

fn restore_gravity_assist_program(noun: usize, verb: usize, vec: &mut Vec<usize>) {
    vec[1] = noun;
    vec[2] = verb;
}

fn run_computer(vec: &mut Vec<usize>, pos: usize) {
    let opcode = vec[pos];
    if opcode == 99 {
        return;
    } else {
        let first_position_address = vec[pos + 1];
        let second_position_address = vec[pos + 2];
        let result_address = vec[pos + 3];

        match opcode {
            1 => vec[result_address] = vec[first_position_address] + vec[second_position_address],
            2 => vec[result_address] = vec[first_position_address] * vec[second_position_address],
            _ => return,
        }

        run_computer(vec, pos + 4);
    }
}

fn get_file_input(filename: &str) -> Vec<usize> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split(",")
        .map(|v| v.to_string().parse::<usize>().unwrap())
        .collect()
}
