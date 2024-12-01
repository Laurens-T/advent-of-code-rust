use std::collections::HashMap;
use std::iter::zip;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let (mut col1, mut col2) = parse_lines(input);

    col1.sort();
    col2.sort();

    let mut result = 0;
    for (a, b) in zip(col1, col2) {
        result += (a - b).abs();
    }

    Some(result)
}

fn parse_lines(input: &str) -> (Vec<i32>, Vec<i32>) {
    let ids: Vec<Vec<&str>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split("   ").collect())
        .collect();

    let col1: Vec<i32> = ids.iter().map(|s| s[0].parse::<i32>().unwrap()).collect();
    let col2: Vec<i32> = ids.iter().map(|s| s[1].parse::<i32>().unwrap()).collect();

    (col1, col2)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (col1, col2) = parse_lines(input);

    let mut counts = HashMap::new();
    col2.into_iter().for_each(|v| {
        counts.entry(v).and_modify(|c| *c += 1).or_insert(1);
    });

    Some(col1.into_iter().fold(0, |acc, v| {
        acc + v * match counts.get(&v) {
            None => 0,
            Some(&count) => count,
        }
    }))
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
