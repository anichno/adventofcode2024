use nom::character::complete::{newline, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(newline, many1(one_of("XMAS")))(input)
}

mod part1 {
    const VALID_WORD: &[char] = &['X', 'M', 'A', 'S'];

    fn check_next_letter(
        input: &[Vec<char>],
        letter_idx: usize,
        cur_x: usize,
        cur_y: usize,
        diff: (isize, isize),
    ) -> bool {
        if letter_idx < VALID_WORD.len() {
            let (Some(new_x), Some(new_y)) = (
                cur_x.checked_add_signed(diff.0),
                cur_y.checked_add_signed(diff.1),
            ) else {
                return false;
            };

            return new_x < input[0].len()
                && new_y < input.len()
                && input[new_y][new_x] == VALID_WORD[letter_idx]
                && check_next_letter(input, letter_idx + 1, new_x, new_y, diff);
        }

        true
    }

    pub fn solve1(input: &[Vec<char>]) -> i64 {
        let mut total = 0;
        for (y, row) in input.iter().enumerate() {
            for (x, letter) in row.iter().enumerate() {
                if *letter == VALID_WORD[0] {
                    for diff_x in -1..=1 {
                        for diff_y in -1..=1 {
                            if check_next_letter(input, 1, x, y, (diff_x, diff_y)) {
                                total += 1;
                            }
                        }
                    }
                }
            }
        }

        total
    }
}

pub fn solve2(input: &[Vec<char>]) -> i64 {
    let mut total = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            if *letter == 'A' {
                // Validate both diagonals are valid (not against edge of input)
                if x.checked_add_signed(-1).is_some()
                    && x + 1 < input[0].len()
                    && y.checked_add_signed(-1).is_some()
                    && y + 1 < input.len()
                {
                    let diag1 = (input[y - 1][x - 1], input[y + 1][x + 1]);
                    let diag2 = (input[y - 1][x + 1], input[y + 1][x - 1]);
                    if ((diag1.0 == 'M' && diag1.1 == 'S') || (diag1.0 == 'S' && diag1.1 == 'M'))
                        && ((diag2.0 == 'M' && diag2.1 == 'S')
                            || (diag2.0 == 'S' && diag2.1 == 'M'))
                    {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

fn main() {
    let parsed = parse_input(include_str!("input.txt")).expect("Failed to parse input");
    assert!(parsed.0.is_empty());

    println!("Part 1: {}", part1::solve1(&parsed.1));
    println!("Part 2: {}", solve2(&parsed.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn parse_test() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(
            parsed.1[0],
            &['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M']
        );
    }

    #[test]
    fn solve1_test() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(part1::solve1(&parsed.1), 18);
    }

    #[test]
    fn solve2_test() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        assert_eq!(solve2(&parsed.1), 9);
    }
}
