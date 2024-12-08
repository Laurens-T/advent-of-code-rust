use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, &vec![sub, div])
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, &vec![sub, div, concat])
}

type Operation = fn(u64, u64) -> (u64, bool);
fn sub(a: u64, b: u64) -> (u64, bool) {
    (a.saturating_sub(b), true)
}
fn div(a: u64, b: u64) -> (u64, bool) {
    (a / b, a % b == 0)
}

fn concat(a: u64, b: u64) -> (u64, bool) {
    let mut num_digits = 1;
    let mut c = b;
    while c > 0 {
        c /= 10;
        num_digits *= 10;
    }

    if a % num_digits == b {
        ((a - b) / num_digits, true)
    } else {
        (0, false)
    }
}

fn solve(input: &str, funcs: &Vec<Operation>) -> Option<u64> {
    let equations = match parse_input(input) {
        Ok((_, result)) => result,
        Err(e) => panic!("{}", e.to_string()),
    };

    let result: u64 = equations
        .into_iter()
        .filter(|(target, nums)| can_reach(*target, nums, funcs))
        .map(|(target, _)| target)
        .sum();

    Some(result)
}

fn can_reach(target: u64, nums: &[u64], funcs: &Vec<Operation>) -> bool {
    if target < *nums.first().unwrap() {
        return false;
    }

    match nums.len() {
        1 => target == *nums.first().unwrap(),
        _ => match nums.last() {
            None => false,
            Some(&num) => funcs.iter().any(|f| {
                let (new_target, ok) = f(target, num);
                ok && can_reach(new_target, &nums[..(nums.len()) - 1], funcs)
            }),
        },
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(tag("\n"), parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(
        complete::u64,
        tag(": "),
        separated_list1(tag(" "), complete::u64),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_concat() {
        let a = 156;
        let b = 6;

        let (remainder, ok) = concat(a, b);

        assert_eq!(remainder, 15);
        assert!(ok, "expected operation to be ok");
    }

    #[test]
    fn test_can_reach_division_only() {
        let result = can_reach(190, &[10, 19], &vec![div]);
        assert!(result);
    }

    #[test]
    fn test_can_reach_concat() {
        let result = can_reach(156, &[15, 6], &vec![concat]);
        assert!(result, "expected that 15 and 6 can be concat to 156");
    }

    #[test]
    fn test_parse_line() {
        let input = "190: 10 19\n3267: 81 40 27";

        match parse_line(input) {
            Ok((remaining, (target, nums))) => {
                assert_eq!(target, 190);
                assert_eq!(nums, vec![10, 19]);

                assert_eq!(remaining, "\n3267: 81 40 27");
            }
            Err(err) => panic!("didn't expect an error, but got {}", err),
        }
    }

    #[test]
    fn test_parse_input() {
        let input = "190: 10 19\n3267: 81 40 27";

        match parse_input(input) {
            Ok((_, rows)) => {
                let expected = vec![(190, vec![10, 19]), (3267, vec![81, 40, 27])];
                assert_eq!(rows, expected);
            }
            Err(err) => panic!("didn't expect an error, but got {}", err),
        }
    }
}
