use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{self, line_ending, space1};
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};
use nom::{IResult, Parser};
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 101, 103)
}

fn solve(input: &str, width: i32, height: i32) -> Option<u32> {
    let (_, mut robots) = parse_input(input).expect("");

    robots
        .iter_mut()
        .for_each(|robot| robot.step_n(100, width, height));

    let mut counts = [0, 0, 0, 0];
    for robot in robots.iter() {
        match robot.quadrant(width, height) {
            None => continue,
            Some(q) => {
                counts[q as usize] += 1;
            }
        };
    }

    Some(counts.iter().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut robots) = parse_input(input).expect("");

    let mut i = 1;
    loop {
        robots
            .iter_mut()
            .for_each(|robot| robot.step_n(1, 101, 103));
        
        if stopping_condition(&robots, 103, 101) {
            // println!("{}", print_grid(101, 103, &robots));
            break Some(i);
        }

        i += 1;

        if i > 100_000 {
            break None;
        }
    }
}

fn stopping_condition(robots: &[Robot], height: usize, width: usize) -> bool {
    let mut counts_1 = vec![0; height];
    let mut counts_2 = vec![0; width];

    robots.iter().for_each(|robot| {
        counts_1[robot.position.1 as usize] += 1;
        counts_2[robot.position.0 as usize] += 1;
    });

    counts_1.iter().k_largest(10).all(|&c| c >= 10)
        && counts_2.iter().k_largest(10).all(|&c| c >= 10)
}

// fn print_grid(width: i32, height: i32, robots: &[Robot]) -> String {
//     let positions: HashSet<&Vec2> = robots.iter().map(|r| &r.position).collect();
//     let mut output = String::with_capacity((width * height + height + 10).try_into().unwrap());
// 
//     for y in 0..height {
//         for x in 0..width {
//             if positions.contains(&Vec2(x, y)) {
//                 output.push('x');
//             } else {
//                 output.push('.');
//             }
//         }
//         output.push('\n');
//     }
// 
//     output
// }

#[derive(PartialEq, Eq, Debug, Hash)]
struct Vec2(i32, i32);

#[derive(PartialEq, Eq, Debug)]
struct Robot {
    position: Vec2,
    speed: Vec2,
}

impl Robot {
    fn step_n(&mut self, n: i32, width: i32, height: i32) {
        self.position = Vec2(
            (self.position.0 + n * self.speed.0).rem_euclid(width),
            (self.position.1 + n * self.speed.1).rem_euclid(height),
        );
    }

    fn quadrant(&self, width: i32, height: i32) -> Option<i32> {
        let x = width / 2;
        let y = height / 2;

        if self.position.0 == x || self.position.1 == y {
            None
        } else {
            let a = if self.position.1 < y { 0 } else { 2 };
            let b = if self.position.0 < x { 0 } else { 1 };

            Some(a + b)
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    // p=0,4 v=3,-3
    separated_list0(
        line_ending,
        separated_pair(
            preceded(tag("p="), parse_vec2),
            space1,
            preceded(tag("v="), parse_vec2),
        )
        .map(|(position, speed)| Robot { position, speed }),
    )(input)
}

fn parse_vec2(input: &str) -> IResult<&str, Vec2> {
    let (input, (x, y)) = separated_pair(complete::i32, tag(","), complete::i32)(input)?;
    Ok((input, Vec2(x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_step() {
        let mut robot = Robot {
            position: Vec2(2, 4),
            speed: Vec2(2, -3),
        };

        let width = 11;
        let height = 7;

        robot.step_n(5, width, height);

        assert_eq!(robot.position, Vec2(1, 3),);
    }

    #[test]
    fn test_parse_input() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3";

        let (_, robots) = parse_input(input).expect("could parse input");

        assert_eq!(
            robots,
            vec![
                Robot {
                    position: Vec2(0, 4),
                    speed: Vec2(3, -3),
                },
                Robot {
                    position: Vec2(6, 3),
                    speed: Vec2(-1, -3),
                },
            ]
        );
    }
}
