#[derive(PartialEq)]
enum Safety {
    Safe,
    Unsafe(i32),
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
}

struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new(levels: Vec<i32>) -> Report {
        Report { levels }
    }

    fn safe(&self) -> Safety {
        let direction = match self.levels[0] < self.levels[1] {
            true => Direction::Up,
            false => Direction::Down,
        };

        for i in 0..self.levels.len() - 1 {
            let level = self.levels[i];
            let next_level = self.levels[i + 1];

            if level == next_level {
                return Safety::Unsafe(i as i32);
            }

            match direction {
                Direction::Up => {
                    if level > next_level {
                        return Safety::Unsafe((i + 1) as i32);
                    }
                }
                Direction::Down => {
                    if level < next_level {
                        return Safety::Unsafe(i as i32);
                    }
                }
            }

            let difference = (level - next_level).abs();

            if !(1..=3).contains(&difference) {
                return Safety::Unsafe(i as i32);
            }
        }

        Safety::Safe
    }
}

pub fn first(input: &str) -> usize {
    let reports = get_reports(input);

    reports
        .iter()
        .filter(|&report| report.safe().eq(&Safety::Safe))
        .count()
}

pub fn second(input: &str) -> usize {
    let reports = get_reports(input);

    let mut total_safe_reports = 0;

    for report in reports.iter() {
        for i in 0..report.levels.len() {
            let mut levels = report.levels.clone();
            levels.remove(i);

            let new_report = Report::new(levels);

            if new_report.safe().eq(&Safety::Safe) {
                total_safe_reports += 1;
                break;
            }
        }
    }

    total_safe_reports
}

#[cfg(test)]
const TEST_INPUT: &str = include_str!("../../test/fixture/two.txt");

#[test]
fn test_second() {
    assert_eq!(second(TEST_INPUT), 4);
}

#[test]
fn test_first() {
    assert_eq!(first(TEST_INPUT), 2);
}

fn get_reports(input: &str) -> Vec<Report> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let mut levels = Vec::new();
        for number in line.split(" ") {
            levels.push(number.parse::<i32>().unwrap());
        }
        reports.push(Report { levels });
    }

    reports
}
