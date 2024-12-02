use std::io::Read;

use clap::Parser;

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

    // Get input from stdin, multiple lines
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();


    match args {
        Args { day, part } => {
            let answer = match day {
                1 => match part {
                    1 => day1::part1(&input),
                    _ => panic!("Invalid part"),
                },
                _ => panic!("Invalid day"),
            };

            println!("Day {} Part {}: {}", day, part, answer);
        }
    }
}

pub mod day1 {
    pub fn part1(input: &str) -> i32 {
        let mut first_list_of_numbers = Vec::new();
        let mut second_list_of_numbers = Vec::new();

        for line in input.lines() {
            let mut numbers = line.split("   ");
            first_list_of_numbers.push(numbers.next().unwrap().parse::<i32>().unwrap());
            second_list_of_numbers.push(numbers.next().unwrap().parse::<i32>().unwrap());
        }

        first_list_of_numbers.sort();
        second_list_of_numbers.sort();

        let mut total_distance = 0;
        for (first, second) in first_list_of_numbers.iter().zip(second_list_of_numbers.iter()) {
            total_distance += (first - second).abs();
        }

        total_distance
    }

    #[test]
    pub fn test_part1() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part1(input), 11);
    }
}
