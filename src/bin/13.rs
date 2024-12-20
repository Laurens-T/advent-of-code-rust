use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use std::{
    collections::{BinaryHeap, HashMap},
    ops
};

advent_of_code::solution!(13);

const COST_A: i64 = 3;
const COST_B: i64 = 1;

pub fn part_one(input: &str) -> Option<i64> {
    let (_, machines) = parse(input).expect("could not parse input");

    let result: i64 = machines.iter().filter_map(smallest_cost).sum();

    Some(result)
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    cost: i64,
    pos: Vec2,
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, machines) = parse(input).expect("could not parse input");
    let offset: i64 = 10000000000000;

    let result: i64 = machines
        .into_iter()
        .map(|machine| solve(&machine, offset))
        .sum();

    Some(result)
}

fn solve(machine: &Machine, offset: i64) -> i64 {
    let x1 = machine.a.0;
    let x2 = machine.a.1;
    let y1 = machine.b.0;
    let y2 = machine.b.1;
    let c1 = machine.prize.0 + offset;
    let c2 = machine.prize.1 + offset;

    let b = (c2 * x1 - c1 * x2) / (y2 * x1 - y1 * x2);
    let a = (c1 - b * y1) / x1;

    if (x1 * a + y1 * b, x2 * a + y2 * b) != (c1, c2) {
        0
    } else {
        a * 3 + b
    }
}

fn solve_machine(machine: &Machine, offset: i64) -> i64 {
    let prize = (machine.prize.0 + offset, machine.prize.1 + offset);
    let det = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;

    let a = (prize.0 * machine.b.1 - prize.1 * machine.b.0) / det;
    let b = (machine.a.0 * prize.1 - machine.a.1 * prize.0) / det;

    if (
        machine.a.0 * a + machine.b.0 * b,
        machine.a.1 * a + machine.b.1 * b,
    ) == (prize.0, prize.1)
    {
        a * COST_A + COST_B
    } else {
        0
    }
}

fn smallest_cost(machine: &Machine) -> Option<i64> {
    let mut cost: HashMap<Vec2, i64> = HashMap::new();
    let mut pq: BinaryHeap<Item> = BinaryHeap::new();
    let mut min_cost = i64::MAX;

    pq.push(Item {
        cost: 0,
        pos: Vec2(0, 0),
    });

    let next_states = [(COST_A, machine.a), (COST_B, machine.b)];

    while let Some(item) = pq.pop() {
        if item.cost >= min_cost {
            break;
        }

        if item.pos == machine.prize {
            min_cost = min_cost.min(item.cost);
            continue;
        }

        if item.pos.0 > machine.prize.0 || item.pos.1 > machine.prize.1 {
            continue;
        }

        for (c, offset) in next_states {
            let new_item = Item {
                cost: item.cost + c,
                pos: item.pos + offset,
            };

            if cost.get(&new_item.pos).is_none_or(|&c| new_item.cost < c) {
                cost.insert(new_item.pos, new_item.cost);
                pq.push(new_item);
            }
        }
    }

    if min_cost < i64::MAX {
        Some(min_cost)
    } else {
        None
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Vec2(i64, i64);

impl ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct Machine {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(tuple((line_ending, line_ending)), machine)(input)
}

fn button(input: &str) -> IResult<&str, Vec2> {
    preceded(
        alt((tag("Button A: X+"), tag("Button B: X+"))),
        separated_pair(complete::i64, tag(", Y+"), complete::i64).map(|(x, y)| Vec2(x, y)),
    )(input)
}

fn prize(input: &str) -> IResult<&str, Vec2> {
    preceded(
        tag("Prize: X="),
        separated_pair(complete::i64, tag(", Y="), complete::i64).map(|(x, y)| Vec2(x, y)),
    )(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a, b, prize)) = tuple((
        terminated(button, line_ending),
        terminated(button, line_ending),
        prize,
    ))(input)?;

    Ok((input, Machine { a, b, prize }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
