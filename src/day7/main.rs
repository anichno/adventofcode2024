use nom::bytes::complete::tag;
use nom::character::complete::{i64 as parse_i64, newline, space1};
use nom::multi::separated_list1;
use nom::{sequence::separated_pair, IResult};

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    let parsed = separated_list1(newline, parse_line)(input).unwrap();
    assert!(parsed.0.is_empty());
    parsed.1
}

fn parse_line(input: &str) -> IResult<&str, (i64, Vec<i64>)> {
    separated_pair(parse_i64, tag(": "), separated_list1(space1, parse_i64))(input)
}

mod part1 {
    fn is_solvable(result: i64, rem_nums: &[i64], value: i64) -> bool {
        if rem_nums.is_empty() {
            value == result
        } else {
            let next_val = rem_nums[0];
            // add
            let new_val = value + next_val;
            if is_solvable(result, &rem_nums[1..], new_val) {
                return true;
            }

            // mul
            let new_val = value * next_val;
            is_solvable(result, &rem_nums[1..], new_val)
        }
    }

    pub fn solve1(input: &[(i64, Vec<i64>)]) -> i64 {
        let mut total = 0;
        for (result, nums) in input {
            if is_solvable(*result, &nums[1..], nums[0]) {
                total += *result;
            }
        }

        total
    }
}

mod part2 {
    fn is_solvable(result: i64, rem_nums: &[i64], value: i64) -> bool {
        if rem_nums.is_empty() {
            value == result
        } else {
            let next_val = rem_nums[0];
            // add
            let new_val = value + next_val;
            if is_solvable(result, &rem_nums[1..], new_val) {
                return true;
            }

            // mul
            let new_val = value * next_val;
            if is_solvable(result, &rem_nums[1..], new_val) {
                return true;
            }

            // concat
            let new_val = (value.to_string() + &next_val.to_string()).parse().unwrap();
            is_solvable(result, &rem_nums[1..], new_val)
        }
    }

    pub fn solve2(input: &[(i64, Vec<i64>)]) -> i64 {
        let mut total = 0;
        for (result, nums) in input {
            if is_solvable(*result, &nums[1..], nums[0]) {
                total += *result;
            }
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

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn parse_test() {
        let parsed = parse_input(INPUT);
        assert_eq!(parsed[0].0, 190);
        assert_eq!(parsed[0].1, &[10, 19]);
        assert_eq!(parsed[7].0, 21037);
        assert_eq!(parsed[7].1, &[9, 7, 18, 13]);
    }

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT);
        assert_eq!(part1::solve1(&parsed), 3749);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT);
        assert_eq!(part2::solve2(&parsed), 11387);
    }
}
