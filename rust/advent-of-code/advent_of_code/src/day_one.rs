use super::error::Error;
use math::round::floor;
use std::fs;

const FILENAME1: &str = "./inputs/day_one/input.txt";
const FILENAME2: &str = "./inputs/day_one/input2.txt";

pub fn run() -> Result<Vec<i32>, Error> {
    let first_mass_total = calculate_first_mass();
    let second_mass_total = calculate_second_mass();
    Ok(vec![first_mass_total, second_mass_total])
}

fn read_input(file_name: &str) -> Vec<f64> {
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
        .split('\n')
        .map(|v| v.parse::<f64>().unwrap())
        .collect::<Vec<f64>>()
}
fn calculate_first_mass() -> i32 {
    read_input(FILENAME1)
        .into_iter()
        .fold(0, |acc, x| acc + (floor(x / 3.0, 0) - 2.0) as i32)
}

fn calculate_second_mass() -> i32 {
    let parsed_lines = read_input(FILENAME2);

    let mut total: f64 = 0.0;
    for mass in parsed_lines {
        let mut total_additional_fuel = 0.0;
        sum_additional_fuel(mass, &mut total_additional_fuel);
        total += total_additional_fuel;
    }

    total as i32
}

fn sum_additional_fuel(mass: f64, total_fuel_required: &mut f64) {
    let mass_calc = floor(mass / 3.0, 0) - 2.0;
    if mass_calc as i32 > 0 {
        *total_fuel_required += mass_calc;
        sum_additional_fuel(mass_calc, total_fuel_required);
    }
}
