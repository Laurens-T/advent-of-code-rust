use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

const OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, HashSet::new)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, Vec::new) // will cause quite some duplicate work to be done
}

fn solve<C, F>(input: &str, create_positions: F) -> Option<u32>
where
    C: FromIterator<(i32, i32)> + IntoIterator<Item = (i32, i32)> + Extend<(i32, i32)>,
    F: Fn() -> C,
{
    let grid = parse_input(input);

    let result: u32 = grid
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(pos, _)| paths_to_9(&grid, [*pos], create_positions()))
        .sum();

    Some(result)
}

fn paths_to_9<I, C>(grid: &HashMap<(i32, i32), u32>, start: I, mut positions: C) -> u32
where
    I: IntoIterator<Item = (i32, i32)>,
    C: FromIterator<(i32, i32)> + IntoIterator<Item = (i32, i32)> + Extend<(i32, i32)>,
{
    positions.extend(start);

    let mut height = 0;
    while height <= 8 {
        positions = positions
            .into_iter()
            .flat_map(|pos| {
                OFFSETS
                    .iter()
                    .map(move |offset| (pos.0 + offset.0, pos.1 + offset.1))
                    .filter(|new_location| {
                        grid.get(new_location)
                            .is_some_and(|value| *value == height + 1)
                    })
                    .collect::<C>()
            })
            .collect();

        height += 1;
    }

    positions.into_iter().count() as u32
}

fn parse_input(input: &str) -> HashMap<(i32, i32), u32> {
    input
        .lines()
        .map(|line| line.trim())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((y as i32, x as i32), c.to_digit(10).unwrap()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let result = part_one(input);
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
