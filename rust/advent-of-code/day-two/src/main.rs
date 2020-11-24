use std::fs;

const FILENAME1: &str = "./input.txt";
fn main() {
    let mut file_indices = get_file_input();
    restore_gravity_assist_program(&mut file_indices);
    move_position(&mut file_indices, 0);
    println!("{:?}", file_indices);
}

fn get_file_input() -> Vec<usize> {
    fs::read_to_string(FILENAME1)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split(",")
        .map(|v| v.to_string().parse::<usize>().unwrap())
        .collect()
}

fn restore_gravity_assist_program(vec: &mut Vec<usize>) {
    vec[1] = 12;
    vec[2] = 2;
}

fn move_position(vec: &mut Vec<usize>, pos: usize) {
    let opcode = vec[pos];
    if opcode == 99 {
        return;
    }
    let first_pos = vec[pos + 1];
    let second_pos = vec[pos + 2];
    let result_pos = vec[pos + 3];

    match opcode {
        99 => return,
        1 => vec[result_pos] = vec[first_pos] + vec[second_pos],
        2 => vec[result_pos] = vec[first_pos] * vec[second_pos],
        _ => return,
    }
    move_position(vec, pos + 4);
}
