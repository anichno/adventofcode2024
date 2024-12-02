use nom::bytes::complete::tag;
use nom::character::complete::i64 as parse_i64;
use nom::multi::separated_list1;
use nom::IResult;

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(tag("\n"), parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(" "), parse_i64)(input)
}

mod part1 {
    pub fn is_safe(input: &[i64]) -> bool {
        let increasing = input[1] > input[0];
        for pair in input.windows(2) {
            let (left, right) = (pair[0], pair[1]);
            if !(increasing && left < right && right - left >= 1 && right - left <= 3
                || !increasing && left > right && left - right >= 1 && left - right <= 3)
            {
                return false;
            }
        }

        true
    }

    pub fn solve1(input: &[Vec<i64>]) -> i64 {
        input.iter().filter(|i| is_safe(i)).count() as i64
    }
}

mod part2 {
    use super::*;

    pub fn is_safe(input: &[i64]) -> bool {
        if part1::is_safe(input) {
            return true;
        } else {
            for idx_removed in 0..input.len() {
                let new_input: Vec<i64> = input
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != idx_removed)
                    .map(|(_, v)| *v)
                    .collect();
                if part1::is_safe(&new_input) {
                    return true;
                }
            }
        }

        false
    }

    pub fn solve2(input: &[Vec<i64>]) -> i64 {
        input.iter().filter(|i| is_safe(i)).count() as i64
    }
}

fn main() {
    let parsed = parse_input(include_str!("input.txt")).expect("Failed to parse input");
    assert!(parsed.0.is_empty());
    let parsed = parsed.1;

    println!("Part 1: {}", part1::solve1(&parsed));
    println!("Part 2: {}", part2::solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn parse_test() {
        let parsed = parse_input(INPUT).unwrap().1;
        assert_eq!(parsed[0], &[7, 6, 4, 2, 1]);
        assert_eq!(parsed[1], &[1, 2, 7, 8, 9]);
    }

    #[test]
    fn test1_solve1() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(part1::is_safe(&parsed.1[0]), true);
        assert_eq!(part1::is_safe(&parsed.1[1]), false);
    }

    #[test]
    fn test2_solve1() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(part1::solve1(&parsed.1), 2);
    }

    #[test]
    fn test1_solve2() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(part2::is_safe(&parsed.1[0]), true);
        assert_eq!(part2::is_safe(&parsed.1[1]), false);
        assert_eq!(part2::is_safe(&parsed.1[3]), true);
    }

    #[test]
    fn test2_solve2() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(part2::solve2(&parsed.1), 4);
    }
}
