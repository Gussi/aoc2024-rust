use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

    _x: usize,
    _y: usize,
    _direction: Direction,
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
            Direction::Up => grid.grid[self.x - 1][self.y] != '.',
            Direction::Down => grid.grid[self.x + 1][self.y] != '.',
            Direction::Left => grid.grid[self.x][self.y - 1] != '.',
            Direction::Right => grid.grid[self.x][self.y + 1] != '.',
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

    fn walk_is_looping(&mut self, grid: &Grid) -> bool {
        let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
        let mut steps = 0;

        while !self.is_facing_edge(grid) {
            self.walk(grid);
            steps += 1;

            if steps > 10000 {
                panic!("Guard has abnormally high number of steps");
            }

            if visited.contains(&(self.x, self.y, self.direction)) {
                self.reset();
                return true;
            }

            visited.insert((self.x, self.y, self.direction));
        }

        self.reset();

        false
    }

    fn reset(&mut self) {
        self.x = self._x;
        self.y = self._y;
        self.direction = self._direction;
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

            _x: 0,
            _y: 0,
            _direction: Direction::Up,
        };

        for (x, row) in grid.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                if grid[x][y] == '^' {
                    guard = Guard { x, y, direction: Direction::Up, _x: x, _y: y, _direction: Direction::Up };
                }
            }
        }

        grid[guard.x][guard.y] = '.';

        (Grid { grid }, guard)
    }

    fn block(&mut self, x: usize, y: usize) -> bool {
        if self.grid[x][y] == '.' {
            self.grid[x][y] = 'O';
            return true;
        }

        false
    }

    fn unblock(&mut self, x: usize, y: usize) {
        if self.grid[x][y] == 'O' {
            self.grid[x][y] = '.';
        }
    }
}

pub fn first(input: &str) -> usize {
    let (grid, mut guard) = Grid::from_input(input);

    guard.walk_until_facing_edge(&grid) as usize
}

pub fn second(input: &str) -> usize {
    let (mut grid, mut guard) = Grid::from_input(input);
    let mut answer = 0;

    for x in 0..grid.grid.len() {
        for y in 0..grid.grid[0].len() {
            if grid.block(x, y) {

                if guard.walk_is_looping(&grid) {
                    answer += 1;
                }

                grid.unblock(x, y);
            }
        }
    }

    answer as usize
}

#[cfg(test)]
const TEST_INPUT: &str = include_str!("../../test/fixture/six.txt");

#[test]
fn test_first() {
    assert_eq!(first(TEST_INPUT), 41);
}

#[test]
fn test_second() {
    assert_eq!(second(TEST_INPUT), 6);
}
