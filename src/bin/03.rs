advent_of_code::solution!(3);

enum Instruction {
    Multiplication { first: i32, second: i32 },
    Enable,
    Disable,
}

struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Parser {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn curr_byte(&self) -> Option<&char> {
        self.input.get(self.pos)
    }
}

impl Parser {
    fn maybe_parse_enable_disable(&mut self) -> Option<Instruction> {
        for b in ['d', 'o'] {
            match self.curr_byte() {
                Some(&c) if c == b => self.pos += 1,
                Some(_) => return None,
                None => return None,
            }
        }

        match self.curr_byte() {
            None => None,
            Some(&b) => match b {
                '(' => self.maybe_parse_enable(),
                'n' => self.maybe_parse_disable(),
                _ => None,
            },
        }
    }

    fn maybe_parse_enable(&mut self) -> Option<Instruction> {
        for b in ['(', ')'] {
            match self.curr_byte() {
                Some(&c) if c == b => self.pos += 1,
                Some(_) => return None,
                None => return None,
            }
        }

        Some(Instruction::Enable)
    }

    fn maybe_parse_disable(&mut self) -> Option<Instruction> {
        for b in ['n', '\'', 't', '(', ')'] {
            match self.curr_byte() {
                Some(&c) if c == b => self.pos += 1,
                Some(_) => return None,
                None => return None,
            }
        }

        Some(Instruction::Disable)
    }
}

impl Parser {
    fn maybe_parse_mul(&mut self) -> Option<Instruction> {
        for b in ['m', 'u', 'l', '('] {
            match self.curr_byte() {
                Some(&c) if c == b => self.pos += 1,
                Some(_) => return None,
                None => return None,
            }
        }

        let first = match self.maybe_parse_num() {
            Some(val) => val,
            None => return None,
        };

        match self.curr_byte() {
            Some(&c) if c == ',' => self.pos += 1,
            Some(_) => return None,
            None => return None,
        }

        let second = match self.maybe_parse_num() {
            Some(val) => val,
            None => return None,
        };

        match self.curr_byte() {
            Some(&c) if c == ')' => self.pos += 1,
            Some(_) => return None,
            None => return None,
        }

        Some(Instruction::Multiplication { first, second })
    }

    fn maybe_parse_num(&mut self) -> Option<i32> {
        let start = self.pos;
        let mut result: i32 = 0;

        while let Some(&b) = self.curr_byte() {
            if b.is_ascii_digit() {
                result = 10 * result + (b as u8 - '0' as u8) as i32;
                self.pos += 1;
            } else {
                break;
            }
        }

        if start < self.pos {
            Some(result)
        } else {
            None
        }
    }
}

impl Iterator for Parser {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&b) = self.input.get(self.pos) {
            match b {
                'm' => {
                    if let Some(instruction) = self.maybe_parse_mul() {
                        return Some(instruction);
                    }
                }
                'd' => {
                    if let Some(instruction) = self.maybe_parse_enable_disable() {
                        return Some(instruction);
                    }
                }
                _ => {}
            }

            self.pos += 1;
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let parser = Parser::new(input);

    let mut result = 0;
    for item in parser {
        if let Instruction::Multiplication { first, second } = item {
            result += first * second
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let parser = Parser::new(input);

    let mut result = 0;
    let mut enabled = true;

    for item in parser {
        match item {
            Instruction::Multiplication { first, second } => {
                if enabled {
                    result += first * second
                }
            }
            Instruction::Enable => enabled = true,
            Instruction::Disable => enabled = false,
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part_one(input);
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(input);
        assert_eq!(result, Some(48));
    }
}
