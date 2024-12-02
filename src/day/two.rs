pub mod part {
    struct Report {
        levels: Vec<i32>,
    }

    pub fn one(input: &str) -> i32 {
        let reports = get_reports(input);

        return reports
            .iter()
            .filter(|&report| is_level_safe(report) == LevelSafety::Safe)
            .count() as i32;
    }

    #[test]
	fn one_test() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";
        assert_eq!(one(input), 2);
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

    enum LevelSafety {
        Safe,
        Unsafe,
    }

    impl PartialEq for LevelSafety {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (LevelSafety::Safe, LevelSafety::Safe) => true,
                (LevelSafety::Unsafe, LevelSafety::Unsafe) => true,
                _ => false,
            }
        }
    }

    enum Direction {
        Up,
        Down,
    }

    // Implement PartialEq
    impl PartialEq for Direction {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Direction::Up, Direction::Up) => true,
                (Direction::Down, Direction::Down) => true,
                _ => false,
            }
        }
    }

    fn is_level_safe(report: &Report) -> LevelSafety {
        let direction = match report.levels[0] < report.levels[1] {
            true => Direction::Up,
            false => Direction::Down,
        };

        for (level, next_level) in report.levels.iter().zip(report.levels.iter().skip(1)) {

            match direction {
                Direction::Up => {
                    if level > next_level {
                        return LevelSafety::Unsafe;
                    }
                },
                Direction::Down => {
                    if level < next_level {
                        return LevelSafety::Unsafe;
                    }
                },
            }

            let difference = (level - next_level).abs();

            if difference < 1 || difference > 3 {
                return LevelSafety::Unsafe;
            }
        }

        LevelSafety::Safe
    }
}
