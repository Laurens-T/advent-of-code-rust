use std::iter::zip;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    Some(grid.into_iter().filter(|report| is_safe(report)).count() as u32)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let (mut incr, mut decr) = (0, 0);

    for (a, b) in zip(report, report.iter().skip(1)) {
        if a > b {
            decr += 1;
        } else {
            incr += 1;
        }
    }

    if incr != 0 && decr != 0 {
        return false;
    }

    for (a, b) in zip(report, report.iter().skip(1)) {
        if !(1..=3).contains(&(a - b).abs()) {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    Some(
        grid.into_iter()
            .filter(|report| {
                for i in 0..report.len() {
                    // bruteforce
                    // rust is blazingly fast 👌
                    let mut cp = report.clone();
                    cp.remove(i);

                    if is_safe(&cp) {
                        return true;
                    }
                }

                is_safe(report)
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
