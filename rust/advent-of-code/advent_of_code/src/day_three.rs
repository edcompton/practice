// Two wires are connected centrally and extend outward
// Trace path each wire takes, which is stored as text
// Aim is to find the point that both wires cross closest to the central port
use super::error::Error;
use differ::{Differ, Tag};
use std::fs;
use std::collections::HashMap;

const FILENAME1: &str = "./inputs/day_three/input.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    let file_input = get_file_input(FILENAME1);
    let (first_wire, second_wire) = create_instruction_arrays(file_input);
    let first_result_tuple = get_coordinates(first_wire);
    let second_result_tuple = get_coordinates(second_wire);
    let (closest_crossing_coordinates, fewest_steps) =
        compare_wire_coordinates(first_result_tuple, second_result_tuple);
    Ok(vec![closest_crossing_coordinates, fewest_steps])
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

struct Moves {
    coordinates: Vec<(i32,i32)>,
    steps: i32,
}

impl Moves {
    pub fn new() -> Self {
        Moves {
            coordinates: vec![(0,0)],
            steps: 0,
        }
    }
    pub fn coordinates(&self) -> &Vec<(i32, i32)> {
        &self.coordinates
    }
    pub fn steps(&self) -> i32 {
        self.steps
    }
}

fn get_coordinates(input: Vec<(Direction, i32)>) -> (Vec<(i32,i32)>,HashMap<(i32, i32), i32>) {
    let mut acc = vec![(0,0)];
    let mut total = 0;
    let mut hash_map: HashMap<(i32, i32), i32> = HashMap::new();
    for (instruction, move_amount) in input.iter() {
        for _ in 0..*move_amount {
            total += 1;
            let previous_index = acc.len() - 1;
            let x = acc[previous_index].0;
            let y = acc[previous_index].1;
            match instruction {
                Direction::U => acc.push((acc[previous_index].0, y + 1)),
                Direction::D => acc.push((acc[previous_index].0, y - 1)),
                Direction::R => acc.push((x + 1, acc[previous_index].1)),
                Direction::L => acc.push((x - 1, acc[previous_index].1)),
            };
            hash_map.entry(acc[acc.len()-1].clone()).or_insert(total);
        }
    }
    println!("{:?}",hash_map);
    acc.sort();
    (acc, hash_map)
}

fn compare_wire_coordinates(
    first_tuple: (Vec<(i32,i32)>,HashMap<(i32, i32), i32>),
    second_tuple: (Vec<(i32,i32)>,HashMap<(i32, i32), i32>),
) -> (i32, i32) {
    let (first_wire, mut first_hash) = first_tuple;
    let (second_wire, mut second_hash) = second_tuple;
    let differ = Differ::new(&first_wire, &second_wire);
    let mut cross_points = Vec::new();
    let mut cross_array_positions = Vec::new();
    for span in differ.spans() {
        if span.tag == Tag::Equal {
            let span = &first_wire[span.a_start];
            println!("{:?}", span);
            println!("{:?} {:?}", first_hash.entry(*span), second_hash.entry(*span));
            cross_array_positions.push(calculate_crossing_array_positions(&mut first_hash, &mut second_hash, span));
            cross_points.push(sum_steps(span.clone()));
        }
    }
    cross_points.sort();
    cross_array_positions.sort();
    println!("{:?}", cross_array_positions);

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
    let mut result= 0;
    if let Some(first_position) = first_hash.get(span) {
        println!("first position {}", first_position);
        if let Some(second_position) = second_hash.get(span) {
            println!("second position {}", second_position);
            result = (first_position + second_position) as i32
        }
    }
    result
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
