use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn within_bounds(&self, map: &Map) -> bool {
        self.x >= map.min_x && self.x <= map.max_x && self.y >= map.min_y && self.y <= map.max_y
    }
}

struct Map {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    antennas: HashMap<char, Vec<Point>>,
}

fn parse_input(input: &str) -> Map {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    for (y, row) in input.lines().enumerate() {
        max_y = y as i64;
        if max_x == 0 {
            max_x = row.chars().count() as i64 - 1;
        }
        for (x, col) in row.chars().enumerate() {
            if col != '.' {
                let antenna = Point {
                    x: x as i64,
                    y: y as i64,
                };
                antennas.entry(col).or_default().push(antenna);
            }
        }
    }

    Map {
        min_x: 0,
        max_x,
        min_y: 0,
        max_y,
        antennas,
    }
}

mod part1 {
    use super::*;

    pub fn get_antinodes(antenna1: &Point, antenna2: &Point) -> (Point, Point) {
        let diff_x = antenna1.x - antenna2.x;
        let diff_y = antenna1.y - antenna2.y;

        (
            Point {
                x: antenna1.x + diff_x,
                y: antenna1.y + diff_y,
            },
            Point {
                x: antenna2.x - diff_x,
                y: antenna2.y - diff_y,
            },
        )
    }

    pub fn solve1(input: &Map) -> usize {
        let mut locations = HashSet::new();
        for (_, antennas) in input.antennas.iter() {
            for i in 0..antennas.len() {
                for j in i + 1..antennas.len() {
                    let (a1, a2) = get_antinodes(&antennas[i], &antennas[j]);
                    if a1.within_bounds(input) {
                        locations.insert(a1);
                    }
                    if a2.within_bounds(input) {
                        locations.insert(a2);
                    }
                }
            }
        }

        locations.len()
    }
}

mod part2 {
    use super::*;

    fn get_antinodes(antenna1: &Point, antenna2: &Point, map: &Map) -> Vec<Point> {
        let diff_x = antenna1.x - antenna2.x;
        let diff_y = antenna1.y - antenna2.y;

        let mut antinodes = vec![];

        // adding diff
        let mut cur_node = *antenna1;
        while cur_node.within_bounds(map) {
            antinodes.push(cur_node);
            cur_node.x += diff_x;
            cur_node.y += diff_y
        }

        // subtracting diff
        let mut cur_node = *antenna2;
        while cur_node.within_bounds(map) {
            antinodes.push(cur_node);
            cur_node.x -= diff_x;
            cur_node.y -= diff_y
        }

        antinodes
    }

    pub fn solve2(input: &Map) -> usize {
        let mut locations = HashSet::new();
        for (_, antennas) in input.antennas.iter() {
            for i in 0..antennas.len() {
                for j in i + 1..antennas.len() {
                    for antinode in get_antinodes(&antennas[i], &antennas[j], input) {
                        locations.insert(antinode);
                    }
                }
            }
        }

        locations.len()
    }
}

fn main() {
    let parsed = parse_input(include_str!("input.txt"));

    println!("Part 1: {}", part1::solve1(&parsed));
    println!("Part 2: {}", part2::solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("test1.txt");
    const INPUT2: &str = include_str!("test2.txt");
    const INPUT3: &str = include_str!("test3.txt");

    #[test]
    fn parse_test() {
        let parsed = parse_input(INPUT1);
        assert_eq!(
            parsed.antennas[&'a'],
            &[Point { x: 4, y: 3 }, Point { x: 5, y: 5 }]
        );
        assert_eq!(parsed.max_x, 9);
        assert_eq!(parsed.max_y, 9);
    }

    #[test]
    fn test_antinodes() {
        let antenna1 = Point { x: 4, y: 3 };
        let antenna2 = Point { x: 5, y: 5 };

        assert_eq!(
            part1::get_antinodes(&antenna1, &antenna2),
            (Point { x: 3, y: 1 }, Point { x: 6, y: 7 })
        );
        assert_eq!(
            part1::get_antinodes(&antenna2, &antenna1),
            (Point { x: 6, y: 7 }, Point { x: 3, y: 1 })
        );
    }

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT1);
        assert_eq!(part1::solve1(&parsed), 2);

        let parsed = parse_input(INPUT2);
        assert_eq!(part1::solve1(&parsed), 14);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT3);
        assert_eq!(part2::solve2(&parsed), 9);

        let parsed = parse_input(INPUT2);
        assert_eq!(part2::solve2(&parsed), 34);
    }
}
