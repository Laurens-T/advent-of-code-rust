use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

type Position = (i32, i32);

const DIRECTIONS: [Position; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let regions = get_regions(&grid);

    let result: u32 = regions
        .iter()
        .map(|(c, region)| {
            region.len() as u32
                * region
                    .iter()
                    .map(|&pos| 4 - fence(&grid, pos, *c).len() as u32)
                    .sum::<u32>()
        })
        .sum();

    Some(result)
}

fn get_regions(grid: &HashMap<Position, char>) -> Vec<(char, Vec<Position>)> {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut result = Vec::new();

    for (&k, v) in grid {
        if visited.contains(&k) {
            continue;
        }

        visited.insert(k);

        let mut region: Vec<Position> = vec![];
        let mut positions: Vec<Position> = vec![k];

        while let Some(pos) = positions.pop() {
            region.push(pos);

            DIRECTIONS.iter().for_each(|&offset| {
                let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
                if visited.contains(&new_pos) {
                    return;
                }

                if let Some(neighbor) = grid.get(&new_pos) {
                    if neighbor == v {
                        visited.insert(new_pos);
                        positions.push(new_pos);
                    }
                }
            });
        }

        result.push((*v, region));
    }

    result
}

fn fence(grid: &HashMap<Position, char>, pos: Position, c: char) -> Vec<Position> {
    DIRECTIONS
        .into_iter()
        .map(|(y0, x0)| (pos.0 + y0, pos.1 + x0))
        .filter(|new_pos| grid.get(new_pos).is_some_and(|&c1| c == c1))
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let regions = get_regions(&grid);

    let result: u32 = regions
        .iter()
        .map(|(c, region)| {
            region.len() as u32
                * region
                    .iter()
                    .map(|pos| corner_count(&grid, pos, c))
                    .sum::<u32>()
        })
        .sum();

    Some(result)
}

fn corner_count(grid: &HashMap<Position, char>, n: &Position, group_id: &char) -> u32 {
    let directions = [
        ((0, 1), (1, 0)),
        ((1, 0), (0, -1)),
        ((0, -1), (-1, 0)),
        ((-1, 0), (0, 1)),
    ];
    let mut count = 0;
    let get_at_offset = |(y, x): &(i32, i32)| grid.get(&(y + n.0, x + n.1));

    for (a, b) in directions {
        let test_a = get_at_offset(&a).is_some_and(|c| c == group_id);
        let test_b = get_at_offset(&b).is_some_and(|c| c == group_id);

        if !test_a && !test_b {
            // exterior corner
            // BB
            // BA
            count += 1;
        } else if test_a && test_b {
            //interior corner
            // .A
            // AA
            if get_at_offset(&(a.0 + b.0, a.1 + b.1)).is_some_and(|c| c != group_id) {
                count += 1;
            }
        }
    }
    count
}

fn parse_input(input: &str) -> HashMap<Position, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((y as i32, x as i32), c))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_parse_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let grid = parse_input(input);

        let expected = HashMap::from([
            ((0, 0), 'A'),
            ((0, 1), 'A'),
            ((0, 2), 'A'),
            ((0, 3), 'A'),
            ((1, 0), 'B'),
            ((1, 1), 'B'),
            ((1, 2), 'C'),
            ((1, 3), 'D'),
            ((2, 0), 'B'),
            ((2, 1), 'B'),
            ((2, 2), 'C'),
            ((2, 3), 'C'),
            ((3, 0), 'E'),
            ((3, 1), 'E'),
            ((3, 2), 'E'),
            ((3, 3), 'C'),
        ]);

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_fence_length() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let grid = parse_input(input);

        let result = fence(&grid, (1, 0), 'B');
        assert_eq!(2, result.len());
    }

    #[test]
    fn test_corner_count() {
        // AAAA
        // BBCD
        // BBCC
        // EEEC
        let input = &advent_of_code::template::read_file("examples", DAY);
        let grid = parse_input(input);

        // assert_eq!(2, corner_count(&grid, &(0, 0), &'A'));
        // assert_eq!(0, corner_count(&grid, &(0, 1), &'A'));
        // assert_eq!(0, corner_count(&grid, &(0, 2), &'A'));
        // assert_eq!(2, corner_count(&grid, &(0, 3), &'A'));
        //
        // assert_eq!(2, corner_count(&grid, &(1, 0), &'B'));

        assert_eq!(4, corner_count(&grid, &(1, 3), &'D'));
    }
}
