use nom::bytes::complete::tag;
use nom::character::complete::{char, i64 as parse_i64};
use nom::sequence::delimited;
use nom::{sequence::separated_pair, IResult};

fn parse_mul(input: &str) -> IResult<&str, (i64, i64)> {
    delimited(
        tag("mul("),
        separated_pair(parse_i64, char(','), parse_i64),
        char(')'),
    )(input)
}

mod part1 {
    use super::*;

    pub fn extract_muls(input: &str) -> Vec<(i64, i64)> {
        let mut results = Vec::new();
        let mut remaining = input;

        while !remaining.is_empty() {
            match parse_mul(remaining) {
                Ok((rem, val)) => {
                    remaining = rem;
                    results.push(val);
                }
                Err(_) => remaining = &remaining[1..],
            }
        }

        results
    }

    pub fn solve1(input: &str) -> i64 {
        extract_muls(input).into_iter().map(|(l, r)| l * r).sum()
    }
}

mod part2 {
    use nom::{branch::alt, combinator::map};

    use super::*;

    enum Statement {
        Do,
        Dont,
        Mul((i64, i64)),
    }

    fn parse_statement(input: &str) -> IResult<&str, Statement> {
        alt((
            map(tag("do()"), |_| Statement::Do),
            map(tag("don't()"), |_| Statement::Dont),
            map(parse_mul, |(l, r)| Statement::Mul((l, r))),
        ))(input)
    }

    pub fn extract_muls(input: &str) -> Vec<(i64, i64)> {
        let mut results = Vec::new();
        let mut remaining = input;
        let mut mul_enabled = true;

        while !remaining.is_empty() {
            match parse_statement(remaining) {
                Ok((rem, stmt)) => {
                    remaining = rem;
                    match stmt {
                        Statement::Do => mul_enabled = true,
                        Statement::Dont => mul_enabled = false,
                        Statement::Mul(val) => {
                            if mul_enabled {
                                results.push(val);
                            }
                        }
                    }
                }
                Err(_) => remaining = &remaining[1..],
            }
        }

        results
    }

    pub fn solve2(input: &str) -> i64 {
        extract_muls(input).into_iter().map(|(l, r)| l * r).sum()
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1::solve1(input));
    println!("Part 2: {}", part2::solve2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn parse_mul_test() {
        let parsed = parse_mul("mul(20,14)").unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(parsed.1 .0, 20);
        assert_eq!(parsed.1 .1, 14);
    }

    #[test]
    fn parse_test() {
        let parsed = part1::extract_muls(INPUT1);
        assert_eq!(parsed[0], (2, 4));
        assert_eq!(parsed[1], (5, 5));
        assert_eq!(parsed[2], (11, 8));
        assert_eq!(parsed[3], (8, 5));
    }

    #[test]
    fn solve1_test() {
        assert_eq!(part1::solve1(INPUT1), 161);
    }

    #[test]
    fn solve2_test() {
        assert_eq!(part2::solve2(INPUT2), 48);
    }
}
