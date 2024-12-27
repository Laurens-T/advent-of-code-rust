use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (mut register, instructions) = parse(input);

    let outputs: Vec<String> = solve(&mut register, &instructions)
        .iter()
        .map(|i| i.to_string())
        .collect();

    Some(outputs.join(","))
}

fn solve(register: &mut Registers, instructions: &[Instruction]) -> Vec<i64> {
    let mut outputs = vec![];

    while register.pointer < instructions.len() {
        if let Some(value) = register.op(
            &instructions[register.pointer],
            &instructions[register.pointer + 1],
        ) {
            outputs.push(value);
        }
    }
    outputs
}

pub fn part_two(input: &str) -> Option<u32> {
    let (register, instructions) = parse(input);
    let expected: Vec<i64> = instructions.iter().map(|i| i.literal()).collect();

    let mut i = 0;
    let result = loop {
        let mut register_clone = register;
        register_clone.a = i;

        let got = solve(&mut register_clone, &instructions);

        if expected == got {
            break i;
        }

        if i == i64::MAX {
            break 0;
        }

        if i % 1_000_000 == 0 {
            println!("{i}")
        }

        i += 1;
    };

    Some(result as u32)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
    pointer: usize,
}

impl Registers {
    fn op(&mut self, instruction: &Instruction, operand: &Instruction) -> Option<i64> {
        match instruction {
            Instruction::Adv => {
                self.a /= 2i64.pow(self.combo(operand) as u32);
            }
            Instruction::Bxl => {
                self.b ^= operand.literal();
            }
            Instruction::Bst => {
                self.b = self.combo(operand) % 8;
            }
            Instruction::Jnz => {
                if self.a == 0 {
                    self.jump();
                } else {
                    self.pointer = operand.literal() as usize;
                }

                return None;
            }
            Instruction::Bxc => {
                self.b ^= self.c;
            }
            Instruction::Out => {
                self.jump();
                return Some(self.combo(operand) % 8);
            }
            Instruction::Bdv => {
                self.b = self.a / 2i64.pow(self.combo(operand) as u32);
            }
            Instruction::Cdv => {
                self.c = self.a / 2i64.pow(self.combo(operand) as u32);
            }
        }
        self.jump();
        None
    }

    fn combo(&self, operand: &Instruction) -> i64 {
        match *operand as i64 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => {
                unreachable!("should not happen")
            }
        }
    }

    fn jump(&mut self) {
        self.pointer += 2;
    }
}

// The computer knows eight instructions, each identified by a 3-bit number
// (called the instruction's opcode). Each instruction also reads the 3-bit number after it as
// an input; this is called its operand.
#[repr(i64)]
#[derive(Debug, Copy, Clone)]
enum Instruction {
    // The adv instruction (opcode 0) performs division. The numerator is the value in the A
    // register. The denominator is found by raising 2 to the power of the instruction's combo
    // operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    // The result of the division operation is truncated to an integer and then written to the A register.
    Adv = 0,
    // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
    // instruction's literal operand, then stores the result in register B.
    Bxl = 1,
    // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
    // (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    Bst = 2,
    // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A
    // register is not zero, it jumps by setting the instruction pointer to the value of its literal
    // operand; if this instruction jumps, the instruction pointer is not increased by 2 after
    // this instruction.
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl Instruction {
    fn try_from(opcode: u8) -> Self {
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("invalid opcode"),
        }
    }

    fn literal(&self) -> i64 {
        *self as i64
    }
}

fn parse(input: &str) -> (Registers, Vec<Instruction>) {
    let (input, registers) = parse_registers(input).unwrap();

    let (_, instructions): (&str, Vec<u8>) = preceded(
        multispace0::<&str, nom::error::Error<&str>>,
        preceded(tag("Program: "), separated_list0(tag(","), complete::u8)),
    )(input)
    .unwrap();

    let instructions = instructions
        .iter()
        .map(|&i| Instruction::try_from(i))
        .collect();

    (registers, instructions)
}

fn parse_registers(input: &str) -> IResult<&str, Registers> {
    let (input, a) = delimited(tag("Register A: "), complete::i64, line_ending)(input)?;
    let (input, b) = delimited(tag("Register B: "), complete::i64, line_ending)(input)?;
    let (input, c) = delimited(tag("Register C: "), complete::i64, line_ending)(input)?;

    Ok((
        input,
        Registers {
            a,
            b,
            c,
            pointer: 0,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_one_2() {
        let mut register = Registers {
            a: 117440,
            b: 0,
            c: 0,
            pointer: 0,
        };

        let instruction = vec![
            Instruction::try_from(0),
            Instruction::try_from(3),
            Instruction::try_from(5),
            Instruction::try_from(4),
            Instruction::try_from(3),
            Instruction::try_from(0),
        ];

        let result = solve(&mut register, &instruction);
        assert_eq!(result, vec![0, 3, 5, 4, 3, 0]);
    }

    #[test]
    fn test_part_two() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        let result = part_two(input);
        assert_eq!(result, Some(117440));
    }

    #[test]
    fn test_parse() {
        let (registers, _) = parse(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(
            registers,
            Registers {
                a: 729,
                b: 0,
                c: 0,
                pointer: 0,
            }
        )
    }
}
