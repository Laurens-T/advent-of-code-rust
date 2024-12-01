use std::collections::HashMap;
use std::iter::zip;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second) = parse_lines(input);

    first.sort();
    second.sort();

    let mut result = 0;
    for (a, b) in zip(first, second) {
        result += (a - b).abs();
    }

    Some(result as u32)
}

fn parse_lines(input: &str) -> (Vec<i32>, Vec<i32>) {
    let ids: Vec<Vec<&str>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split("   ").collect())
        .collect();

    let first: Vec<i32> = ids.iter().map(|s| s[0].parse::<i32>().unwrap()).collect();
    let second: Vec<i32> = ids.iter().map(|s| s[1].parse::<i32>().unwrap()).collect();

    (first, second)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second) = parse_lines(input);

    let mut counts = HashMap::new();
    second.into_iter().for_each(|v| {
        counts.entry(v).and_modify(|c| *c += 1).or_insert(1);
    });

    let mut result = 0;
    for val in first {
        result += val * match counts.get(&val) {
            None => 0,
            Some(&count) => count,
        };
    }

    Some(result as u32)
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
