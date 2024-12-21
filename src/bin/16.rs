use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Add;
advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start, end) = parse_input(input);
    let (_, cost) = shortest_path(&grid, start, end);

    Some(cost)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start, end) = parse_input(input);
    let (shortest_paths, _) = shortest_path(&grid, start, end);

    let mut result = HashSet::new();

    shortest_paths.iter().for_each(|path| {
        for pos in path {
            result.insert(pos);
        }
    });

    Some(result.len() as u32)
}

fn shortest_path(grid: &HashMap<Vec2, Tile>, start: Vec2, end: Vec2) -> (Vec<Vec<Vec2>>, u32) {
    #[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone)]
    struct State {
        cost: u32,
        pos: Vec2,
        direction: Direction,
        prev: Option<Box<State>>,
    }

    let mut pq: BinaryHeap<Reverse<State>> = BinaryHeap::from([Reverse(State {
        cost: 0,
        pos: start,
        direction: Direction::East,
        prev: None,
    })]);

    let mut cost: HashMap<(Vec2, Direction), u32> = HashMap::from([((start, Direction::East), 0)]);
    let mut min_cost = u32::MAX;
    let mut shortest_paths: Vec<State> = Vec::new();

    while let Some(state) = pq.pop() {
        let state = state.0;
        if state.cost > min_cost {
            break;
        }

        if state.pos == end {
            let cost = state.cost;
            match cost.cmp(&min_cost) {
                Ordering::Equal => shortest_paths.push(state),
                Ordering::Less => {
                    shortest_paths.clear();
                    shortest_paths.push(state);
                }
                Ordering::Greater => {}
            }

            min_cost = min_cost.min(cost);
            continue;
        }

        let new_states = [
            State {
                cost: state.cost + 1,
                pos: state.pos + state.direction.as_vec2(),
                direction: state.direction,
                prev: Some(Box::new(state.clone())),
            },
            State {
                cost: state.cost + 1000,
                pos: state.pos,
                direction: state.direction.clockwise(),
                prev: Some(Box::new(state.clone())),
            },
            State {
                cost: state.cost + 1000,
                pos: state.pos,
                direction: state.direction.counter_clockwise(),
                prev: Some(Box::new(state.clone())),
            },
        ];

        for new_state in new_states {
            if let Some(tile) = grid.get(&new_state.pos) {
                match tile {
                    Tile::Wall => {}
                    Tile::Open | Tile::End => {
                        let key = (new_state.pos, new_state.direction);
                        // <= is necessary for part 2, but this is not efficient since lots of
                        // duplicate stuff gets calculated.
                        if cost.get(&key).is_none_or(|&c| new_state.cost <= c) {
                            cost.insert(key, new_state.cost);
                            pq.push(Reverse(new_state));
                        }
                    }
                }
            }
        }
    }

    let mut paths = Vec::new();
    for state in shortest_paths {
        let mut path = Vec::new();
        let mut current_state = state;
        while let Some(prev_state) = current_state.prev {
            path.push(current_state.pos);
            current_state = *prev_state;
        }
        path.push(start);
        path.reverse();
        paths.push(path);
    }

    (paths, min_cost)
}


enum Tile {
    Wall,
    Open,
    End,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn clockwise(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }

    fn counter_clockwise(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
        }
    }

    fn as_vec2(&self) -> Vec2 {
        match self {
            Direction::North => Vec2(-1, 0),
            Direction::East => Vec2(0, 1),
            Direction::South => Vec2(1, 0),
            Direction::West => Vec2(0, -1),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd, Debug)]
struct Vec2(i32, i32);

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn parse_input(input: &str) -> (HashMap<Vec2, Tile>, Vec2, Vec2) {
    let mut start: Option<Vec2> = None;
    let mut end: Option<Vec2> = None;

    let grid = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let pos = Vec2(y as i32, x as i32);
                    match c {
                        'S' => {
                            start = Some(Vec2(y as i32, x as i32));
                            (pos, Tile::Open)
                        }
                        'E' => {
                            end = Some(Vec2(y as i32, x as i32));
                            (pos, Tile::End)
                        }
                        '#' => (pos, Tile::Wall),
                        '.' => (pos, Tile::Open),
                        _ => panic!(""),
                    }
                })
                .collect::<Vec<(Vec2, Tile)>>()
        })
        .collect();

    (grid, start.unwrap(), end.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
