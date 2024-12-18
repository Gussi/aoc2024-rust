#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
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

    fn check_xmas(&self, point: Point) -> i32 {
        let word = "XMAS".to_string();
        let mut total = 0;

        for direction in ONE_DIRECTIONS.iter() {
            let mut found = true;
            let mut point = point;

            for letter in word.chars() {
                if self.out_of_bounds(&point) {
                    found = false;
                    break;
                }

                if self.char_at_point(point).unwrap() != letter {
                    found = false;
                    break;
                }

                point = point.add(*direction);
            }

            if found {
                total += 1;
            }
        }

        total
    }

    fn check_x_mas(&self, point: Point) -> bool {
        if self.char_at_point(point).unwrap() != 'A' {
            return false;
        }

        let diag_left = match self.char_at_point(point.add(Point { x: -1, y: -1 })) {
            Some('M') => matches!(self.char_at_point(point.add(Point { x: 1, y: 1 })), Some('S')),
            Some('S') => matches!(self.char_at_point(point.add(Point { x: 1, y: 1 })), Some('M')),
            _ => false,
        };

        let diag_right = match self.char_at_point(point.add(Point { x: -1, y: 1 })) {
            Some('M') => matches!(self.char_at_point(point.add(Point { x: 1, y: -1 })), Some('S')),
            Some('S') => matches!(self.char_at_point(point.add(Point { x: 1, y: -1 })), Some('M')),
            _ => false,
        };

        diag_left && diag_right
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

pub fn first(input: &str) -> usize {
    let grid = parse_input(input);
    let mut total = 0;

    for (x, row) in grid.grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            total += grid.check_xmas(Point { x: x as i32, y: y as i32 });
        }
    }

    total as usize
}

#[test]
pub fn test_first() {
    let input = include_str!("../../test/fixture/four.txt");
    assert_eq!(first(input), 18);
}

pub fn second(input: &str) -> usize {
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

    total as usize
}

#[test]
pub fn test_second() {
    let input = include_str!("../../test/fixture/four.txt");
    assert_eq!(second(input), 9);
}

fn parse_input(input: &str) -> Grid {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid { grid }
}
