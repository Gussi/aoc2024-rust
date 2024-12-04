pub mod part {
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    struct Grid {
        grid: Vec<Vec<char>>,
    }

    impl Grid {
        fn out_of_bounds(&self, point: &Point) -> bool {
            point.x < 0 || point.x >= self.grid.len() as i32 || point.y < 0 || point.y >= self.grid[point.x as usize].len() as i32
        }

        fn char_at_point(&self, point: Point) -> Option<char> {
            if self.out_of_bounds(&point) {
                return None;
            }

            match self.grid.get(point.x as usize) {
                Some(row) => row.get(point.y as usize).cloned(),
                None => None,
            }
        }

        fn check_x_mas(&self, point: Point) -> bool {
            if self.char_at_point(point).unwrap() != 'A' {
                return false;
            }

            let diag_left = match self.char_at_point(Point { x: point.x - 1, y: point.y - 1 }) {
                Some('M') => match self.char_at_point(Point { x: point.x + 1, y: point.y + 1 }) {
                    Some('S') => true,
                    _ => false,
                },
                Some('S') => match self.char_at_point(Point { x: point.x + 1, y: point.y + 1 }) {
                    Some('M') => true,
                    _ => false,
                },
                _ => return false,
            };

            if !diag_left {
                return false;
            }

             let diag_right = match self.char_at_point(Point { x: point.x - 1, y: point.y + 1 }) {
                Some('M') => match self.char_at_point(Point { x: point.x + 1, y: point.y - 1 }) {
                    Some('S') => true,
                    _ => false,
                },
                Some('S') => match self.char_at_point(Point { x: point.x + 1, y: point.y - 1 }) {
                    Some('M') => true,
                    _ => false,
                },
                _ => return false,
            };

            if !diag_right {
                return false;
            }

            true
        }
    }

    const ONE_DIRECTIONS: [Point; 8] = [
        Point { x: -1, y: -1 },
        Point { x: -1, y: 0 },
        Point { x: -1, y: 1 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: -1 },
        Point { x: 1, y: 0 },
        Point { x: 1, y: 1 },
    ];

    pub fn one(input: &str) -> i32 {
        let grid = parse_input(input);
        let word = "XMAS".to_string();

        let mut total = 0;

        for (x, row) in grid.grid.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                // Search for the word in every direction
                for direction in ONE_DIRECTIONS.iter() {
                    let mut point = Point { x: x as i32, y: y as i32 };
                    let mut found = true;

                    for letter in word.chars() {
                        if grid.out_of_bounds(&point) {
                            found = false;
                            break;
                        }

                        if grid.char_at_point(point).unwrap() != letter {
                            found = false;
                            break;
                        }

                        point.x += direction.x;
                        point.y += direction.y;
                    }

                    if found {
                        total += 1;
                    }
                }
            }
        }

        total
    }

    #[test]
    pub fn one_test() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(one(input), 18);
    }

    pub fn two(input: &str) -> i32 {
        let grid = parse_input(input);
        let mut total = 0;

        for (x, row) in grid.grid.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                let point = Point { x: x as i32, y: y as i32 };

                if grid.check_x_mas(point) {
                    total += 1;
                }
            }
        }

        total
    }

    #[test]
    pub fn two_test() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(two(input), 9);
    }

    fn parse_input(input: &str) -> Grid {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Grid { grid }
    }
}
