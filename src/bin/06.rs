use std::collections::HashSet;
advent_of_code::solution!(6);

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,    // < ^
            Direction::Up => Direction::Right,   // ^ >
            Direction::Right => Direction::Down, // > v
            Direction::Down => Direction::Left,  // v <
        }
    }
}

impl Direction {
    fn from_char(c: &char) -> Self {
        match *c {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("not allowed"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn add_direction(self, direction: &Direction) -> Self {
        match direction {
            Direction::Left => Position {
                col: self.col - 1,
                ..self
            },
            Direction::Right => Position {
                col: self.col + 1,
                ..self
            },
            Direction::Up => Position {
                row: self.row - 1,
                ..self
            },
            Direction::Down => Position {
                row: self.row + 1,
                ..self
            },
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Grid {
    // performance can be improved by only keeping track of obstacles, and stepping
    // multiple positions at once. Requires a bit more bookkeeping to find the nearest obstacle
    // based on the current position and the current direction.
    grid: Vec<Vec<char>>,
    pos: Position,
    direction: Direction,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let pos = Self::find_start(&grid);
        let char = grid
            .get(pos.row as usize)
            .unwrap()
            .get(pos.col as usize)
            .unwrap();

        let direction = Direction::from_char(char);
        Grid {
            grid,
            pos,
            direction,
        }
    }

    fn find_start(grid: &[Vec<char>]) -> Position {
        for (y, row) in grid.iter().enumerate() {
            match row
                .iter()
                .enumerate()
                .find(|(_, &c)| c == '>' || c == '^' || c == '<' || c == 'v')
            {
                None => continue,
                Some((x, _)) => {
                    return Position {
                        row: y as i32,
                        col: x as i32,
                    }
                }
            }
        }

        panic!("didn't find starting position")
    }

    fn char_at_curr_pos(&self) -> Option<char> {
        self.char_at_pos(self.pos)
    }

    fn char_at_pos(&self, pos: Position) -> Option<char> {
        if pos.col < 0 || pos.row < 0 {
            return None;
        }

        match self.grid.get(pos.row as usize) {
            Some(row) => row.get(pos.col as usize).copied(),
            None => None,
        }
    }

    fn move_in_direction(&mut self) {
        let new_pos = self.pos.add_direction(&self.direction);

        match self.char_at_pos(new_pos) {
            None => self.pos = new_pos, // allow to go out of bounds
            Some(c) => match c {
                '#' => self.direction = self.direction.rotate_right(),
                '.' | '^' | 'v' | '<' | '>' => self.pos = new_pos,
                _ => {}
            },
        };
    }

    fn set_value(&mut self, x: usize, y: usize, val: char) {
        let row = self.grid.get_mut(y).unwrap();
        let value = row.get_mut(x).unwrap();
        *value = val;
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut grid = parse_input(input);
    let mut visited = HashSet::new();

    while grid.char_at_curr_pos().is_some() {
        visited.insert(grid.pos);
        grid.move_in_direction();
    }

    Some(visited.len() as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    fn contains_loop(grid: &mut Grid) -> bool {
        let mut visited: HashSet<(Position, Direction)> = HashSet::new();

        while grid.char_at_curr_pos().is_some() {
            let key = (grid.pos, grid.direction);
            if visited.contains(&key) {
                return true;
            }

            visited.insert((grid.pos, grid.direction));
            grid.move_in_direction();
        }

        false
    }

    let mut grid = parse_input(input);
    let n_rows = grid.grid.len();
    let n_cols = grid.grid[0].len();

    let initial_direction = grid.direction;
    let initial_position = grid.pos;

    let mut result = 0;

    for y in 0..n_rows {
        for x in 0..n_cols {
            let c = grid.char_at_pos(Position {
                row: y as i32,
                col: x as i32,
            }).unwrap();

            if c != '.' {
                continue;
            }

            grid.set_value(x, y, '#');
            if contains_loop(&mut grid) {
                result += 1;
            }
            grid.set_value(x, y, '.');
            grid.pos = initial_position;
            grid.direction = initial_direction;
        }
    }

    Some(result)
}

fn parse_input(input: &str) -> Grid {
    Grid::new(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_parse_input() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

        let parsed = parse_input(input);

        let expected = Grid::new(vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ]);

        assert_eq!(parsed, expected);
    }
}
