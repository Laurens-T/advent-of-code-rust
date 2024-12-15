use std::collections::HashMap;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, movements, mut start) = parse_input(input);

    // println!("Initial State:");
    // print_grid(&grid, start);

    for c in movements {
        start = step(&mut grid, c, start);
        // println!("Move {c}:");
        // print_grid(&grid, start);
        // println!("");
    }

    let result: u32 = grid
        .iter()
        .filter(|&(_, c)| *c == 'O')
        .map(|((y, x), _)| {
            (y * 100 + x) as u32
        })
        .sum();

    Some(result)
}

fn print_grid(grid: &HashMap<(i32, i32), char>, start: (i32, i32)) {
    for y in 0..8 {
        for x in 0..8 {
            print!(
                "{}{}",
                if (y, x) == start {
                    '@'
                } else {
                    *grid.get(&(y, x)).unwrap()
                },
                if x == 7 { "\n" } else { "" }
            );
        }
    }
}

fn step(grid: &mut HashMap<(i32, i32), char>, step: char, pos: (i32, i32)) -> (i32, i32) {
    let direction = match step {
        '>' => (0, 1),
        '<' => (0, -1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => (0, 0),
    };

    let new_pos = (pos.0 + direction.0, pos.1 + direction.1);
    if let Some(c) = grid.get(&new_pos) {
        match c {
            '#' => pos,     // can't move
            '.' => new_pos, // can move here
            'O' => {
                let mut space = new_pos;
                let empty_space: Option<(i32, i32)> = loop {
                    match grid.get(&space) {
                        Some('#') => break None,
                        Some('.') => {
                            break Some(space);
                        }
                        Some('O') => {
                            space = (space.0 + direction.0, space.1 + direction.1);
                            continue;
                        }
                        _ => break Some(pos),
                    }
                };

                match empty_space {
                    None => pos,
                    Some(space) => {
                        grid.insert(space, 'O');
                        grid.insert(new_pos, '.');

                        new_pos
                    }
                }
            }
            _ => panic!("eek"),
        }
    } else {
        pos
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> (HashMap<(i32, i32), char>, Vec<char>, (i32, i32)) {
    let mut split = input.split("\n\n");
    let grid = split.next().unwrap();
    let movements = split.next().unwrap();
    let mut start: Option<(i32, i32)> = None;

    let grid: HashMap<(i32, i32), char> = grid
        .lines()
        .map(|line| line.trim())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '@' => {
                        start = Some((y as i32, x as i32));
                        ((y as i32, x as i32), '.')
                    }
                    _ => ((y as i32, x as i32), c),
                })
                .collect::<Vec<((i32, i32), char)>>()
        })
        .collect();

    let movements: Vec<char> = movements
        .lines()
        .map(|line| line.trim())
        .flat_map(|line| line.chars())
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
        let (mut grid, movements, start) = parse_input(input);

        // ########
        // #..O.O.#
        // ##@.O..#
        let new_pos = step(&mut grid, '<', start);
        assert_eq!(new_pos, start);

        // ########
        // #..O.O.#
        // ##@.O..#
        let new_pos = step(&mut grid, '^', start);
        assert_eq!(new_pos, (1, 2));

        // ########
        // #..O.O.#
        // ##.@O..#
        let new_pos = step(&mut grid, '>', (2, 2));
        assert_eq!(new_pos, (2, 3));

        // ########
        // #..O.O.#
        // ##..@O.#
        assert_eq!(grid.get(&(2, 3)), Some(&'.'));
        assert_eq!(grid.get(&(2, 4)), Some(&'O'));

        let new_pos = step(&mut grid, '>', (2, 3));
        assert_eq!(new_pos, (2, 4));

        assert_eq!(grid.get(&(2, 4)), Some(&'.'));
        assert_eq!(grid.get(&(2, 5)), Some(&'O'));

        let new_pos = step(&mut grid, '>', (2, 4));
        assert_eq!(new_pos, (2, 5));

        assert_eq!(grid.get(&(2, 5)), Some(&'.'));
        assert_eq!(grid.get(&(2, 6)), Some(&'O'));

        let new_pos = step(&mut grid, '>', (2, 5));
        assert_eq!(new_pos, (2, 5));

        assert_eq!(grid.get(&(2, 5)), Some(&'.'));
        assert_eq!(grid.get(&(2, 6)), Some(&'O'));
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
            vec!['<', '^', '^', '>', '>', '>', 'v', 'v', '<', 'v', '>', '>', 'v', '<', '<']
        );
        assert_eq!(
            grid,
            HashMap::from([
                ((0, 0), '#'),
                ((0, 1), '#'),
                ((0, 2), '#'),
                ((0, 3), '#'),
                ((0, 4), '#'),
                ((0, 5), '#'),
                ((0, 6), '#'),
                ((0, 7), '#'),
                ((1, 0), '#'),
                ((1, 1), '.'),
                ((1, 2), '.'),
                ((1, 3), 'O'),
                ((1, 4), '.'),
                ((1, 5), 'O'),
                ((1, 6), '.'),
                ((1, 7), '#'),
                ((2, 0), '#'),
                ((2, 1), '#'),
                ((2, 2), '.'),
                ((2, 3), '.'),
                ((2, 4), 'O'),
                ((2, 5), '.'),
                ((2, 6), '.'),
                ((2, 7), '#'),
                ((3, 0), '#'),
                ((3, 1), '.'),
                ((3, 2), '.'),
                ((3, 3), '.'),
                ((3, 4), 'O'),
                ((3, 5), '.'),
                ((3, 6), '.'),
                ((3, 7), '#'),
                ((4, 0), '#'),
                ((4, 1), '.'),
                ((4, 2), '#'),
                ((4, 3), '.'),
                ((4, 4), 'O'),
                ((4, 5), '.'),
                ((4, 6), '.'),
                ((4, 7), '#'),
                ((5, 0), '#'),
                ((5, 1), '.'),
                ((5, 2), '.'),
                ((5, 3), '.'),
                ((5, 4), 'O'),
                ((5, 5), '.'),
                ((5, 6), '.'),
                ((5, 7), '#'),
                ((6, 0), '#'),
                ((6, 1), '.'),
                ((6, 2), '.'),
                ((6, 3), '.'),
                ((6, 4), '.'),
                ((6, 5), '.'),
                ((6, 6), '.'),
                ((6, 7), '#'),
                ((7, 0), '#'),
                ((7, 1), '#'),
                ((7, 2), '#'),
                ((7, 3), '#'),
                ((7, 4), '#'),
                ((7, 5), '#'),
                ((7, 6), '#'),
                ((7, 7), '#'),
            ])
        );
    }
}
