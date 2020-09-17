use math::round::floor;
use std::fs;

const FILENAME: &str = "./input.txt";

fn main() {
    let lines = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    let mut total: f64 = 0.0;
    let lines: Vec<&str> = lines.split('\n').collect();
    for line in lines.iter() {
        let mass = line.parse::<f64>().unwrap();
        total += floor(mass / 3.0, 0) - 2.0;
    }

    println!("{}", total);
}
