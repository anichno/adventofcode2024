use std::collections::{HashMap, HashSet};

use nom::character::complete::{char, i64 as parse_i64, multispace1, newline};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::{sequence::separated_pair, IResult};

type DependsOn = HashMap<i64, HashSet<i64>>;

fn parse_input(input: &str) -> IResult<&str, (DependsOn, Vec<Vec<i64>>)> {
    let (rem, depends) = parse_depends(input).unwrap();
    let mut depends_on: DependsOn = HashMap::new();
    for (k, v) in depends.into_iter().map(|(v, k)| (k, v)) {
        depends_on.entry(k).or_default().insert(v);
    }

    let (rem, updates) = preceded(multispace1, parse_updates)(rem).unwrap();

    Ok((rem, (depends_on, updates)))
}

fn parse_depends(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    separated_list1(newline, separated_pair(parse_i64, char('|'), parse_i64))(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(newline, separated_list1(char(','), parse_i64))(input)
}

fn update_in_order(input: &[i64], depends_on: &DependsOn) -> bool {
    for (i, page) in input.iter().enumerate() {
        if let Some(prec) = depends_on.get(page) {
            for p in prec {
                if input.iter().any(|v| *v == *p) && !input[..i].iter().any(|v| *v == *p) {
                    return false;
                }
            }
        }
    }
    true
}

fn fix_order(input: &[i64], depends_on: &DependsOn) -> Vec<i64> {
    let mut fixed: Vec<i64> = input.into();

    'outer: loop {
        for i in 0..fixed.len() {
            let val = fixed[i];
            if let Some(prec) = depends_on.get(&val) {
                for p in prec {
                    if let Some(idx) = fixed.iter().position(|v| v == p) {
                        if idx > i {
                            fixed.remove(idx);
                            fixed.insert(i, *p);
                            continue 'outer;
                        }
                    }
                }
            }
        }
        break;
    }

    fixed
}

fn solve1(updates: &[Vec<i64>], depends_on: &DependsOn) -> i64 {
    let mut total = 0;
    for update in updates {
        if update_in_order(update, depends_on) {
            total += update[update.len() / 2];
        }
    }
    total
}

fn solve2(updates: &[Vec<i64>], depends_on: &DependsOn) -> i64 {
    let mut total = 0;
    for update in updates {
        if !update_in_order(update, depends_on) {
            let dup_check: HashSet<&i64> = HashSet::from_iter(update);
            assert_eq!(update.len(), dup_check.len());
            let fixed = fix_order(update, depends_on);

            total += fixed[fixed.len() / 2];
        }
    }
    total
}

fn main() {
    let parsed = parse_input(include_str!("input.txt")).expect("Failed to parse input");
    assert!(parsed.0.is_empty());
    let parsed = parsed.1;

    println!("Part 1: {}", solve1(&parsed.1, &parsed.0));
    println!("Part 2: {}", solve2(&parsed.1, &parsed.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn parse_test() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        let (depends_on, updates) = parsed.1;

        assert!(depends_on.get(&53).unwrap().contains(&47));
        assert!(depends_on.get(&53).unwrap().contains(&75));
        assert!(depends_on.get(&53).unwrap().contains(&61));
        assert!(depends_on.get(&53).unwrap().contains(&97));

        assert!(depends_on.get(&13).unwrap().contains(&97));
        assert!(depends_on.get(&13).unwrap().contains(&61));
        assert!(depends_on.get(&13).unwrap().contains(&47));
        assert!(depends_on.get(&13).unwrap().contains(&75));
        assert!(depends_on.get(&13).unwrap().contains(&53));
        assert!(depends_on.get(&13).unwrap().contains(&29));

        assert_eq!(updates[0], &[75, 47, 61, 53, 29]);
        assert_eq!(updates[5], &[97, 13, 75, 29, 47])
    }

    #[test]
    fn test_update_in_order() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        let (depends_on, updates) = parsed.1;

        assert!(update_in_order(&updates[0], &depends_on));
        assert!(update_in_order(&updates[1], &depends_on));
        assert!(!update_in_order(&updates[3], &depends_on));
        assert!(!update_in_order(&updates[4], &depends_on));
    }

    #[test]
    fn test_solve1() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        let (depends_on, updates) = parsed.1;

        assert_eq!(solve1(&updates, &depends_on), 143);
    }

    #[test]
    fn test_solve2() {
        let parsed = parse_input(INPUT).unwrap();
        assert!(parsed.0.is_empty());
        let (depends_on, updates) = parsed.1;

        assert_eq!(solve2(&updates, &depends_on), 123);
    }
}
