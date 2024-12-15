use std::collections::HashMap;

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Object {
    Wall,
    Space,
    Box,
    // BoxLeftEdge,
    // BoxRightEdge,
}

impl Object {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Space,
            'O' => Self::Box,
            // '[' => Self::BoxLeftEdge,
            // ']' => Self::BoxRightEdge,
            _ => panic!(),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Space => '.',
            Self::Box => 'O',
            // Self::BoxLeftEdge => '[',
            // Self::BoxRightEdge => ']',
        }
    }
}

impl Direction {
    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, movements, mut start) = parse_input(input);
    for c in movements {
        start = step(&mut grid, c, start);
    }

    let result: u32 = grid
        .iter()
        .filter(|&(_, c)| *c == Object::Box)
        .map(|((y, x), _)| (y * 100 + x) as u32)
        .sum();

    Some(result)
}

fn step(grid: &mut HashMap<(i32, i32), Object>, step: Direction, pos: (i32, i32)) -> (i32, i32) {
    let direction = step.to_tuple();

    let new_pos = (pos.0 + direction.0, pos.1 + direction.1);

    let &c = match grid.get(&new_pos) {
        None => return pos,
        Some(c) => c,
    };

    match c {
        Object::Wall => pos,      // can't move
        Object::Space => new_pos, // can move here
        Object::Box => {
            let mut space = new_pos;
            let empty_space: Option<(i32, i32)> = loop {
                match grid.get(&space) {
                    Some(Object::Wall) => break None,
                    Some(Object::Space) => {
                        break Some(space);
                    }
                    Some(Object::Box) => {
                        space = (space.0 + direction.0, space.1 + direction.1);
                        continue;
                    }
                    _ => break Some(pos),
                }
            };

            match empty_space {
                None => pos,
                Some(space) => {
                    grid.insert(space, Object::Box);
                    grid.insert(new_pos, Object::Space);

                    new_pos
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> (HashMap<(i32, i32), Object>, Vec<Direction>, (i32, i32)) {
    let mut split = input.split("\n\n");
    let grid = split.next().unwrap();
    let movements = split.next().unwrap();
    let mut start: Option<(i32, i32)> = None;

    let grid: HashMap<(i32, i32), Object> = grid
        .lines()
        .map(|line| line.trim())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '@' => {
                        start = Some((y as i32, x as i32));
                        ((y as i32, x as i32), Object::Space)
                    }
                    _ => ((y as i32, x as i32), Object::from_char(c)),
                })
                .collect::<Vec<((i32, i32), Object)>>()
        })
        .collect();

    let movements: Vec<Direction> = movements
        .lines()
        .map(|line| line.trim())
        .flat_map(|line| line.chars())
        .map(Direction::from_char)
        .collect();

    (grid, movements, start.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_step() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (mut grid, _, start) = parse_input(input);

        // ########
        // #..O.O.#
        // ##@.O..#
        let new_pos = step(&mut grid, Direction::Left, start);
        assert_eq!(new_pos, start);

        // ########
        // #..O.O.#
        // ##@.O..#
        let new_pos = step(&mut grid, Direction::Up, start);
        assert_eq!(new_pos, (1, 2));

        // ########
        // #..O.O.#
        // ##.@O..#
        let new_pos = step(&mut grid, Direction::Right, (2, 2));
        assert_eq!(new_pos, (2, 3));

        // ########
        // #..O.O.#
        // ##..@O.#
        assert_eq!(grid.get(&(2, 3)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 4)), Some(&Object::Box));

        let new_pos = step(&mut grid, Direction::Right, (2, 3));
        assert_eq!(new_pos, (2, 4));

        assert_eq!(grid.get(&(2, 4)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 5)), Some(&Object::Box));

        let new_pos = step(&mut grid, Direction::Right, (2, 4));
        assert_eq!(new_pos, (2, 5));

        assert_eq!(grid.get(&(2, 5)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 6)), Some(&Object::Box));

        let new_pos = step(&mut grid, Direction::Right, (2, 5));
        assert_eq!(new_pos, (2, 5));

        assert_eq!(grid.get(&(2, 5)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 6)), Some(&Object::Box));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (grid, movements, start) = parse_input(input);

        assert_eq!(start, (2, 2));
        assert_eq!(
            movements,
            vec![
                Direction::Left,
                Direction::Up,
                Direction::Up,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Down,
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left
            ]
        );
        assert_eq!(
            grid,
            HashMap::from([
                ((0, 0), Object::Wall),
                ((0, 1), Object::Wall),
                ((0, 2), Object::Wall),
                ((0, 3), Object::Wall),
                ((0, 4), Object::Wall),
                ((0, 5), Object::Wall),
                ((0, 6), Object::Wall),
                ((0, 7), Object::Wall),
                ((1, 0), Object::Wall),
                ((1, 1), Object::Space),
                ((1, 2), Object::Space),
                ((1, 3), Object::Box),
                ((1, 4), Object::Space),
                ((1, 5), Object::Box),
                ((1, 6), Object::Space),
                ((1, 7), Object::Wall),
                ((2, 0), Object::Wall),
                ((2, 1), Object::Wall),
                ((2, 2), Object::Space),
                ((2, 3), Object::Space),
                ((2, 4), Object::Box),
                ((2, 5), Object::Space),
                ((2, 6), Object::Space),
                ((2, 7), Object::Wall),
                ((3, 0), Object::Wall),
                ((3, 1), Object::Space),
                ((3, 2), Object::Space),
                ((3, 3), Object::Space),
                ((3, 4), Object::Box),
                ((3, 5), Object::Space),
                ((3, 6), Object::Space),
                ((3, 7), Object::Wall),
                ((4, 0), Object::Wall),
                ((4, 1), Object::Space),
                ((4, 2), Object::Wall),
                ((4, 3), Object::Space),
                ((4, 4), Object::Box),
                ((4, 5), Object::Space),
                ((4, 6), Object::Space),
                ((4, 7), Object::Wall),
                ((5, 0), Object::Wall),
                ((5, 1), Object::Space),
                ((5, 2), Object::Space),
                ((5, 3), Object::Space),
                ((5, 4), Object::Box),
                ((5, 5), Object::Space),
                ((5, 6), Object::Space),
                ((5, 7), Object::Wall),
                ((6, 0), Object::Wall),
                ((6, 1), Object::Space),
                ((6, 2), Object::Space),
                ((6, 3), Object::Space),
                ((6, 4), Object::Space),
                ((6, 5), Object::Space),
                ((6, 6), Object::Space),
                ((6, 7), Object::Wall),
                ((7, 0), Object::Wall),
                ((7, 1), Object::Wall),
                ((7, 2), Object::Wall),
                ((7, 3), Object::Wall),
                ((7, 4), Object::Wall),
                ((7, 5), Object::Wall),
                ((7, 6), Object::Wall),
                ((7, 7), Object::Wall),
            ])
        );
    }
}
