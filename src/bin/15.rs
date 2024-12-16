use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Object {
    Wall,
    Space,
    Box,
    BoxLeftEdge,
    BoxRightEdge,
}

impl Object {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Space,
            'O' => Self::Box,
            '[' => Self::BoxLeftEdge,
            ']' => Self::BoxRightEdge,
            _ => panic!(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Wall => write!(f, "#"),
            Object::Space => write!(f, "."),
            Object::Box => write!(f, "O"),
            Object::BoxLeftEdge => write!(f, "["),
            Object::BoxRightEdge => write!(f, "]"),
        }
    }
}

impl Direction {
    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, movements, mut start) = parse_input(input);
    for c in movements {
        start = step(&mut grid, c, start);
    }

    Some(calculate_result(&grid))
}

fn calculate_result(grid: &HashMap<(i32, i32), Object>) -> u32 {
    grid.iter()
        .filter(|&(_, c)| *c == Object::Box || *c == Object::BoxLeftEdge)
        .map(|((y, x), _)| (y * 100 + x) as u32)
        .sum()
}

fn step(grid: &mut HashMap<(i32, i32), Object>, step: Direction, pos: (i32, i32)) -> (i32, i32) {
    let direction = step.to_tuple();
    let new_pos = (pos.0 + direction.0, pos.1 + direction.1);

    let &c = match grid.get(&new_pos) {
        None => return pos,
        Some(c) => c,
    };

    match c {
        Object::BoxLeftEdge | Object::BoxRightEdge => panic!("[ and ] are not part of part 1"),
        Object::Wall => pos,      // can't move
        Object::Space => new_pos, // can move here
        Object::Box => {
            let mut space = new_pos;
            let empty_space: Option<(i32, i32)> = loop {
                match grid.get(&space) {
                    Some(Object::Wall) => break None,
                    Some(Object::Space) => {
                        break Some(space);
                    }
                    Some(Object::Box) => {
                        space = (space.0 + direction.0, space.1 + direction.1);
                        continue;
                    }
                    _ => break Some(pos),
                }
            };

            match empty_space {
                None => pos,
                Some(space) => {
                    grid.insert(space, Object::Box);
                    grid.insert(new_pos, Object::Space);

                    new_pos
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, movements, start) = parse_input(input);
    let (mut grid, mut start) = expand_grid(grid, start);
    // print_grid(&grid, start);

    for c in movements {
        // println!("Move: {c:?}");
        start = step_part_two(&mut grid, c, start);
        // print_grid(&grid, start);
    }

    Some(calculate_result(&grid))
}

fn step_part_two(
    grid: &mut HashMap<(i32, i32), Object>,
    step: Direction,
    pos: (i32, i32),
) -> (i32, i32) {
    match step {
        Direction::Left | Direction::Right => horizontal_step(grid, step, pos),
        Direction::Up | Direction::Down => vertical_step(grid, step, pos),
    }
}

fn horizontal_step(
    grid: &mut HashMap<(i32, i32), Object>,
    step: Direction,
    pos: (i32, i32),
) -> (i32, i32) {
    fn maybe_move_box(
        grid: &mut HashMap<(i32, i32), Object>,
        step: (i32, i32),
        pos: (i32, i32),
    ) -> Option<(i32, i32)> {
        let start = (pos.0 + step.0, pos.1 + step.1);
        let mut new_pos = start;

        let space = loop {
            let &c = match grid.get(&new_pos) {
                None => break None,
                Some(c) => c,
            };

            match c {
                Object::Wall => break None,
                Object::Space => break Some(new_pos),
                Object::Box | Object::BoxLeftEdge | Object::BoxRightEdge => {}
            }

            new_pos = (new_pos.0 + step.0, new_pos.1 + step.1);
        };

        match space {
            None => None,
            Some(mut space) => {
                while space != start {
                    let next = (space.0 - step.0, space.1 - step.1);
                    grid.insert(space, *grid.get(&next).unwrap());

                    space = next;
                }

                grid.insert(start, Object::Space);
                Some(start)
            }
        }
    }

    let direction = step.to_tuple();
    let new_pos = (pos.0 + direction.0, pos.1 + direction.1);

    let &c = match grid.get(&new_pos) {
        None => return pos,
        Some(c) => c,
    };

    match c {
        Object::Box => panic!("\"O\" boxes shouldn't be part of an extended grid"),
        Object::Wall => pos,      // can't move into a wall
        Object::Space => new_pos, // can move into an empty spot
        Object::BoxLeftEdge | Object::BoxRightEdge => {
            // @[]
            maybe_move_box(grid, direction, pos).unwrap_or(pos)
        }
    }
}

fn vertical_step(
    grid: &mut HashMap<(i32, i32), Object>,
    step: Direction,
    pos: (i32, i32),
) -> (i32, i32) {
    fn can_move(
        grid: &mut HashMap<(i32, i32), Object>,
        step: (i32, i32),
        boxes: Vec<((i32, i32), (i32, i32))>,
        moves: &mut HashMap<(i32, i32), Object>,
    ) -> bool {
        // there is some duplicate processing here, but I can't be bothered anymore
        for ((y0, x0), (y1, x1)) in boxes.iter() {
            let new_left = (y0 + step.0, x0 + step.1);
            let new_right = (y1 + step.0, x1 + step.1);

            match (grid.get(&new_left), grid.get(&new_right)) {
                (_, Some(Object::Wall)) | (Some(Object::Wall), _) => return false,
                (Some(Object::Space), Some(Object::Space)) => continue,
                _ => {
                    // ????
                    //  []
                    let a = (new_left.0, new_left.1 - 1);
                    let b = new_left;
                    let c = new_right;
                    let d = (new_right.0, new_right.1 + 1);

                    match (grid.get(&a), grid.get(&b), grid.get(&c), grid.get(&d)) {
                        // [][]
                        //  []
                        (
                            Some(Object::BoxLeftEdge),
                            Some(Object::BoxRightEdge),
                            Some(Object::BoxLeftEdge),
                            Some(Object::BoxRightEdge),
                        ) => {
                            if !can_move(grid, step, vec![(a, b), (c, d)], moves) {
                                return false;
                            }
                        }
                        // []..
                        //  []
                        (Some(Object::BoxLeftEdge), Some(Object::BoxRightEdge), _, _) => {
                            if !can_move(grid, step, vec![(a, b)], moves) {
                                return false;
                            }
                        }
                        // ..[]
                        //  []
                        (_, _, Some(Object::BoxLeftEdge), Some(Object::BoxRightEdge)) => {
                            if !can_move(grid, step, vec![(c, d)], moves) {
                                return false;
                            }
                        }
                        // .[].
                        //  []
                        (_, Some(Object::BoxLeftEdge), Some(Object::BoxRightEdge), _) => {
                            if !can_move(grid, step, vec![(b, c)], moves) {
                                return false;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        for ((y0, x0), (y1, x1)) in boxes {
            let new_left = (y0 + step.0, x0 + step.1);
            let new_right = (y1 + step.0, x1 + step.1);

            moves.entry((y0, x0)).or_insert(Object::Space);
            moves.entry((y1, x1)).or_insert(Object::Space);

            moves.insert(new_left, Object::BoxLeftEdge);
            moves.insert(new_right, Object::BoxRightEdge);
        }

        true
    }

    let direction = step.to_tuple();
    let new_pos = (pos.0 + direction.0, pos.1 + direction.1);

    let &c = match grid.get(&new_pos) {
        None => return pos,
        Some(c) => c,
    };

    let mut moves = HashMap::new();

    match c {
        Object::Box => panic!("\"O\" boxes shouldn't be part of an extended grid"),
        Object::Wall => pos,      // can't move into a wall
        Object::Space => new_pos, // can move into an empty spot
        Object::BoxLeftEdge => {
            // []
            // @.
            let boxes = vec![((new_pos), (new_pos.0, new_pos.1 + 1))];
            if can_move(grid, direction, boxes, &mut moves) {
                for (pos, val) in moves {
                    grid.insert(pos, val);
                }

                new_pos
            } else {
                pos
            }
        }
        Object::BoxRightEdge => {
            // []
            // .@
            let boxes = vec![((new_pos.0, new_pos.1 - 1), new_pos)];
            if can_move(grid, direction, boxes, &mut moves) {
                for (pos, val) in moves {
                    grid.insert(pos, val);
                }

                new_pos
            } else {
                pos
            }
        }
    }
}

fn expand_grid(
    grid: HashMap<(i32, i32), Object>,
    start: (i32, i32),
) -> (HashMap<(i32, i32), Object>, (i32, i32)) {
    let expanded = grid
        .iter()
        .flat_map(|(&k, &v)| {
            let (p1, p2) = ((k.0, k.1 * 2), (k.0, k.1 * 2 + 1));

            let (v1, v2) = match v {
                Object::Wall | Object::Space => (v, v),
                Object::Box => (Object::BoxLeftEdge, Object::BoxRightEdge),
                c => panic!("unknown object: {:?}", c),
            };

            vec![(p1, v1), (p2, v2)]
        })
        .collect::<HashMap<(i32, i32), Object>>();

    (expanded, (start.0, start.1 * 2))
}

fn parse_input(input: &str) -> (HashMap<(i32, i32), Object>, Vec<Direction>, (i32, i32)) {
    let mut split = input.split("\n\n");
    let grid = split.next().unwrap();
    let movements = split.next().unwrap();

    let (start, grid) = parse_grid(grid);

    let movements: Vec<Direction> = movements
        .lines()
        .map(|line| line.trim())
        .flat_map(|line| line.chars())
        .map(Direction::from_char)
        .collect();

    (grid, movements, start)
}

fn parse_grid(grid: &str) -> ((i32, i32), HashMap<(i32, i32), Object>) {
    let mut start: Option<(i32, i32)> = None;

    let grid: HashMap<(i32, i32), Object> = grid
        .lines()
        .map(|line| line.trim())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '@' => {
                        start = Some((y as i32, x as i32));
                        ((y as i32, x as i32), Object::Space)
                    }
                    _ => ((y as i32, x as i32), Object::from_char(c)),
                })
                .collect::<Vec<((i32, i32), Object)>>()
        })
        .collect();

    (start.unwrap(), grid)
}

fn print_grid(grid: &HashMap<(i32, i32), Object>, start: (i32, i32)) {
    // Determine the grid bounds
    let min_x = grid.keys().map(|&(_, x)| x).min().unwrap_or(0);
    let max_x = grid.keys().map(|&(_, x)| x).max().unwrap_or(0);
    let min_y = grid.keys().map(|&(y, _)| y).min().unwrap_or(0);
    let max_y = grid.keys().map(|&(y, _)| y).max().unwrap_or(0);

    // Print the grid row by row
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if (y, x) == start {
                print!("@");
            } else {
                print!("{}", grid.get(&(y, x)).unwrap());
            }
        }
        println!();
    }
    println!(); // Add an extra newline for better readability
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two_example_1() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        let result = part_two(input);
        assert_eq!(result, Some(618));
    }
    #[test]
    fn test_part_two_example_2() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let result = part_two(input);
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_step() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (mut grid, _, start) = parse_input(input);

        // ########
        // #..O.O.#
        // ##@.O..#
        let new_pos = step(&mut grid, Direction::Left, start);
        assert_eq!(new_pos, start);

        // ########
        // #..O.O.#
        // ##@.O..#
        let new_pos = step(&mut grid, Direction::Up, start);
        assert_eq!(new_pos, (1, 2));

        // ########
        // #..O.O.#
        // ##.@O..#
        let new_pos = step(&mut grid, Direction::Right, (2, 2));
        assert_eq!(new_pos, (2, 3));

        // ########
        // #..O.O.#
        // ##..@O.#
        assert_eq!(grid.get(&(2, 3)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 4)), Some(&Object::Box));

        let new_pos = step(&mut grid, Direction::Right, (2, 3));
        assert_eq!(new_pos, (2, 4));

        assert_eq!(grid.get(&(2, 4)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 5)), Some(&Object::Box));

        let new_pos = step(&mut grid, Direction::Right, (2, 4));
        assert_eq!(new_pos, (2, 5));

        assert_eq!(grid.get(&(2, 5)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 6)), Some(&Object::Box));

        let new_pos = step(&mut grid, Direction::Right, (2, 5));
        assert_eq!(new_pos, (2, 5));

        assert_eq!(grid.get(&(2, 5)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 6)), Some(&Object::Box));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1751));
    }

    #[test]
    fn test_horizontal_step_case_1() {
        // Case 1: ##...[]..@..##
        // Case 2: ##...[][]@..##
        // Case 3: ##@...[][]..##
        // Case 4: ##[][]@.....##
        let (start, mut grid) = parse_grid("##...[]..@..##");

        let new_pos = horizontal_step(&mut grid, Direction::Left, start);
        assert_eq!(new_pos, (0, start.1 - 1));
    }

    #[test]
    fn test_horizontal_step_case_2() {
        // Case 2: ##...[][]@..##
        // Case 2: ##..[][]@...##
        let (start, mut grid) = parse_grid("##...[][]@..##");
        assert_eq!(start, (0, 9));

        let new_pos = horizontal_step(&mut grid, Direction::Left, start);
        assert_eq!(new_pos, (0, 8));
        assert_eq!(Some(&Object::Space), grid.get(&new_pos));

        assert_eq!(Some(&Object::BoxLeftEdge), grid.get(&(0, 4)));
        assert_eq!(Some(&Object::BoxRightEdge), grid.get(&(0, 5)));

        assert_eq!(Some(&Object::BoxLeftEdge), grid.get(&(0, 6)));
        assert_eq!(Some(&Object::BoxRightEdge), grid.get(&(0, 7)));
    }

    #[test]
    fn test_horizontal_step_case_3() {
        // Case 3: ##@...[][]..##
        let (start, mut grid) = parse_grid("##@...[][]..##");

        let new_pos = horizontal_step(&mut grid, Direction::Left, start);
        assert_eq!(new_pos, start);
    }

    #[test]
    fn test_horizontal_step_case_4() {
        // Case 4: ##[][]@.....##
        let (start, mut grid) = parse_grid("##[][]@.....##");

        let new_pos = horizontal_step(&mut grid, Direction::Left, start);
        assert_eq!(new_pos, start);

        assert_eq!(Some(&Object::BoxLeftEdge), grid.get(&(0, 2)));
        assert_eq!(Some(&Object::BoxRightEdge), grid.get(&(0, 3)));

        assert_eq!(Some(&Object::BoxLeftEdge), grid.get(&(0, 4)));
        assert_eq!(Some(&Object::BoxRightEdge), grid.get(&(0, 5)));
    }

    #[test]
    fn test_horizontal_step_case_5() {
        // Case 5: #@[].. --> #.@[].
        let (start, mut grid) = parse_grid("#@[]..");

        let new_pos = horizontal_step(&mut grid, Direction::Right, start);

        assert_eq!(new_pos, (0, 2));
        assert_eq!(Some(&Object::Space), grid.get(&(0, 2)));
        assert_eq!(Some(&Object::BoxLeftEdge), grid.get(&(0, 3)));
        assert_eq!(Some(&Object::BoxRightEdge), grid.get(&(0, 4)));
    }

    #[test]
    fn test_horizontal_step_case_6() {
        // Case 5: #@[].. --> #.@[].
        let (start, mut grid) = parse_grid("#@[][].#");

        let new_pos = horizontal_step(&mut grid, Direction::Right, start);

        // assert_eq!(new_pos, (0, 2));
        // assert_eq!(Some(&Object::Space), grid.get(&(0, 2)));
        // assert_eq!(Some(&Object::BoxLeftEdge), grid.get(&(0, 3)));
        // assert_eq!(Some(&Object::BoxRightEdge), grid.get(&(0, 4)));

        let (expected_start, expected_grid) = parse_grid("#.@[][]#");
        assert_eq!(new_pos, expected_start);
        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn test_horizontal_step_case_7() {
        // Case 5: #@[].. --> #.@[].
        let (start, mut grid) = parse_grid("#@[][]#");

        let new_pos = horizontal_step(&mut grid, Direction::Right, start);

        let (expected_start, expected_grid) = parse_grid("#@[][]#");
        assert_eq!(new_pos, expected_start);
        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn test_vertical_step_case_1() {
        // ##.....##
        // ##@....##
        let (start, mut grid) = parse_grid("##.....##\n##@....##");

        let new_pos = vertical_step(&mut grid, Direction::Up, start);
        assert_eq!(new_pos, (0, 2));
    }

    #[test]
    fn test_vertical_step_case_2() {
        // ##
        // #@
        let (start, mut grid) = parse_grid("##\n#@");

        let new_pos = vertical_step(&mut grid, Direction::Up, start);
        assert_eq!(new_pos, (1, 1));
    }

    #[test]
    fn test_vertical_step_case_3() {
        // ...
        // #[]
        // #[]
        // #@.
        let (start, mut grid) = parse_grid("...\n#[]\n#[]\n#@.");

        let new_pos = vertical_step(&mut grid, Direction::Up, start);
        assert_eq!(new_pos, (2, 1));

        assert_eq!(grid.get(&(0, 1)), Some(&Object::BoxLeftEdge));
        assert_eq!(grid.get(&(0, 2)), Some(&Object::BoxRightEdge));

        assert_eq!(grid.get(&(1, 1)), Some(&Object::BoxLeftEdge));
        assert_eq!(grid.get(&(1, 2)), Some(&Object::BoxRightEdge));

        assert_eq!(grid.get(&(2, 1)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 2)), Some(&Object::Space));

        assert_eq!(grid.get(&(3, 1)), Some(&Object::Space));
        assert_eq!(grid.get(&(3, 2)), Some(&Object::Space));
    }

    #[test]
    fn test_vertical_step_case_4() {
        // ....
        // [][]
        // #[].
        // #@..
        let (start, mut grid) = parse_grid("....\n[][]\n#[].\n#@..");

        let new_pos = vertical_step(&mut grid, Direction::Up, start);
        assert_eq!(new_pos, (2, 1));

        assert_eq!(grid.get(&(0, 0)), Some(&Object::BoxLeftEdge));
        assert_eq!(grid.get(&(0, 1)), Some(&Object::BoxRightEdge));
        assert_eq!(grid.get(&(0, 2)), Some(&Object::BoxLeftEdge));
        assert_eq!(grid.get(&(0, 3)), Some(&Object::BoxRightEdge));

        assert_eq!(grid.get(&(1, 1)), Some(&Object::BoxLeftEdge));
        assert_eq!(grid.get(&(1, 2)), Some(&Object::BoxRightEdge));

        assert_eq!(grid.get(&(2, 1)), Some(&Object::Space));
        assert_eq!(grid.get(&(2, 2)), Some(&Object::Space));

        assert_eq!(grid.get(&(3, 1)), Some(&Object::Space));
        assert_eq!(grid.get(&(3, 2)), Some(&Object::Space));
    }

    #[test]
    fn test_vertical_step_case_5() {
        // #.........@..
        // #.........[].
        // #........[]..
        // #.......[][].
        // #......[][][]
        // #............
        let input = "...@..
        ...[].
        ..[]..
        .[][].
        [][][]
        .[]...
        ......";

        let (start, mut grid) = parse_grid(input);

        print_grid(&grid, start);

        let new_pos = vertical_step(&mut grid, Direction::Down, start);
        print_grid(&grid, new_pos);
    }

    #[test]
    fn test_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (grid, movements, start) = parse_input(input);

        assert_eq!(start, (2, 2));
        assert_eq!(
            movements,
            vec![
                Direction::Left,
                Direction::Up,
                Direction::Up,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Down,
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left
            ]
        );
        assert_eq!(
            grid,
            HashMap::from([
                ((0, 0), Object::Wall),
                ((0, 1), Object::Wall),
                ((0, 2), Object::Wall),
                ((0, 3), Object::Wall),
                ((0, 4), Object::Wall),
                ((0, 5), Object::Wall),
                ((0, 6), Object::Wall),
                ((0, 7), Object::Wall),
                ((1, 0), Object::Wall),
                ((1, 1), Object::Space),
                ((1, 2), Object::Space),
                ((1, 3), Object::Box),
                ((1, 4), Object::Space),
                ((1, 5), Object::Box),
                ((1, 6), Object::Space),
                ((1, 7), Object::Wall),
                ((2, 0), Object::Wall),
                ((2, 1), Object::Wall),
                ((2, 2), Object::Space),
                ((2, 3), Object::Space),
                ((2, 4), Object::Box),
                ((2, 5), Object::Space),
                ((2, 6), Object::Space),
                ((2, 7), Object::Wall),
                ((3, 0), Object::Wall),
                ((3, 1), Object::Space),
                ((3, 2), Object::Space),
                ((3, 3), Object::Space),
                ((3, 4), Object::Box),
                ((3, 5), Object::Space),
                ((3, 6), Object::Space),
                ((3, 7), Object::Wall),
                ((4, 0), Object::Wall),
                ((4, 1), Object::Space),
                ((4, 2), Object::Wall),
                ((4, 3), Object::Space),
                ((4, 4), Object::Box),
                ((4, 5), Object::Space),
                ((4, 6), Object::Space),
                ((4, 7), Object::Wall),
                ((5, 0), Object::Wall),
                ((5, 1), Object::Space),
                ((5, 2), Object::Space),
                ((5, 3), Object::Space),
                ((5, 4), Object::Box),
                ((5, 5), Object::Space),
                ((5, 6), Object::Space),
                ((5, 7), Object::Wall),
                ((6, 0), Object::Wall),
                ((6, 1), Object::Space),
                ((6, 2), Object::Space),
                ((6, 3), Object::Space),
                ((6, 4), Object::Space),
                ((6, 5), Object::Space),
                ((6, 6), Object::Space),
                ((6, 7), Object::Wall),
                ((7, 0), Object::Wall),
                ((7, 1), Object::Wall),
                ((7, 2), Object::Wall),
                ((7, 3), Object::Wall),
                ((7, 4), Object::Wall),
                ((7, 5), Object::Wall),
                ((7, 6), Object::Wall),
                ((7, 7), Object::Wall),
            ])
        );
    }
}
