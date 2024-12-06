use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn add_dir(&self, direction: Direction) -> Self {
        let offset: (i64, i64) = direction.into();
        Self {
            x: self.x + offset.0,
            y: self.y + offset.1,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    obstacles: HashSet<Position>,
}

impl Map {
    fn in_map(&self, pos: &Position) -> bool {
        pos.x >= self.min_x && pos.x <= self.max_x && pos.y >= self.min_y && pos.y <= self.max_y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn do_move(&mut self, obstacles: &HashSet<Position>) {
        let candidate_pos = self.position.add_dir(self.direction);
        if !obstacles.contains(&candidate_pos) {
            self.position = candidate_pos;
        } else {
            self.direction = self.direction.rotate_right();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_right(self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::South => Self::West,
            Direction::East => Self::South,
            Direction::West => Self::North,
        }
    }
}

impl From<Direction> for (i64, i64) {
    fn from(val: Direction) -> Self {
        match val {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

fn parse_input(input: &str) -> (Guard, Map) {
    let mut guard = None;
    let mut obstacles = HashSet::new();

    let mut max_x = 0;
    let mut max_y = 0;
    for (y, row) in input.lines().enumerate() {
        max_y = y as i64;
        max_x = row.chars().count() as i64 - 1;
        for (x, col) in row.chars().enumerate() {
            match col {
                '.' => (),
                '^' => {
                    guard = Some(Guard {
                        position: Position {
                            x: x as i64,
                            y: y as i64,
                        },
                        direction: Direction::North,
                    })
                }
                '#' => {
                    obstacles.insert(Position {
                        x: x as i64,
                        y: y as i64,
                    });
                }
                _ => panic!("Invalid char: {col}"),
            }
        }
    }

    let Some(guard) = guard else {
        panic!("Didn't find guard");
    };

    let mut by_x: HashMap<i64, HashSet<Position>> = HashMap::new();
    let mut by_y: HashMap<i64, HashSet<Position>> = HashMap::new();

    for obs in obstacles.iter() {
        by_x.entry(obs.x).or_default().insert(*obs);
        by_y.entry(obs.y).or_default().insert(*obs);
    }

    (
        guard,
        Map {
            min_x: 0,
            max_x,
            min_y: 0,
            max_y,
            obstacles,
        },
    )
}

fn solve1(mut guard: Guard, map: &Map) -> usize {
    let mut visited = HashSet::new();
    visited.insert(guard.position);

    while map.in_map(&guard.position) {
        guard.do_move(&map.obstacles);
        visited.insert(guard.position);
    }

    visited.len() - 1
}

fn solve2(guard: &Guard, mut map: Map) -> usize {
    let mut total = 0;

    for x in map.min_x..=map.max_x {
        for y in map.min_y..=map.max_y {
            let candidate_obstacle_position = Position { x, y };
            if candidate_obstacle_position != guard.position
                && !map.obstacles.contains(&candidate_obstacle_position)
            {
                map.obstacles.insert(candidate_obstacle_position);
                let mut guard_copy = *guard;

                let mut visited = HashSet::new();
                visited.insert(guard_copy);

                while map.in_map(&guard_copy.position) {
                    guard_copy.do_move(&map.obstacles);
                    if visited.contains(&guard_copy) {
                        total += 1;
                        break;
                    }
                    visited.insert(guard_copy);
                }

                map.obstacles.remove(&candidate_obstacle_position);
            }
        }
    }

    total
}

fn main() {
    let (guard, map) = parse_input(include_str!("input.txt"));

    println!("Part 1: {}", solve1(guard, &map));
    println!("Part 2: {}", solve2(&guard, map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn parse_test() {
        let (guard, _obstacles) = parse_input(INPUT);
        assert_eq!(
            guard,
            Guard {
                position: Position { x: 4, y: 6 },
                direction: Direction::North
            }
        );
    }

    #[test]
    fn test_solve1() {
        let (guard, map) = parse_input(INPUT);
        assert_eq!(solve1(guard, &map), 41);
    }

    #[test]
    fn test_solve2() {
        let (guard, map) = parse_input(INPUT);
        assert_eq!(solve2(&guard, map), 6);
    }
}
