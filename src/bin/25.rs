advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let (keys, locks) = parse_input(input);
    let mut result = 0;

    for lock in locks {
        for key in keys.iter() {
            if lock.iter().zip(key).all(|(k, l)| k + l <= 5) {
                result += 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let split = input.split("\n\n");

    for s in split {
        if s.is_empty() {
            continue;
        }

        let mut counts = vec![0; 5];
        s.lines().for_each(|line| {
            line.chars().enumerate().for_each(|(col, c)| {
                if c == '#' {
                    counts[col] += 1;   
                }
            })
        });
        counts.iter_mut().for_each(|c| *c -= 1);

        match s.as_bytes().first() {
            Some(b'#') => locks.push(counts),
            Some(b'.') => keys.push(counts),
            _ => panic!("aoc")
        };
    }

    (keys, locks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_input() {
        let (keys, locks) = parse_input(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(
            vec![
                vec![5, 0, 2, 1, 3],
                vec![4, 3, 4, 0, 2],
                vec![3, 0, 2, 0, 1],
            ],
            keys,
        );

        assert_eq!(vec![vec![0, 5, 3, 4, 3], vec![1, 2, 0, 5, 3],], locks,);
    }
}
