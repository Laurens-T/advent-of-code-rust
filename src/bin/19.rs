use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, line_ending, multispace0};
use nom::multi::separated_list0;
use nom::sequence::preceded;
use nom::IResult;
advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (available, designs)) = parse_input(input).unwrap();

    let result = designs
        .into_iter()
        .filter(|&line| !line.is_empty())
        .filter(|&x| num_possibilities(x, &available) > 0)
        .count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (available, designs)) = parse_input(input).unwrap();

    let result: usize = designs
        .into_iter()
        .filter(|&line| !line.is_empty())
        .map(|x| num_possibilities(x, &available))
        .sum();

    Some(result as u64)
}

fn num_possibilities(design: &str, available: &Vec<&str>) -> usize {
    fn inner(design: &str, available: &Vec<&str>, cache: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(&result) = cache.get(design) {
            return result;
        }

        let mut count = 0;
        for &a in available.iter() {
            if let Some(stripped) = design.strip_prefix(a) {
                count += inner(stripped, available, cache);
            }
        }

        cache.insert(design.to_string(), count);
        count
    }

    let mut cache: HashMap<String, usize> = HashMap::new();
    inner(design, available, &mut cache)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    fn parse_available(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list0(tag(", "), alpha0)(input)
    }

    let (remaining, available) = parse_available(input)?;
    let (_, patterns) = preceded(multispace0, separated_list0(line_ending, alpha0))(remaining)?;

    Ok((input, (available, patterns)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_num_possibilities() {
        let available = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

        assert_eq!(num_possibilities("bggr", &available), 1);
        assert_eq!(num_possibilities("brwrr", &available), 2);
        assert_eq!(num_possibilities("gbbr", &available), 4);
        assert_eq!(num_possibilities("rrbgbr", &available), 6);

        assert_eq!(num_possibilities("bbrgwb", &available), 0);
        assert_eq!(num_possibilities("ubwu", &available), 0);
    }

    #[test]
    fn test_num_possibilities2() {
        let available = vec!["br", "g", "b", "r"];
        assert_eq!(num_possibilities("brgggg", &available), 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_parse_input() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let (_, (available, designs)) = parse_input(input).expect("can parse");

        assert_eq!(
            available,
            vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
        );
        assert_eq!(
            designs,
            vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",]
        );
    }
}
