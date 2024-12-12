use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::IResult;
use std::collections::{HashMap};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 75)
}

fn solve(input: &str, num_iterations: usize) -> Option<u64> {
    let (_, stones) = parse_input(input).unwrap();
    let mut stones: HashMap<u64, u64> = stones.into_iter().map(|num| (num, 1)).collect();

    for _ in 0..num_iterations {
        let updated: Vec<(u64, u64)> = stones.iter().flat_map(|(&num, &count)| {
            match num {
                0 => vec![(1, count)],
                num if num_digits(num) % 2 == 0 => {
                    let (a, b) = split_number(num);
                    vec![(a, count), (b, count)]
                }
                num => vec![(num * 2024, count)],
            }
        }).collect();
        
        stones = updated.iter().fold(HashMap::new(), |mut acc, &(num, count)| {
            *acc.entry(num).or_insert(0) += count;
            acc
        });
    }

    Some(stones.values().sum::<u64>())
}

fn num_digits(mut num: u64) -> u64 {
    let mut result = 0;
    while num > 0 {
        result += 1;
        num /= 10;
    }

    result
}

fn split_number(num: u64) -> (u64, u64) {
    let n = num_digits(num) as u32 / 2;
    (num / 10u64.pow(n), num % 10u64.pow(n))
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(" "), complete::u64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn solve_0() {
        let result = solve("0", 25);
        assert_eq!(result, Some(19778));
    }

    #[test]
    fn test_input() {
        let (_, result) = parse_input(&advent_of_code::template::read_file("examples", DAY))
            .expect("expected no error");

        assert_eq!(result, vec![125, 17]);
    }

    #[test]
    fn test_split() {
        let (a, b) = split_number(1234);

        assert_eq!(a, 12);
        assert_eq!(b, 34);
    }

    #[test]
    fn test_split_17() {
        let (a, b) = split_number(17);

        assert_eq!(a, 1);
        assert_eq!(b, 7);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
