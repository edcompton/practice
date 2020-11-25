use std::fs;

const FILENAME1: &str = "./input.txt";
fn main() {
    initialise_intcode_program()
}

fn initialise_intcode_program() {
    for x in 0..100 {
        let mut result = run_intcode_program(x, 1);
        if result != 19690720 {
            for y in 0..100 {
                result = run_intcode_program(x, y);
                if result == 19690720 {
                    println!("{}", x);
                    println!("{}", y);
                    println!("{}", result);
                    return;
                }
            }
        }
    }
}

fn run_intcode_program(noun: usize, verb: usize) -> usize {
    let mut file_indices = get_file_input(FILENAME1);
    restore_gravity_assist_program(noun, verb, &mut file_indices);
    move_position(&mut file_indices, 0);
    file_indices[0]
}

fn get_file_input(filename: &str) -> Vec<usize> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split(",")
        .map(|v| v.to_string().parse::<usize>().unwrap())
        .collect()
}

fn restore_gravity_assist_program(noun: usize, verb: usize, vec: &mut Vec<usize>) {
    vec[1] = noun;
    vec[2] = verb;
}

fn move_position(vec: &mut Vec<usize>, pos: usize) {
    let opcode = vec[pos];
    if opcode == 99 {
        return;
    } else {
        let first_param_address = vec[pos + 1];
        let second_param_address = vec[pos + 2];
        let result_address = vec[pos + 3];

        match opcode {
            1 => vec[result_address] = vec[first_param_address] + vec[second_param_address],
            2 => vec[result_address] = vec[first_param_address] * vec[second_param_address],
            _ => return,
        }

        move_position(vec, pos + 4);
    }
}
