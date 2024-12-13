#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ClawMachine {
    button_a_x: u64,
    button_a_y: u64,
    button_b_x: u64,
    button_b_y: u64,
    prize_x: u64,
    prize_y: u64,
}

mod parser {
    use super::*;
    use nom::bytes::complete::take;
    use nom::character::complete::u64 as parse_u64;
    use nom::multi::many1;
    use nom::sequence::terminated;
    use nom::{
        bytes::complete::tag,
        character::complete::newline,
        combinator::map,
        multi::separated_list1,
        sequence::{preceded, separated_pair, tuple},
        IResult,
    };

    pub fn parse_input(input: &str) -> Vec<ClawMachine> {
        let parsed: IResult<&str, Vec<ClawMachine>> =
            separated_list1(many1(newline), parse_machine)(input);
        let parsed = parsed.unwrap();
        assert!(parsed.0.is_empty());

        parsed.1
    }

    fn parse_machine(input: &str) -> IResult<&str, ClawMachine> {
        map(
            tuple((parse_button, parse_button, parse_prize)),
            |(button_a, button_b, prize)| ClawMachine {
                button_a_x: button_a.0,
                button_a_y: button_a.1,
                button_b_x: button_b.0,
                button_b_y: button_b.1,
                prize_x: prize.0,
                prize_y: prize.1,
            },
        )(input)
    }

    /// Button A: X+94, Y+34
    fn parse_button(input: &str) -> IResult<&str, (u64, u64)> {
        terminated(
            preceded(
                tuple((tag("Button "), take(3_usize))),
                separated_pair(
                    preceded(tag("X+"), parse_u64),
                    tag(", "),
                    preceded(tag("Y+"), parse_u64),
                ),
            ),
            newline,
        )(input)
    }

    /// Prize: X=8400, Y=5400
    fn parse_prize(input: &str) -> IResult<&str, (u64, u64)> {
        preceded(
            tag("Prize: "),
            separated_pair(
                preceded(tag("X="), parse_u64),
                tag(", "),
                preceded(tag("Y="), parse_u64),
            ),
        )(input)
    }
}

fn solve1(input: &[ClawMachine]) -> u64 {
    let mut total = 0;

    for machine in input {
        let mut best_score = u64::MAX;
        for a in 0..100 {
            for b in 0..100 {
                if machine.button_a_x * a + machine.button_b_x * b == machine.prize_x
                    && machine.button_a_y * a + machine.button_b_y * b == machine.prize_y
                {
                    let num_tokens = 3 * a + b;
                    if num_tokens < best_score {
                        best_score = num_tokens;
                    }
                }
            }
        }
        if best_score != u64::MAX {
            total += best_score;
        }
    }

    total
}

fn solve2(input: &[ClawMachine]) -> i64 {
    let mut total = 0;
    for machine in input {
        let a1 = machine.button_a_x as i64;
        let a2 = machine.button_a_y as i64;
        let b1 = machine.button_b_x as i64;
        let b2 = machine.button_b_y as i64;
        let c1 = machine.prize_x as i64 + 10000000000000;
        let c2 = machine.prize_y as i64 + 10000000000000;

        let a_top = c1 * b2 - b1 * c2;
        let a_bottom = a1 * b2 - b1 * a2;

        let b_top = a1 * c2 - c1 * a2;
        let b_bottom = a1 * b2 - b1 * a2;

        let a_presses = if a_top % a_bottom == 0 {
            Some(a_top / a_bottom)
        } else {
            None
        };

        let b_presses = if b_top % b_bottom == 0 {
            Some(b_top / b_bottom)
        } else {
            None
        };

        if let (Some(a_presses), Some(b_presses)) = (a_presses, b_presses) {
            total += a_presses * 3 + b_presses;
        }
    }

    total
}

fn main() {
    let parsed = parser::parse_input(include_str!("input.txt"));

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn parse_test() {
        let parsed = parser::parse_input(INPUT);
        assert_eq!(
            parsed[0],
            ClawMachine {
                button_a_x: 94,
                button_a_y: 34,
                button_b_x: 22,
                button_b_y: 67,
                prize_x: 8400,
                prize_y: 5400
            }
        );
    }

    #[test]
    fn test1() {
        let parsed = parser::parse_input(INPUT);
        assert_eq!(solve1(&parsed), 480);
    }
}
