use std::collections::HashMap;

enum Tile {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn move_dir(&self, dir: Direction) -> Self {
        let offset = match dir {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };

        Self {
            x: self.x.checked_add_signed(offset.0).unwrap(),
            y: self.y.checked_add_signed(offset.1).unwrap(),
        }
    }
}

type Map = Vec<Vec<Tile>>;

fn parse_input(input: &str) -> (Map, Point, Point) {
    let mut start = None;
    let mut end = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, col)| match col {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => {
                        start = Some(Point { x, y });
                        Tile::Empty
                    }
                    'E' => {
                        end = Some(Point { x, y });
                        Tile::Empty
                    }
                    _ => panic!("Invalid map char: {}", col),
                })
                .collect()
        })
        .collect();

    (map, start.unwrap(), end.unwrap())
}

fn solve1(map: &Map, start: Point, end: Point) -> usize {
    let mut moves = Vec::new();
    moves.push((0, start, Direction::East));
    let mut best_at_loc = HashMap::new();

    while let Some((score, cur_loc, dir)) = moves.pop() {
        let best_so_far = best_at_loc.entry((cur_loc, dir)).or_insert(usize::MAX);
        if score < *best_so_far {
            *best_so_far = score;
        } else {
            continue;
        }

        if cur_loc == end {
            return score;
        }
        for possible_dir in [dir, dir.rotate_left(), dir.rotate_right()] {
            let new_loc = cur_loc.move_dir(possible_dir);
            if let Tile::Empty = map[new_loc.y][new_loc.x] {
                let mut new_score = score + 1;
                if possible_dir != dir {
                    new_score += 1000;
                }

                moves.push((new_score, new_loc, possible_dir));
                moves.sort_by(|a, b| a.0.cmp(&b.0).reverse());
            }
        }
    }

    0
}

fn solve2(map: &Map, start: Point, end: Point) -> usize {
    let mut moves = Vec::new();
    moves.push((
        0,
        start,
        Direction::East,
        vec![(0, (start, Direction::East))],
    ));
    let mut best_at_loc = HashMap::new();
    let mut best_path: HashMap<Point, usize> = HashMap::new();
    let mut best_score = usize::MAX;

    while let Some((score, cur_loc, dir, path)) = moves.pop() {
        let best_so_far = best_at_loc.entry((cur_loc, dir)).or_insert(usize::MAX);
        if score <= *best_so_far {
            *best_so_far = score;
        } else {
            continue;
        }

        if cur_loc == end && score <= best_score {
            best_score = score;
            for p in path {
                best_path.insert(p.1 .0, p.0);
            }
            continue;
        } else if best_path.contains_key(&cur_loc) {
            let val = best_path.get(&cur_loc).unwrap();
            if score <= *val {
                for p in path {
                    best_path.insert(p.1 .0, p.0);
                }
            }

            continue;
        }
        for possible_dir in [dir, dir.rotate_left(), dir.rotate_right()] {
            let new_loc = cur_loc.move_dir(possible_dir);
            if let Tile::Empty = map[new_loc.y][new_loc.x] {
                let mut new_score = score + 1;
                if possible_dir != dir {
                    new_score += 1000;
                }
                let mut new_path = path.clone();
                new_path.push((new_score, (new_loc, possible_dir)));

                moves.push((new_score, new_loc, possible_dir, new_path));
                moves.sort_by(|a, b| a.0.cmp(&b.0).reverse());
            }
        }
    }

    best_path.len()
}

fn main() {
    let (map, start, end) = parse_input(include_str!("input.txt"));
    println!("Part 1: {}", solve1(&map, start, end));
    println!("Part 2: {}", solve2(&map, start, end));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("test1.txt");
    const INPUT2: &str = include_str!("test2.txt");

    #[test]
    fn parse_test() {
        let _parsed = parse_input(INPUT1);
        let _parsed = parse_input(INPUT2);
    }

    #[test]
    fn solve1_test1() {
        let (map, start, end) = parse_input(INPUT1);
        assert_eq!(solve1(&map, start, end), 7036);
    }

    #[test]
    fn solve1_test2() {
        let (map, start, end) = parse_input(INPUT2);
        assert_eq!(solve1(&map, start, end), 11048);
    }

    #[test]
    fn solve2_test1() {
        let (map, start, end) = parse_input(INPUT1);
        assert_eq!(solve2(&map, start, end), 45);
    }

    #[test]
    fn solve2_test2() {
        let (map, start, end) = parse_input(INPUT2);
        assert_eq!(solve2(&map, start, end), 64);
    }
}
