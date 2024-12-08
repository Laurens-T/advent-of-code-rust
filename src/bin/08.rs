use std::collections::{HashMap, HashSet};
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut antinodes: HashSet<Pos> = HashSet::new();

    grid.antennas.iter().for_each(|(_, antennas)| {
        for (i, pos1) in antennas.iter().enumerate() {
            for pos2 in antennas.iter().skip(i + 1) {
                let (a1, a2) = grid.single_antinodes(pos1, pos2);

                if grid.in_bounds(&a1) {
                    antinodes.insert(a1);
                }

                if grid.in_bounds(&a2) {
                    antinodes.insert(a2);
                }
            }
        }
    });

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut antinodes: HashSet<Pos> = HashSet::new();

    grid.antennas.iter().for_each(|(_, antennas)| {
        for (i, pos1) in antennas.iter().enumerate() {
            for pos2 in antennas.iter().skip(i + 1) {
                let nodes = grid.resonant_antinodes(pos1, pos2);
                nodes.into_iter().for_each(|pos| {
                    antinodes.insert(pos);
                });
                
                antinodes.insert(*pos1);
                antinodes.insert(*pos2);
            }
        }
    });

    // debug
    // let mut a = vec![vec!['.'; grid.n_cols as usize]; grid.n_rows as usize];
    // 
    // grid.antennas.iter().for_each(|(c, positions)| {
    //     for x in positions {
    //         a[x.row as usize][x.col as usize] = *c;
    //     }
    // });
    // 
    // antinodes.iter().for_each(|x| {
    //     a[x.row as usize][x.col as usize] = '#';
    // });
    // 
    // let something: Vec<String> = a
    //     .iter()
    //     .map(|line| line.iter().collect::<String>())
    //     .collect();
    // 
    // something.iter().for_each(|line| println!("{line}"));

    Some(antinodes.len() as u32)
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn new(row: i32, col: i32) -> Self {
        Pos { row, col }
    }
}

struct Grid {
    antennas: HashMap<char, Vec<Pos>>,
    n_cols: i32,
    n_rows: i32,
}

impl Grid {
    fn in_bounds(&self, pos: &Pos) -> bool {
        pos.row >= 0 && pos.col >= 0 && pos.row < self.n_rows && pos.col < self.n_cols
    }

    fn single_antinodes(&self, pos1: &Pos, pos2: &Pos) -> (Pos, Pos) {
        let (pos1, pos2) = if pos1.col < pos2.col {
            (pos1, pos2)
        } else {
            (pos2, pos1)
        };

        let delta_y = (pos1.row - pos2.row).abs();
        let delta_x = (pos1.col - pos2.col).abs();

        if pos1.row <= pos2.row {
            // diagonal (2, 5) and (3, 7)
            (
                Pos::new(pos1.row - delta_y, pos1.col - delta_x),
                Pos::new(pos2.row + delta_y, pos2.col + delta_x),
            )
        } else {
            // anti-diagonal (2,5) and (1, 8)
            (
                Pos::new(pos1.row + delta_y, pos1.col - delta_x),
                Pos::new(pos2.row - delta_y, pos2.col + delta_x),
            )
        }
    }

    fn resonant_antinodes(&self, pos1: &Pos, pos2: &Pos) -> Vec<Pos> {
        let (pos1, pos2) = if pos1.col < pos2.col {
            (pos1, pos2)
        } else {
            (pos2, pos1)
        };

        let delta_y = (pos1.row - pos2.row).abs();
        let delta_x = (pos1.col - pos2.col).abs();

        let mut antinodes = Vec::new();

        if pos1.row <= pos2.row {
            // exercise for the reader: refactor this so that it uses fewer lines of code 
            // go to top left
            let mut pos = Pos::new(pos1.row - delta_y, pos1.col - delta_x);
            while self.in_bounds(&pos) {
                antinodes.push(pos);
                pos = Pos::new(pos.row - delta_y, pos.col - delta_x);
            }

            // go to bottom left
            let mut pos = Pos::new(pos2.row + delta_y, pos2.col + delta_x);
            while self.in_bounds(&pos) {
                antinodes.push(pos);
                pos = Pos::new(pos.row + delta_y, pos.col + delta_x);
            }
        } else {
            // anti-diagonal (2,5) and (1, 8)
            let mut pos = Pos::new(pos1.row + delta_y, pos1.col - delta_x);
            while self.in_bounds(&pos) {
                antinodes.push(pos);
                pos = Pos::new(pos.row + delta_y, pos.col - delta_x);
            }

            let mut pos = Pos::new(pos2.row - delta_y, pos2.col + delta_x);
            while self.in_bounds(&pos) {
                antinodes.push(pos);
                pos = Pos::new(pos.row - delta_y, pos.col + delta_x);
            }
        }

        antinodes
    }
}

fn parse_input(input: &str) -> Grid {
    let mut antennas = HashMap::new();
    let mut n_rows = 0;
    let mut n_cols = 0;
    for (row, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        n_rows += 1;
        n_cols = line.len() as i32;

        line.chars().enumerate().for_each(|(col, c)| {
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push(Pos {
                    row: row as i32,
                    col: col as i32,
                });
            }
        });
    }

    Grid {
        antennas,
        n_cols,
        n_rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_antinodes_1() {
        // ............
        // ........0... <-- pos2
        // .....0...... <-- pos1
        // ............
        let pos1 = &Pos::new(2, 5);
        let pos2 = &Pos::new(1, 8);

        let result = Grid {
            antennas: HashMap::new(),
            n_cols: 12,
            n_rows: 12,
        }
        .single_antinodes(pos1, pos2);
        let expected = (Pos::new(3, 2), Pos::new(0, 11));

        assert_eq!(expected, result);
    }

    #[test]
    fn test_antinodes_2() {
        // ............
        // ........0... <-- pos1
        // .....0...... <-- pos2
        // ............
        let pos1 = &Pos::new(1, 8);
        let pos2 = &Pos::new(2, 5);

        let result = Grid {
            antennas: HashMap::new(),
            n_cols: 12,
            n_rows: 12,
        }
        .single_antinodes(pos1, pos2);
        let expected = (Pos::new(3, 2), Pos::new(0, 11));

        assert_eq!(expected, result);
    }

    #[test]
    fn test_antinodes_3() {
        // ............
        // ............
        // .....0...... <-- pos1
        // .......0.... <-- pos2
        let pos1 = &Pos::new(2, 5);
        let pos2 = &Pos::new(3, 7);

        let result = Grid {
            antennas: HashMap::new(),
            n_cols: 12,
            n_rows: 12,
        }
        .single_antinodes(pos1, pos2);
        let expected = (Pos::new(1, 3), Pos::new(4, 9));

        assert_eq!(expected, result);
    }

    #[test]
    fn test_antinodes_4() {
        // ............
        // ............
        // .....0...... <-- pos2
        // .......0.... <-- pos1
        let pos1 = &Pos::new(3, 7);
        let pos2 = &Pos::new(2, 5);

        let result = Grid {
            antennas: HashMap::new(),
            n_cols: 12,
            n_rows: 12,
        }
        .single_antinodes(pos1, pos2);
        let expected = (Pos::new(1, 3), Pos::new(4, 9));

        assert_eq!(expected, result);
    }

    #[test]
    fn test_antinodes_vertical() {
        let pos1 = &Pos::new(3, 5);
        let pos2 = &Pos::new(5, 5);

        let result = Grid {
            antennas: HashMap::new(),
            n_cols: 12,
            n_rows: 12,
        }
        .single_antinodes(pos1, pos2);
        let expected = (Pos::new(7, 5), Pos::new(1, 5));

        assert_eq!(expected, result);
    }

    #[test]
    fn test_antinodes_horizontal() {
        let pos1 = &Pos::new(3, 5);
        let pos2 = &Pos::new(3, 7);

        let result = Grid {
            antennas: HashMap::new(),
            n_cols: 12,
            n_rows: 12,
        }
        .single_antinodes(pos1, pos2);
        let expected = (Pos::new(3, 3), Pos::new(3, 9));

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result.n_cols, 12);
        assert_eq!(result.n_rows, 12);
        assert_eq!(
            HashMap::from([
                (
                    '0',
                    vec![
                        Pos::new(1, 8),
                        Pos::new(2, 5),
                        Pos::new(3, 7),
                        Pos::new(4, 4)
                    ]
                ),
                ('A', vec![Pos::new(5, 6), Pos::new(8, 8), Pos::new(9, 9)]),
            ]),
            result.antennas,
        );
    }
}
