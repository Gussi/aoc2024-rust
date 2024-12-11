use std::io::Read;
use clap::Parser;

pub mod day;

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

    let Args { day, part } = args;
    {
        let answer = match day {
            1 => match part {
                1 => day::one::first(&input),
                2 => day::one::second(&input),
                _ => panic!("Invalid part"),
            },
            2 => match part {
                1 => day::two::first(&input),
                2 => day::two::second(&input),
                _ => panic!("Invalid part"),
            },
            3 => match part {
                1 => day::three::first(&input),
                2 => day::three::second(&input),
                _ => panic!("Invalid part"),
            },
            4 => match part {
                1 => day::four::first(&input),
                2 => day::four::second(&input),
                _ => panic!("Invalid part"),
            },
            5 => match part {
                1 => day::five::first(&input),
                2 => day::five::second(&input),
                _ => panic!("Invalid part"),
            },
            6 => match part {
                1 => day::six::first(&input),
                2 => day::six::second(&input),
                _ => panic!("Invalid part"),
            },
            7 => match part {
                1 => day::seven::first(&input),
                2 => day::seven::second(&input),
                _ => panic!("Invalid part"),
            },
            _ => panic!("Invalid day"),
        };

        println!("Day {} Part {}: {}", day, part, answer);
    }
}
