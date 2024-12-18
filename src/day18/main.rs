use std::collections::{HashMap, HashSet};

mod parser {
    use nom::bytes::complete::tag;
    use nom::character::complete::{newline, u64 as parse_u64};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::{sequence::separated_pair, IResult};

    pub fn parse_input(input: &str) -> Vec<(usize, usize)> {
        let parsed =
            separated_list1(newline, map(parse_line, |(a, b)| (a as usize, b as usize)))(input)
                .unwrap();

        assert!(parsed.0.is_empty());
        parsed.1
    }

    fn parse_line(input: &str) -> IResult<&str, (u64, u64)> {
        separated_pair(parse_u64, tag(","), parse_u64)(input)
    }
}

fn solve1(input: &[(usize, usize)], grid_size: usize) -> Option<usize> {
    let mut corrupted_spaces: HashSet<(usize, usize)> = HashSet::new();
    corrupted_spaces.extend(input);
    let mut moves = Vec::new();
    let start = (0, 0);
    let end = (grid_size, grid_size);
    moves.push((0, start));
    let mut best_at_loc = HashMap::new();

    while let Some((dist, cur_loc)) = moves.pop() {
        let best_so_far = best_at_loc.entry(cur_loc).or_insert(usize::MAX);
        if dist < *best_so_far {
            *best_so_far = dist;
        } else {
            continue;
        }

        if cur_loc == end {
            return Some(dist);
        }

        for diff in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let Some(new_x) = cur_loc.0.checked_add_signed(diff.0) {
                if let Some(new_y) = cur_loc.1.checked_add_signed(diff.1) {
                    if new_x <= grid_size
                        && new_y <= grid_size
                        && !corrupted_spaces.contains(&(new_x, new_y))
                    {
                        moves.push((dist + 1, (new_x, new_y)));
                        moves.sort_by(|a, b| a.0.cmp(&b.0).reverse());
                    }
                }
            }
        }
    }

    None
}

fn solve2(input: &[(usize, usize)], grid_size: usize) -> Option<(usize, usize)> {
    let start = (0, 0);
    let end = (grid_size, grid_size);
    let mut corrupted_spaces: HashSet<(usize, usize)> = HashSet::new();

    'outer: for byte in input {
        corrupted_spaces.insert(*byte);
        let mut moves = Vec::new();

        moves.push((0, start));
        let mut best_at_loc = HashMap::new();

        while let Some((dist, cur_loc)) = moves.pop() {
            let best_so_far = best_at_loc.entry(cur_loc).or_insert(usize::MAX);
            if dist < *best_so_far {
                *best_so_far = dist;
            } else {
                continue;
            }

            if cur_loc == end {
                continue 'outer;
            }

            for diff in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if let Some(new_x) = cur_loc.0.checked_add_signed(diff.0) {
                    if let Some(new_y) = cur_loc.1.checked_add_signed(diff.1) {
                        if new_x <= grid_size
                            && new_y <= grid_size
                            && !corrupted_spaces.contains(&(new_x, new_y))
                        {
                            moves.push((dist + 1, (new_x, new_y)));
                            moves.sort_by(|a, b| a.0.cmp(&b.0).reverse());
                        }
                    }
                }
            }
        }

        return Some(*byte);
    }

    None
}

fn main() {
    let parsed = parser::parse_input(include_str!("input.txt"));

    println!("Part 1: {}", solve1(&parsed[..1024], 70).unwrap());

    let part2 = solve2(&parsed, 70).unwrap();
    println!("Part 2: {},{}", part2.0, part2.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn parse_test() {
        let _parsed = parser::parse_input(INPUT);
    }

    #[test]
    fn test1() {
        let parsed = parser::parse_input(INPUT);
        assert_eq!(solve1(&parsed[..12], 6), Some(22));
    }

    #[test]
    fn test2() {
        let parsed = parser::parse_input(INPUT);
        assert_eq!(solve2(&parsed, 6), Some((6, 1)));
    }
}
