use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(18);

const SIZE: usize = 71;
const NUM_BYTES: usize = 1024;

pub fn part_one(input: &str) -> Option<usize> {
    solve_part_one(input, SIZE, NUM_BYTES)
}

fn solve_part_one(input: &str, size: usize, num_bytes: usize) -> Option<usize> {
    let (_, positions) = parse_input(input).expect("advent of code");
    
    let positions: &[Vec2] = &positions.as_slice()[..num_bytes];
    let corrupted: HashSet<Vec2> = positions.iter().copied().collect();

    shortest_path(&corrupted, size)
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Item {
    cost: usize,
    pos: Vec2,
}

fn shortest_path(corrupted: &HashSet<Vec2>, size: usize) -> Option<usize> {
    let size = size as i32;
    let mut cost: HashMap<Vec2, usize> = HashMap::new();
    let mut pq: BinaryHeap<Reverse<Item>> = BinaryHeap::new();
    let mut min_cost = usize::MAX;
    
    cost.insert(Vec2(0, 0), 0);
    pq.push(Reverse(Item {
        cost: 0,
        pos: Vec2(0, 0),
    }));

    let directions = [Vec2::LEFT, Vec2::RIGHT, Vec2::UP, Vec2::DOWN];
    let target = Vec2(size - 1, size - 1);

    while let Some(Reverse(item)) = pq.pop() {
        if item.cost >= min_cost {
            break;
        }

        if item.pos == target {
            min_cost = min_cost.min(item.cost);
            continue;
        }

        for offset in directions.iter() {
            let (new_x, new_y) = (item.pos.0 + offset.0, item.pos.1 + offset.1);

            if !(0..size).contains(&new_x) || !(0..size).contains(&new_y) {
                continue;
            }

            let new_item = Item {
                cost: item.cost + 1,
                pos: Vec2(new_x, new_y),
            };

            if corrupted.contains(&new_item.pos) {
                continue;
            }

            if cost.get(&new_item.pos).is_some_and(|&c| new_item.cost >= c) {
                continue;
            }

            cost.insert(new_item.pos, new_item.cost);
            pq.push(Reverse(new_item));
        }
    }
    
    cost.get(&target).copied()
}

// fn print_grid(size: usize, positions: &[Vec2]) -> String {
//     let mut grid = vec![vec!['.'; size]; size];
// 
//     for pos in positions {
//         grid[pos.1 as usize][pos.0 as usize] = '#';
//     }
// 
//     let mut output = String::new();
// 
//     for (y, row) in grid.iter().enumerate() {
//         output.extend(row);
//         if y < size - 1 {
//             output.push('\n');
//         }
//     }
// 
//     output
// }

fn parse_input(input: &str) -> IResult<&str, Vec<Vec2>> {
    separated_list0(line_ending, parse_vec2)(input)
}

fn parse_vec2(input: &str) -> IResult<&str, Vec2> {
    let (input, (x, y)) = separated_pair(
        nom::character::complete::i32,
        tag(","),
        nom::character::complete::i32,
    )(input)?;

    Ok((input, Vec2(x, y)))
}

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct Vec2(i32, i32);

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}, {})", self.0, self.1)
    }
}

impl Vec2 {
    const LEFT: Self = Vec2(-1, 0);
    const RIGHT: Self = Vec2(1, 0);
    const UP: Self = Vec2(0, -1);
    const DOWN: Self = Vec2(0, 1);
}

pub fn part_two(input: &str) -> Option<Vec2> {
    solve_part_two(input, SIZE)
}

fn solve_part_two(input: &str, size: usize) -> Option<Vec2> {
    let (_, positions) = parse_input(input).expect("advent of code");

    let positions: &[Vec2] = positions.as_slice();
    let mut i = 0;
    let result = loop {
        let corrupted: HashSet<Vec2> = positions[..i].iter().copied().collect();
        
        if shortest_path(&corrupted, size).is_none() {
            break positions[i-1];
        }

        i += 1;
    };

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SIZE: usize = 7;
    const NUM_BYTES: usize = 12;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = solve_part_one(input, TEST_SIZE, NUM_BYTES);

        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(
            &advent_of_code::template::read_file("examples", DAY),
            TEST_SIZE,
        );
        assert_eq!(result, Some(Vec2(6, 1)));
    }
}
