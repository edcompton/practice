use super::error::Error;
use math::round::floor;
use std::fs;

const FILENAME1: &str = "./inputs/day_one/input.txt";
const FILENAME2: &str = "./inputs/day_one/input2.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    let first_mass_total = total_fuel_requirement();
    let second_mass_total = calculate_additional_total_fuel();
    Ok(vec![first_mass_total, second_mass_total])
}

fn read_input(file_name: &str) -> Vec<i32> {
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
        .split('\n')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
fn total_fuel_requirement() -> i32 {
    read_input(FILENAME1)
        .into_iter()
        .fold(0, |acc, x| acc + (floor(x as f64 / 3.0, 0) as i32 - 2))
}

fn calculate_additional_total_fuel() -> i32 {
    let parsed_lines = read_input(FILENAME2);

    let mut total: i32 = 0;
    for mass in parsed_lines {
        let mut total_additional_fuel = 0;
        sum_additional_fuel(mass, &mut total_additional_fuel);
        total += total_additional_fuel;
    }

    total as i32
}

fn sum_additional_fuel(mass: i32, total_fuel_required: &mut i32) {
    let mass_calc = floor(mass as f64 / 3.0, 0) as i32 - 2;
    if mass_calc as i32 > 0 {
        *total_fuel_required += mass_calc;
        sum_additional_fuel(mass_calc, total_fuel_required);
    }
}
