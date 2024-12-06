use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i32> {
    let (rules, pages) = parse_input(input);
    let graph = build_graph(&rules);

    let result = pages
        .iter()
        .filter(|&row| in_order(row, &graph))
        .map(|row| row.get(row.len() / 2).unwrap()) // sum middle number
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (rules, mut pages) = parse_input(input);
    let graph = build_graph(&rules);

    let result = pages
        .iter_mut()
        .filter(|row| !in_order(row, &graph))
        .map(|row| {
            row.sort_by(|&a, &b| {
                if precedes(&graph, a, b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            row.get(row.len() / 2).unwrap()
        })
        .sum();

    Some(result)
}

fn in_order(sequence: &[i32], graph: &HashMap<i32, HashSet<i32>>) -> bool {
    sequence
        .windows(2)
        .all(|window| precedes(graph, window[0], window[1]))
}

fn precedes(graph: &HashMap<i32, HashSet<i32>>, a: i32, b: i32) -> bool {
    if let Some(s) = graph.get(&a) {
        s.contains(&b)
    } else {
        false
    }
}

// Build a mapping from integers to a set of larger numbers.
fn build_graph(rules: &[(i32, i32)]) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    rules.iter().for_each(|rule| {
        graph.entry(rule.0).or_default().insert(rule.1);
        graph.entry(rule.1).or_default();
    });

    graph
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let rules: Vec<(i32, i32)> = input
        .lines()
        .map(|line| line.trim())
        .take_while(|&line| !line.is_empty())
        .map(|line| {
            let split: Vec<&str> = line.split("|").collect();
            (
                split.first().unwrap().parse().unwrap(),
                split.get(1).unwrap().parse().unwrap(),
            )
        })
        .collect();

    let page_numbers: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.trim())
        .skip_while(|line| !line.contains(","))
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let split: Vec<&str> = line.split(",").collect();
            split
                .iter()
                .map(|&val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, page_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_parse_input() {
        let input = "1|2
        2|3
        3|4

        1,2,3
        2,3,4";

        let (rules, pages) = parse_input(input);

        assert_eq!(rules, vec![(1, 2), (2, 3), (3, 4)]);

        let expected = vec![vec![1, 2, 3], vec![2, 3, 4]];
        assert_eq!(pages, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
