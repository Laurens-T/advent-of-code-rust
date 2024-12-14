use std::collections::HashMap;
use std::iter::zip;

use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let (mut col1, mut col2) = parse_input(input);

    col1.sort();
    col2.sort();

    Some(zip(col1, col2).map(|(l, r)| (l - r).abs()).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let (col1, col2) = parse_input(input);

    let mut counts = HashMap::new();
    col2.into_iter().for_each(|v| {
        counts.entry(v).and_modify(|c| *c += 1).or_insert(1);
    });

    Some(
        col1.into_iter()
            .map(|v| {
                v * match counts.get(&v) {
                    None => 0,
                    Some(&count) => count,
                }
            })
            .sum(),
    )
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (_, values) = separated_list1(
        line_ending::<&str, ()>,
        separated_pair(complete::i32, space1, complete::i32),
    )(input)
    .expect("");

    let (mut first, mut second): (Vec<i32>, Vec<i32>) = (
        Vec::with_capacity(values.len()),
        Vec::with_capacity(values.len()),
    );

    for (l, r) in values {
        first.push(l);
        second.push(r);
    }

    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
