#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn move_point(&self, x: usize, y: usize) -> (usize, usize) {
        let diff = match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        let x = x.checked_add_signed(diff.0).unwrap();
        let y = y.checked_add_signed(diff.1).unwrap();

        (x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Box,
    Wall,
}

type Map = Vec<Vec<Tile>>;

mod parser {
    use super::*;

    use nom::{
        bytes::complete::take_while1,
        character::complete::newline,
        combinator::map,
        multi::{many1, separated_list1},
        sequence::{separated_pair, terminated},
        IResult,
    };

    pub fn parse_input(input: &str) -> (Map, Vec<Direction>, (usize, usize)) {
        let parsed = separated_pair(parse_map, newline, parse_directions)(input).unwrap();
        assert!(parsed.0.is_empty());
        (parsed.1 .0 .0, parsed.1 .1, parsed.1 .0 .1)
    }

    fn parse_map(input: &str) -> IResult<&str, (Map, (usize, usize))> {
        map(
            many1(terminated(
                take_while1(|c| c == '#' || c == '.' || c == 'O' || c == '@'),
                newline,
            )),
            |l: Vec<&str>| {
                let mut map = Vec::new();
                let mut robot_loc = None;

                for (y, line) in l.iter().enumerate() {
                    let row = line
                        .chars()
                        .enumerate()
                        .map(|(x, c)| match c {
                            '#' => Tile::Wall,
                            '.' => Tile::Empty,
                            'O' => Tile::Box,
                            '@' => {
                                robot_loc = Some((x, y));
                                Tile::Empty
                            }
                            _ => unreachable!(),
                        })
                        .collect();
                    map.push(row);
                }
                (map, robot_loc.unwrap())
            },
        )(input)
    }

    fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
        map(
            separated_list1(
                newline,
                take_while1(|c| c == '^' || c == 'v' || c == '<' || c == '>'),
            ),
            |l: Vec<&str>| {
                let mut directions = Vec::new();
                for line in l {
                    for chr in line.chars() {
                        directions.push(match chr {
                            '^' => Direction::Up,
                            '>' => Direction::Right,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            _ => unreachable!(),
                        });
                    }
                }
                directions
            },
        )(input)
    }
}

mod part1 {
    use super::*;

    fn push_tile(x: usize, y: usize, direction: Direction, map: &mut Map) -> bool {
        let adjacent = direction.move_point(x, y);

        match map[adjacent.1][adjacent.0] {
            Tile::Empty => {
                map[adjacent.1][adjacent.0] = map[y][x];
                map[y][x] = Tile::Empty;

                true
            }
            Tile::Box => {
                if push_tile(adjacent.0, adjacent.1, direction, map) {
                    map[adjacent.1][adjacent.0] = map[y][x];
                    map[y][x] = Tile::Empty;

                    true
                } else {
                    false
                }
            }
            Tile::Wall => false,
        }
    }

    pub fn solve1(robot_start: (usize, usize), robot_directions: &[Direction], map: &Map) -> usize {
        let mut map = map.to_owned();
        let mut robot_loc = robot_start;

        for dir in robot_directions {
            let (robot_x, robot_y) = robot_loc;
            if push_tile(robot_x, robot_y, *dir, &mut map) {
                robot_loc = dir.move_point(robot_x, robot_y);
            }
        }

        let mut total = 0;
        for (y, row) in map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let Tile::Box = col {
                    total += 100 * y + x;
                }
            }
        }
        total
    }
}

mod part2 {
    use super::*;

    type BoxToMove = ((usize, usize), (usize, usize));

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Part2Tile {
        Empty,
        BoxLeft,
        BoxRight,
        Wall,
    }

    type Part2Map = Vec<Vec<Part2Tile>>;

    fn expand_map(map: &Map) -> Part2Map {
        map.iter()
            .map(|row| {
                let mut new_row = Vec::new();
                for col in row.iter() {
                    match col {
                        Tile::Empty => {
                            new_row.push(Part2Tile::Empty);
                            new_row.push(Part2Tile::Empty);
                        }
                        Tile::Box => {
                            new_row.push(Part2Tile::BoxLeft);
                            new_row.push(Part2Tile::BoxRight);
                        }
                        Tile::Wall => {
                            new_row.push(Part2Tile::Wall);
                            new_row.push(Part2Tile::Wall);
                        }
                    }
                }
                new_row
            })
            .collect()
    }

    fn get_box_left(x: usize, y: usize, map: &Part2Map) -> (usize, usize) {
        match map[y][x] {
            Part2Tile::Empty | Part2Tile::Wall => panic!("Didn't provide box coords"),
            Part2Tile::BoxLeft => (x, y),
            Part2Tile::BoxRight => (x - 1, y),
        }
    }

    fn can_push_tile(
        x: usize,
        y: usize,
        direction: Direction,
        map: &Part2Map,
    ) -> Option<Vec<BoxToMove>> {
        match map[y][x] {
            Part2Tile::Empty => Some(vec![]),
            Part2Tile::BoxLeft | Part2Tile::BoxRight => {
                let (box_x, box_y) = get_box_left(x, y, map);
                let moved_children = match direction {
                    Direction::Up => {
                        if let (Some(mut left), Some(right)) = (
                            can_push_tile(box_x, box_y - 1, direction, map),
                            can_push_tile(box_x + 1, box_y - 1, direction, map),
                        ) {
                            left.extend(right);
                            Some(left)
                        } else {
                            None
                        }
                    }
                    Direction::Right => can_push_tile(box_x + 2, y, direction, map),
                    Direction::Down => {
                        if let (Some(mut left), Some(right)) = (
                            can_push_tile(box_x, box_y + 1, direction, map),
                            can_push_tile(box_x + 1, box_y + 1, direction, map),
                        ) {
                            left.extend(right);
                            Some(left)
                        } else {
                            None
                        }
                    }
                    Direction::Left => can_push_tile(box_x - 1, y, direction, map),
                };
                if let Some(moved_children) = moved_children {
                    let mut moved = vec![((box_x, box_y), (box_x + 1, box_y))];
                    moved.extend(moved_children);
                    Some(moved)
                } else {
                    None
                }
            }
            Part2Tile::Wall => None,
        }
    }

    pub fn solve2(robot_start: (usize, usize), robot_directions: &[Direction], map: &Map) -> usize {
        let mut map = expand_map(map);
        let mut robot_loc = robot_start;
        robot_loc.0 *= 2;

        for dir in robot_directions {
            let (robot_x, robot_y) = robot_loc;
            let possible_robot_loc = dir.move_point(robot_x, robot_y);
            if let Some(moved_tiles) =
                can_push_tile(possible_robot_loc.0, possible_robot_loc.1, *dir, &map)
            {
                for (left, right) in moved_tiles.iter() {
                    map[left.1][left.0] = Part2Tile::Empty;
                    map[right.1][right.0] = Part2Tile::Empty;
                }

                for (left, right) in moved_tiles.iter() {
                    let new_loc_left = dir.move_point(left.0, left.1);
                    let new_loc_right = dir.move_point(right.0, right.1);

                    map[new_loc_left.1][new_loc_left.0] = Part2Tile::BoxLeft;
                    map[new_loc_right.1][new_loc_right.0] = Part2Tile::BoxRight;
                }
                robot_loc = possible_robot_loc;
            }
        }

        let mut total = 0;
        for (y, row) in map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let Part2Tile::BoxLeft = col {
                    total += 100 * y + x;
                }
            }
        }
        total
    }
}

fn main() {
    let parsed = parser::parse_input(include_str!("input.txt"));

    println!("Part 1: {}", part1::solve1(parsed.2, &parsed.1, &parsed.0));
    println!("Part 2: {}", part2::solve2(parsed.2, &parsed.1, &parsed.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("test1.txt");
    const INPUT2: &str = include_str!("test2.txt");

    #[test]
    fn parse_test() {
        let _parsed = parser::parse_input(INPUT1);
        let _parsed = parser::parse_input(INPUT2);
    }

    #[test]
    fn solve1_test1() {
        let parsed = parser::parse_input(INPUT1);
        assert_eq!(part1::solve1(parsed.2, &parsed.1, &parsed.0), 2028);
    }

    #[test]
    fn solve1_test2() {
        let parsed = parser::parse_input(INPUT2);
        assert_eq!(part1::solve1(parsed.2, &parsed.1, &parsed.0), 10092);
    }

    #[test]
    fn test2() {
        let parsed = parser::parse_input(INPUT2);
        assert_eq!(part2::solve2(parsed.2, &parsed.1, &parsed.0), 9021);
    }
}
