use std::collections::HashMap;
advent_of_code::solution!(4);

#[derive(Eq, PartialEq, Hash)]
struct Offset(i32, i32);

pub fn part_one(input: &str) -> Option<i32> {
    let directions: [[Offset; 3]; 8] = [
        [Offset(-1, -1), Offset(-2, -2), Offset(-3, -3)],
        [Offset(-1, 0), Offset(-2, 0), Offset(-3, 0)],
        [Offset(-1, 1), Offset(-2, 2), Offset(-3, 3)],
        [Offset(0, -1), Offset(0, -2), Offset(0, -3)],
        [Offset(0, 1), Offset(0, 2), Offset(0, 3)],
        [Offset(1, -1), Offset(2, -2), Offset(3, -3)],
        [Offset(1, 0), Offset(2, 0), Offset(3, 0)],
        [Offset(1, 1), Offset(2, 2), Offset(3, 3)],
    ];

    let positions = build_map(input);

    let mas = ['M', 'A', 'S'];
    let num_matches: usize = positions
        .iter()
        .filter(|(_, value)| **value == 'X')
        .map(|(pos, _)| num_matches_from_position(&pos, &directions, &positions, &mas))
        .sum();

    Some(num_matches as i32)
}

fn num_matches_from_position<const N: usize, const M: usize>(
    pos: &&Offset,
    directions: &[[Offset; M]; N],
    positions: &HashMap<Offset, char>,
    mas: &[char],
) -> usize {
    directions
        .iter()
        .filter(|&offsets| {
            offsets
                .iter()
                .map(|offset| positions.get(&Offset(pos.0 + offset.0, pos.1 + offset.1)))
                .enumerate()
                .all(|(index, value)| value == mas.get(index))
        })
        .count()
}

fn build_map(input: &str) -> HashMap<Offset, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (Offset(x as i32, y as i32), value))
        })
        .collect::<HashMap<Offset, char>>()
}

pub fn part_two(input: &str) -> Option<i32> {
    let directions: [[Offset; 2]; 4] = [
        [Offset(-1, -1), Offset(1, 1)],
        [Offset(-1, 1), Offset(1, -1)],
        [Offset(1, -1), Offset(-1, 1)],
        [Offset(1, 1), Offset(-1, -1)],
    ];

    let positions = build_map(input);
    let ms = ['M', 'S'];

    let num_matches: usize = positions
        .iter()
        .filter(|(_, value)| **value == 'A')
        .map(|(pos, _)| {
            // starting at X, count all directions that end up being equal to 'MAS'
            num_matches_from_position(&pos, &directions, &positions, &ms) == 2
        })
        .filter(|&b| b)
        .count();

    Some(num_matches as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
