fn parse_input(input: &str) -> Vec<String> {
    input
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect()
}

mod part1 {
    fn update_stones(stones: Vec<String>) -> Vec<String> {
        let mut new_stones = Vec::new();
        for stone in stones {
            if stone == "0" {
                new_stones.push(String::from("1"));
            } else if stone.len() % 2 == 0 {
                let (left, right) = stone.split_at(stone.len() / 2);
                let mut left = left.trim_start_matches("0");
                if left.is_empty() {
                    left = "0";
                }
                let mut right = right.trim_start_matches("0");
                if right.is_empty() {
                    right = "0";
                }
                new_stones.push(left.to_string());
                new_stones.push(right.to_string());
            } else {
                let val: u64 = stone.parse().unwrap();
                new_stones.push((val * 2024).to_string());
            }
        }

        new_stones
    }

    pub fn solve1(input: &[String]) -> usize {
        let mut stones = input.to_owned();
        for _ in 0..25 {
            stones = update_stones(stones);
        }

        stones.len()
    }
}

mod part2 {
    use std::collections::HashMap;

    fn update_stone(
        stone: u64,
        max_depth: u64,
        cur_depth: u64,
        lookup: &mut HashMap<(u64, u64), u64>,
    ) -> u64 {
        if max_depth == cur_depth {
            return 1;
        }

        let new_stone1;
        let mut new_stone2 = None;

        if stone == 0 {
            new_stone1 = 1;
        } else {
            let stone_string = stone.to_string();
            if stone_string.len() % 2 == 0 {
                let (left, right) = stone_string.split_at(stone_string.len() / 2);
                let mut left = left.trim_start_matches("0");
                if left.is_empty() {
                    left = "0";
                }
                let mut right = right.trim_start_matches("0");
                if right.is_empty() {
                    right = "0";
                }
                new_stone1 = left.parse().unwrap();
                new_stone2 = Some(right.parse().unwrap());
            } else {
                new_stone1 = stone * 2024;
            }
        }

        let mut tot = 0;

        if let Some(known) = lookup.get(&(new_stone1, cur_depth)) {
            tot += *known;
        } else {
            let val = update_stone(new_stone1, max_depth, cur_depth + 1, lookup);
            lookup.insert((new_stone1, cur_depth), val);
            tot += val;
        }

        if let Some(new_stone2) = new_stone2 {
            if let Some(known) = lookup.get(&(new_stone2, cur_depth)) {
                tot += *known;
            } else {
                let val = update_stone(new_stone2, max_depth, cur_depth + 1, lookup);
                lookup.insert((new_stone2, cur_depth), val);
                tot += val;
            }
        }

        tot
    }

    pub fn solve2(input: &Vec<String>) -> u64 {
        let mut total = 0;

        let mut lookup = HashMap::new();
        for stone in input {
            total += update_stone(stone.parse().unwrap(), 75, 0, &mut lookup);
        }

        total
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

    const INPUT: &str = "125 17";

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT);
        assert_eq!(part1::solve1(&parsed), 55312);
    }
}
