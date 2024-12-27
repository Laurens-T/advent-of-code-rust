use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::vec2::Vec2;

advent_of_code::solution!(20);

const DIRECTIONS: [Vec2; 4] = [Vec2::LEFT, Vec2::RIGHT, Vec2::UP, Vec2::DOWN];

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 20)
}

pub fn solve(input: &str, max_distance: u32) -> Option<u32> {
    let grid = parse_input(input);
    let costs = dfs(&grid);

    let offsets = calculate_offsets(max_distance);

    let result = costs
        .iter()
        .flat_map(|(pos, &cost)| {
            let mut items = vec![];

            for &offset in offsets.iter() {
                let distance = (offset.0.abs() + offset.1.abs()) as u64;
                let pos_tile = *pos + offset;

                if grid
                    .get(&pos_tile)
                    .is_some_and(|t| t == &Tile::Open || t == &Tile::End)
                {
                    let cost_2 = *costs.get(&pos_tile).unwrap();

                    if cost_2 > cost + distance {
                        let savings = cost_2 - cost - distance;
                        items.push(savings);
                    }
                }
            }

            items
        })
        .filter(|&saving| saving >= 100)
        .count();

    Some(result as u32)
}

// mapping from every open tile to the cost it takes to get there
fn dfs(grid: &HashMap<Vec2, Tile>) -> HashMap<Vec2, u64> {
    let (&start, _) = grid.iter().find(|&(_, tile)| *tile == Tile::Start).unwrap();
    let (&target, _) = grid.iter().find(|&(_, tile)| *tile == Tile::End).unwrap();

    let mut costs: HashMap<Vec2, u64> = HashMap::new();

    let mut prev = Vec2(-10, -10);
    let mut node = Some(start);

    while let Some(n) = node {
        costs.insert(n, costs.len() as u64);

        let new_pos = DIRECTIONS.iter().map(|&dir| n + dir).find(|&new_pos| {
            if new_pos == prev {
                return false;
            }

            grid.get(&new_pos).is_some_and(|t| t == &Tile::Open)
        });

        prev = n;
        node = new_pos;
    }

    costs.insert(target, costs.len() as u64);

    if !costs.contains_key(&target) {
        panic!("oops")
    }

    costs
}

/// Generate a grid of at most distance 20 around (0,0)
fn calculate_offsets(max_distance: u32) -> HashSet<Vec2> {
    let mut offsets: HashSet<Vec2> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((Vec2(0, 0), 0));

    while let Some((current_pos, distance)) = queue.pop_front() {
        if distance > max_distance {
            break;
        }

        offsets.insert(current_pos);

        for dir in DIRECTIONS {
            let next_pos = current_pos + dir;
            if !offsets.contains(&next_pos) {
                queue.push_back((next_pos, distance + 1));
            }
        }
    }

    offsets
}

fn parse_input(input: &str) -> HashMap<Vec2, Tile> {
    input
        .lines()
        .map(|line| line.trim())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (Vec2(y as i32, x as i32), Tile::try_from(c).unwrap()))
                .collect::<Vec<(Vec2, Tile)>>()
        })
        .collect()
}

#[derive(Eq, PartialEq)]
enum Tile {
    End,
    Open,
    Start,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Open),
            'E' => Ok(Self::End),
            'S' => Ok(Self::Start),
            _ => Err("unexpected character"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
