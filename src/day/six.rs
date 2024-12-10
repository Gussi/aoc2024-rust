pub mod part {
    use std::collections::HashSet;

    #[derive(Copy, Clone)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Copy, Clone)]
    struct Guard {
        x: usize,
        y: usize,
        direction: Direction,
    }

    impl Guard {
        fn walk(&mut self, grid: &Grid) {
            while self.is_blocked(grid) {
                self.turn();
            }

            match self.direction {
                Direction::Up => self.x -= 1,
                Direction::Down => self.x += 1,
                Direction::Left => self.y -= 1,
                Direction::Right => self.y += 1,
            }
        }

        fn is_blocked(&self, grid: &Grid) -> bool {
            match self.direction {
                Direction::Up => grid.grid[self.x - 1][self.y] == '#',
                Direction::Down => grid.grid[self.x + 1][self.y] == '#',
                Direction::Left => grid.grid[self.x][self.y - 1] == '#',
                Direction::Right => grid.grid[self.x][self.y + 1] == '#',
            }
        }

        fn turn(&mut self) {
            match self.direction {
                Direction::Up => self.direction = Direction::Right,
                Direction::Down => self.direction = Direction::Left,
                Direction::Left => self.direction = Direction::Up,
                Direction::Right => self.direction = Direction::Down,
            }
        }

        fn is_facing_edge(&self, grid: &Grid) -> bool {
            match self.direction {
                Direction::Up => self.x == 0,
                Direction::Down => self.x == grid.grid.len() - 1,
                Direction::Left => self.y == 0,
                Direction::Right => self.y == grid.grid[0].len() - 1,
            }
        }

        fn walk_until_facing_edge(&mut self, grid: &Grid) -> usize {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();

            while !self.is_facing_edge(grid) {
                self.walk(grid);
                visited.insert((self.x, self.y));
            }

            visited.len()
        }
    }

    struct Grid {
        grid: Vec<Vec<char>>,
    }


    impl Grid {

        fn from_input(input: &str) -> (Grid, Guard) {
            let mut grid: Vec<Vec<char>> = Vec::new();

            for line in input.lines() {
                grid.push(line.chars().collect());
            }

            let mut guard = Guard {
                x: 0,
                y: 0,
                direction: Direction::Up,
            };
            for (x, row) in grid.iter().enumerate() {
                for (y, _) in row.iter().enumerate() {
                    if grid[x][y] == '^' {
                        guard = Guard { x, y, direction: Direction::Up };
                    }
                }
            }

            (Grid { grid }, guard)
        }
    }
    
    pub fn one(input: &str) -> i32 {
        let (grid, mut guard) = Grid::from_input(input);

        guard.walk_until_facing_edge(&grid) as i32
    }

    pub fn two(_input: &str) -> i32 {
        0
    }

    #[cfg(test)]
    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        assert_eq!(one(TEST_INPUT), 41);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(two(TEST_INPUT), 0);
    }
}
