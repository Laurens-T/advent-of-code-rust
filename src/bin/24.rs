use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alphanumeric0, line_ending, multispace0};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{IResult, InputTakeAtPosition};

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (given, instructions)) = parse_input(input).unwrap();

    let mut context = HashMap::new();
    given.iter().for_each(|&(key, value)| {
        context.insert(key.to_string(), value);
    });

    let mut instructions: Vec<&Instruction> = instructions.iter().collect();
    while !instructions.is_empty() {
        let mut new_instructions = vec![];

        for instr in instructions {
            if instr.apply(&context).is_none() {
                new_instructions.push(instr);
            } else if let Some(result) = instr.apply(&context) {
                context.insert(instr.out.to_string(), result);
            };
        }

        instructions = new_instructions;
    }

    dbg!(&context);

    let mut out: u64 = 0;
    for i in 0..64u64 {
        let key = format!("z{i:0>2}");

        match context.get(&key) {
            None => continue,
            Some(&val) => out += (val as u64) << i,
        }
    }

    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    // find all values
    // sum xs and ys to zs
    // find all conflicting values
    //
    None
}

#[derive(Debug)]
struct Instruction {
    left: String,
    op: Operation,
    right: String,
    out: String,
}

impl Instruction {
    fn apply(&self, context: &HashMap<String, u8>) -> Option<u8> {
        match (context.get(&self.left), context.get(&self.right)) {
            (Some(&l), Some(&r)) => Some(self.op.apply(l, r)),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!("err"),
        }
    }
}

impl Operation {
    fn apply(&self, left: u8, right: u8) -> u8 {
        match self {
            Operation::And => left & right,
            Operation::Or => left | right,
            Operation::Xor => left ^ right,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<(&str, u8)>, Vec<Instruction>)> {
    let (input, (g, instructions)) =
        separated_pair(parse_given, multispace0, parse_formulas)(input)?;

    let instructions = instructions
        .into_iter()
        .map(|i| Instruction {
            left: i[0].to_string(),
            op: i[1].into(),
            right: i[2].to_string(),
            out: i[4].to_string(),
        })
        .collect();

    Ok((input, (g, instructions)))
}

fn parse_given(input: &str) -> IResult<&str, Vec<(&str, u8)>> {
    separated_list0(
        line_ending,
        separated_pair(alphanumeric0, tag(": "), complete::u8),
    )(input)
}

fn parse_formulas(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list0(line_ending, parse_formula)(input)
}

fn parse_formula(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(" "), non_space)(input)
}

fn non_space(input: &str) -> IResult<&str, &str> {
    input.split_at_position_complete(char::is_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
