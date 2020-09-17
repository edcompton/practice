use math::round::floor;
use std::fs;

const FILENAME1: &str = "./input.txt";
const FILENAME2: &str = "./input2.txt";

fn main() {
    let first_mass_total = calculate_first_mass();
    let second_mass_total = calculate_second_mass();
    println!("first mass total {}", first_mass_total);
    println!("second mass total {}", second_mass_total);
}

fn calculate_first_mass() -> i32 {
    fs::read_to_string(FILENAME1)
        .expect("Something went wrong reading the file")
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.parse::<f64>().unwrap())
        .fold(0, |acc, x| acc + (floor(x / 3.0, 0) - 2.0) as i32)
}

fn calculate_second_mass() -> i32 {
    let parsed_lines = fs::read_to_string(FILENAME2)
        .expect("Something went wrong reading the file")
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let mut total: f64 = 0.0;
    for mass in parsed_lines {
        let calculated_mass = calculate_additional_fuel(mass);
        total += calculated_mass;
    }

    total as i32
}

fn calculate_additional_fuel(mass: f64) -> f64 {
    let mut total_fuel_required = 0.0;
    sum_fuel(mass, &mut total_fuel_required);
    total_fuel_required
}

fn sum_fuel(mass: f64, total: &mut f64) {
    let mass_calc = floor(mass / 3.0, 0) - 2.0;
    if mass_calc as i32 > 0 {
        *total += mass_calc;
        sum_fuel(mass_calc, total);
    } else {
        unreachable!()
    }
}
