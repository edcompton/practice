// Two wires are connected centrally and extend outward
// Trace path each wire takes, which is stored as text
// Aim is to find the point that both wires cross closest to the central port
use super::error::Error;
use differ::{Differ, Tag};
use std::collections::HashMap;
use std::fs;

const FILENAME1: &str = "./inputs/day_three/input.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    let file_input = get_file_input(FILENAME1);
    let (first_wire, second_wire) = create_instruction_arrays(file_input);
    let first_wire_results = get_coordinates(first_wire);
    let second_wire_results = get_coordinates(second_wire);
    let (closest_crossing_coordinates, fewest_steps) =
        compare_wire_coordinates(first_wire_results, second_wire_results);
    Ok(vec![closest_crossing_coordinates, fewest_steps])
}

struct Moves {
    coordinates: Vec<(i32, i32)>,
    coordinates_map: HashMap<(i32, i32), i32>,
}

impl Moves {
    pub fn new() -> Self {
        Moves {
            coordinates: vec![(0, 0)],
            coordinates_map: HashMap::new(),
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

fn get_coordinates(input: Vec<(Direction, i32)>) -> Moves {
    let mut acc = Moves::new();
    let mut total = 0;
    for (instruction, move_amount) in input.iter() {
        for _ in 0..*move_amount {
            total += 1;
            let previous_index = acc.coordinates.len() - 1;
            let x = acc.coordinates[previous_index].0;
            let y = acc.coordinates[previous_index].1;
            match instruction {
                Direction::U => acc
                    .coordinates
                    .push((acc.coordinates[previous_index].0, y + 1)),
                Direction::D => acc
                    .coordinates
                    .push((acc.coordinates[previous_index].0, y - 1)),
                Direction::R => acc
                    .coordinates
                    .push((x + 1, acc.coordinates[previous_index].1)),
                Direction::L => acc
                    .coordinates
                    .push((x - 1, acc.coordinates[previous_index].1)),
            };
            acc.coordinates_map
                .entry(acc.coordinates[acc.coordinates.len() - 1].clone())
                .or_insert(total);
        }
    }

    acc.coordinates.sort();
    acc
}

fn compare_wire_coordinates(
    mut first_wire_results: Moves,
    mut second_wire_results: Moves,
) -> (i32, i32) {
    let first_wire = first_wire_results.coordinates;
    let second_wire = second_wire_results.coordinates;
    let differ = Differ::new(&first_wire, &second_wire);
    let mut cross_points = Vec::new();
    let mut cross_array_positions = Vec::new();
    for span in differ.spans() {
        if span.tag == Tag::Equal {
            let span = &first_wire[span.a_start];
            cross_array_positions.push(calculate_crossing_array_positions(
                &mut first_wire_results.coordinates_map,
                &mut second_wire_results.coordinates_map,
                span,
            ));
            cross_points.push(sum_steps(span.clone()))
        }
    }
    cross_points.sort();
    cross_array_positions.sort();
    (cross_points[1], cross_array_positions[1] as i32)
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

// At the moment, I'm just summing the coordinates and assuming they are steps.
// But in reality, coordinates don't dictate the number of steps required to get to a coordinate.
// So what I actually need is to know the number of steps taken to get to each of the coordinates
// which is the index...?
fn calculate_crossing_array_positions(
    first_hash: &mut HashMap<(i32, i32), i32>,
    second_hash: &mut HashMap<(i32, i32), i32>,
    span: &(i32, i32),
) -> i32 {
    let mut result = 0;
    if let Some(first_position) = first_hash.get(span) {
        if let Some(second_position) = second_hash.get(span) {
            result = (first_position + second_position) as i32
        }
    }
    result
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
