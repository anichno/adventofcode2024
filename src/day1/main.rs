use nom::bytes::complete::tag;
use nom::character::complete::{i64 as parse_i64, multispace1};
use nom::multi::separated_list0;
use nom::{sequence::separated_pair, IResult};

fn parse_input(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    separated_list0(tag("\n"), parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(parse_i64, multispace1, parse_i64)(input)
}

fn solve1(input: &[(i64, i64)]) -> i64 {
    let mut left: Vec<i64> = input.iter().map(|(l, _)| *l).collect();
    let mut right: Vec<i64> = input.iter().map(|(_, r)| *r).collect();
    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn solve2(input: &[(i64, i64)]) -> i64 {
    let mut tot = 0;

    for (left, _) in input {
        tot += left * input.iter().filter(|(_, r)| *left == *r).count() as i64;
    }

    tot
}

fn main() {
    let parsed = parse_input(include_str!("input.txt")).expect("Failed to parse input");
    assert!(parsed.0.is_empty());
    let parsed = parsed.1;

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn parse_test() {
        let parsed = parse_input(INPUT).unwrap().1;
        assert_eq!(parsed[0], (3, 4));
        assert_eq!(parsed[1], (4, 3));
        assert_eq!(parsed[2], (2, 5));
        assert_eq!(parsed[3], (1, 3));
        assert_eq!(parsed[4], (3, 9));
        assert_eq!(parsed[5], (3, 3));
    }

    #[test]
    fn parse_line_test() {
        let parsed = parse_line("3   4").unwrap();
        assert_eq!(parsed.1, (3, 4));
    }

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(solve1(&parsed.1), 11);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(solve2(&parsed.1), 31);
    }
}
