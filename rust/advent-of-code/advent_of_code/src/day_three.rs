// Two wires are connected centrally and extend outward
// Trace path each wire takes, which is stored as text
// Aim is to find the point that both wires cross closest to the central port
// For each of the instruction vectors, carry out the instructions and
use super::error::Error;
use differ::{Differ, Tag};
use std::fs;

const FILENAME1: &str = "./inputs/day_three/input.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    let file_input = get_file_input(FILENAME1);
    let (first_wire, second_wire) = create_instruction_arrays(file_input);
    let first_coordinates = get_coordinates(first_wire);
    let second_coordinates = get_coordinates(second_wire);
    let (closest_crossing_coordinates, _) =
        compare_wire_coordinates(first_coordinates, second_coordinates);
    Ok(vec![closest_crossing_coordinates])
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
) -> (Vec<(Direction, i32)>, Vec<(Direction, i32)>) {
    let result: Vec<Vec<(Direction, i32)>> = file_input
        .into_iter()
        .map(|v| {
            v.split(",")
                .map(|s| {
                    let inner_val = s.to_string();
                    let split_val = inner_val.split_at(1);
                    let parsed_val: i32 = split_val.1.parse().unwrap();
                    (split_val.0.parse::<Direction>().unwrap(), parsed_val)
                })
                .collect::<Vec<(Direction, i32)>>()
        })
        .collect();

    (result[0].clone(), result[1].clone())
}

fn get_coordinates(input: Vec<(Direction, i32)>) -> Vec<(i32, i32)> {
    let mut acc = vec![(0, 0)];
    for (instruction, move_amount) in input.iter() {
        for _ in 0..*move_amount {
            let previous_index = acc.len() - 1;
            let x = acc[previous_index].0;
            let y = acc[previous_index].1;

            match instruction {
                Direction::U => acc.push((acc[previous_index].0, y + 1)),
                Direction::D => acc.push((acc[previous_index].0, y - 1)),
                Direction::R => acc.push((x + 1, acc[previous_index].1)),
                Direction::L => acc.push((x - 1, acc[previous_index].1)),
            };
        }
    }
    acc.sort();
    acc
}

fn compare_wire_coordinates(
    first_wire: Vec<(i32, i32)>,
    second_wire: Vec<(i32, i32)>,
) -> (i32, Vec<(usize, usize)>) {
    let differ = Differ::new(&first_wire, &second_wire);
    let mut cross_points = Vec::new();
    let mut cross_array_positions = Vec::new();
    for span in differ.spans() {
        if span.tag == Tag::Equal {
            let span = &first_wire[span.a_start];
            cross_points.push(sum_steps(*span));
            calculate_crossing_array_positions(
                &first_wire,
                &second_wire,
                span,
                &mut cross_array_positions,
            );
        }
    }
    cross_points.sort();
    let closest_intersection = cross_points[1];

    let mut total_steps = cross_array_positions
        .iter()
        .map(|(x, y)| x + y)
        .collect::<Vec<usize>>();
    total_steps.sort();

    println!("{:?}", total_steps);
    (closest_intersection, cross_array_positions)
}

fn sum_steps(span: (i32, i32)) -> i32 {
    let (a, b) = span;
    if a < 0 {
        (a * -1) + b
    } else if b < 0 {
        (b * -1) + a
    } else {
        a + b
    }
}

fn calculate_crossing_array_positions(
    first_wire: &Vec<(i32, i32)>,
    second_wire: &Vec<(i32, i32)>,
    span: &(i32, i32),
    position_array: &mut Vec<(usize, usize)>,
) {
    if let Some(first_position) = first_wire.iter().position(|x| x == span) {
        println!("first position {}", first_position);
        if let Some(second_position) = second_wire.iter().position(|x| x == span) {
            println!("second position {}", second_position);
            position_array.push((first_position, second_position))
        }
    }
}

custom_derive! {
#[derive(Debug, EnumFromStr, PartialEq, Clone)]
    enum Direction {
    U,
    D,
    L,
    R,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_steps() {
        assert_eq!(
            sum_steps((20, 20)),
            40,
            "Two values should be equal to each other"
        );
    }
    #[test]
    fn test_sum_steps_with_incorrect_values() {
        assert_ne!(sum_steps((2, 2)), 40);
    }
}
