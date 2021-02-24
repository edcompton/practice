use std::path::PathBuf;

use advent_of_code::error::Error;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
struct Opt {
    /// Day
    day: usize,
    /// Optional path to input file; if not supplied will read from stdin
    input: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let answers = match opt.day {
        1 => advent_of_code::day_one::run()?,
        2 => advent_of_code::day_two::run()?,
        3 => advent_of_code::day_three::run()?,
        4 => advent_of_code::day_four::run()?,
        5 => advent_of_code::day_five::run()?,
        _ => return Err(Error::Custom(String::from("No valid day given"))),
    };

    println!("\nDay: {day}", day = opt.day);
    for (i, answer) in answers.iter().enumerate() {
        println!(
            "Answer {number}: {answer}\n",
            number = i + 1,
            answer = answer
        )
    }
    Ok(())
}
