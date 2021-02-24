// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
//  - So this needs to halt the program and ask the user for an input

// Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
//  - So this will print whatever is at the index referenced

// Parameter mode - switch how the inputs are interpreted
//      0 = position (index)
//      1 = immediate (used as value)

// Instructions:
// Instruction is determined by whatever is at position 0 after moving the  required number of pos from the previous instruction.
// Check the length first - if it's greater than 1, it's an instruction with params
// Rightmost of the two digits that are the first value in an instruction. Pop them off and trim to get the Opcode.
// Pop off the params in order, using index or value depending on what param it is.

// Run the programme - as the first value is 3, it will ask for an input
// Shift forward the number of spaces that the instruction pointer consisted of.

// Edits to computer file;
// Opcode determination needs a new function
// Param mode params need to be stored on the computer
// New Opcode functions for 3 and 4
// Move number of pos based on number of instructions, rather than +4

// Rust learnings to incorporate if possible;
// - Generics (input parsing?)
// - Concurrency
// - Closures
// - Return results
// - Trait bounds/ trait objects
// - Tests and integration tests
// - Doc tests

use crate::computer::{Computer, ComputerActions};
use crate::error::Error;
use std::fs;

const FILENAME1: &str = "./inputs/day_five/input.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    let mut computer = Computer::new(0, 0, get_file_input(FILENAME1));
    computer.run();
    let output_diagnostic = computer.outputs[computer.outputs.len() - 1];
    Ok(vec![output_diagnostic])
}

fn get_file_input(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split(",")
        .map(|v| v.to_string().parse::<i32>().unwrap())
        .collect()
}
