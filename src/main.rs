use std::io::Read;
use clap::Parser;

pub mod day1;

#[derive(Parser, Debug)]
#[clap(name = "aoc", version = "1.0", author = "Gussi", about = "Advent of Code 2024")]
struct Args {
    #[arg(required(true))]
    day : u8,

    #[arg(required(true))]
    part: u8,
}

fn main() {
    let args = Args::parse();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    match args {
        Args { day, part } => {
            let answer = match day {
                1 => match part {
                    1 => day1::part1(&input),
                    2 => day1::part2(&input),
                    _ => panic!("Invalid part"),
                },
                _ => panic!("Invalid day"),
            };

            println!("Day {} Part {}: {}", day, part, answer);
        }
    }
}
