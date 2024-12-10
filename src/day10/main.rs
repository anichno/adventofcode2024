fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for chr in line.chars() {
            if chr == '.' {
                row.push(255);
            } else {
                row.push(chr.to_digit(10).unwrap() as u8);
            }
        }
        map.push(row);
    }
    map
}

mod part1 {
    use std::collections::HashSet;

    fn follow_trail(
        x: usize,
        y: usize,
        completed_trails: &mut u64,
        visited: &mut HashSet<(usize, usize)>,
        map: &[Vec<u8>],
    ) {
        let cur_height = map[y][x];

        if cur_height == 9 {
            *completed_trails += 1;
            return;
        }

        let next_height = cur_height + 1;
        for diff in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let (Some(new_x), Some(new_y)) =
                (x.checked_add_signed(diff.0), y.checked_add_signed(diff.1))
            {
                if new_x < map[0].len()
                    && new_y < map.len()
                    && map[new_y][new_x] == next_height
                    && !visited.contains(&(new_x, new_y))
                {
                    visited.insert((new_x, new_y));
                    follow_trail(new_x, new_y, completed_trails, visited, map);
                }
            }
        }
    }

    pub fn solve1(input: &[Vec<u8>]) -> u64 {
        let mut completed_trails = 0;

        for (y, row) in input.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == 0 {
                    let mut visited = HashSet::new();
                    follow_trail(x, y, &mut completed_trails, &mut visited, input);
                }
            }
        }
        completed_trails
    }
}

mod part2 {
    use std::collections::HashMap;

    fn follow_trail(
        x: usize,
        y: usize,
        visited: &mut HashMap<(usize, usize), u64>,
        map: &[Vec<u8>],
    ) -> u64 {
        let cur_height = map[y][x];

        if cur_height == 9 {
            return 1;
        }

        let next_height = cur_height + 1;
        let mut tot_paths = 0;
        for diff in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let (Some(new_x), Some(new_y)) =
                (x.checked_add_signed(diff.0), y.checked_add_signed(diff.1))
            {
                if new_x < map[0].len() && new_y < map.len() && map[new_y][new_x] == next_height {
                    if let Some(answer) = visited.get(&(new_x, new_y)) {
                        tot_paths += *answer;
                    } else {
                        tot_paths += follow_trail(new_x, new_y, visited, map);
                    }
                }
            }
        }

        assert!(visited.insert((x, y), tot_paths).is_none());
        tot_paths
    }

    pub fn solve2(input: &[Vec<u8>]) -> u64 {
        let mut total_score = 0;

        for (y, row) in input.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == 0 {
                    let mut visited = HashMap::new();
                    total_score += follow_trail(x, y, &mut visited, input);
                }
            }
        }
        total_score
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
    const INPUT4: &str = include_str!("test4.txt");
    const INPUT5: &str = include_str!("test5.txt");
    const INPUT6: &str = include_str!("test6.txt");
    const INPUT7: &str = include_str!("test7.txt");

    #[test]
    fn parse_test() {
        let parsed = parse_input(INPUT1);
        assert_eq!(parsed[0][0], 0);
        assert_eq!(parsed[3][2], 7);

        let parsed = parse_input(INPUT2);
        assert_eq!(parsed[0][0], 255);
        assert_eq!(parsed[0][3], 0);
    }

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT1);
        assert_eq!(part1::solve1(&parsed), 1);

        let parsed = parse_input(INPUT2);
        assert_eq!(part1::solve1(&parsed), 2);

        let parsed = parse_input(INPUT4);
        assert_eq!(part1::solve1(&parsed), 4);

        let parsed = parse_input(INPUT3);
        assert_eq!(part1::solve1(&parsed), 36);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT5);
        assert_eq!(part2::solve2(&parsed), 3);

        let parsed = parse_input(INPUT6);
        assert_eq!(part2::solve2(&parsed), 13);

        let parsed = parse_input(INPUT7);
        assert_eq!(part2::solve2(&parsed), 227);

        let parsed = parse_input(INPUT3);
        assert_eq!(part2::solve2(&parsed), 81);
    }
}
