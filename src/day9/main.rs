use std::collections::VecDeque;

use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::{sequence::pair, IResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileData {
    id: u64,
    size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Record {
    File(FileData),
    Free(u64),
}

fn parse_input(mut input: &str) -> VecDeque<Record> {
    let mut filesystem = VecDeque::new();

    let mut id = 0;
    while !input.is_empty() {
        let (rem, (file_size, free_size)) = parse_record(input).unwrap();
        input = rem;

        assert!(file_size > 0);

        filesystem.push_back(Record::File(FileData {
            id,
            size: file_size,
        }));
        id += 1;

        if free_size > 0 {
            filesystem.push_back(Record::Free(free_size));
        }
    }

    filesystem
}

fn parse_record(input: &str) -> IResult<&str, (u64, u64)> {
    alt((pair(parse_num, parse_num), map(parse_num, |v| (v, 0))))(input)
}

fn parse_num(input: &str) -> IResult<&str, u64> {
    map(take(1_usize), |v: &str| v.parse().unwrap())(input)
}

mod part1 {
    use super::*;

    fn take_from_end(filesystem: &mut VecDeque<Record>) -> Option<u64> {
        let mut last = filesystem.pop_back()?;

        while let Record::Free(_) = last {
            last = filesystem.pop_back().unwrap();
        }

        let Record::File(FileData { id, mut size }) = last else {
            panic!()
        };

        size -= 1;

        if size > 0 {
            filesystem.push_back(Record::File(FileData { id, size }));
        }

        Some(id)
    }

    pub fn solve1(mut input: VecDeque<Record>) -> u64 {
        let mut checksum = 0;
        let mut position = 0;

        while let Some(record) = input.pop_front() {
            match record {
                Record::File(FileData { id, size }) => {
                    for i in 0..size {
                        checksum += id * (position + i);
                    }
                    position += size;
                }
                Record::Free(mut size) => {
                    size -= 1;
                    if let Some(id) = take_from_end(&mut input) {
                        checksum += position * id;
                        position += 1;
                        if size > 0 {
                            input.push_front(Record::Free(size));
                        }
                    }
                }
            }
        }
        checksum
    }
}

mod part2 {
    use super::*;

    fn find_file_that_fits(free_space: u64, filesystem: &mut VecDeque<Record>) -> Option<FileData> {
        let mut found_idx = None;
        for (i, record) in filesystem.iter().enumerate().rev() {
            match record {
                Record::File(file_data) => {
                    if file_data.size <= free_space {
                        found_idx = Some(i);
                        break;
                    }
                }
                Record::Free(_) => (),
            }
        }

        if let Some(found_idx) = found_idx {
            let record = filesystem.remove(found_idx).unwrap();
            let Record::File(record) = record else {
                panic!();
            };
            filesystem.insert(found_idx, Record::Free(record.size));

            return Some(record);
        }
        None
    }

    pub fn solve2(mut input: VecDeque<Record>) -> u64 {
        let mut checksum = 0;
        let mut position = 0;

        while let Some(record) = input.pop_front() {
            match record {
                Record::File(FileData { id, size }) => {
                    for i in 0..size {
                        checksum += id * (position + i);
                    }
                    position += size;
                }
                Record::Free(size) => {
                    if let Some(file) = find_file_that_fits(size, &mut input) {
                        let new_free = size - file.size;
                        if new_free > 0 {
                            input.push_front(Record::Free(new_free));
                        }
                        input.push_front(Record::File(file));
                    } else {
                        position += size;
                    }
                }
            }
        }
        checksum
    }
}

fn main() {
    let parsed = parse_input(include_str!("input.txt"));

    println!("Part 1: {}", part1::solve1(parsed.clone()));
    println!("Part 2: {}", part2::solve2(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn parse_test() {
        let parsed = parse_input(INPUT);
        assert_eq!(parsed[0], Record::File(FileData { id: 0, size: 2 }));
        assert_eq!(parsed[1], Record::Free(3));
        assert_eq!(parsed[17], Record::File(FileData { id: 9, size: 2 }));
    }

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT);
        assert_eq!(part1::solve1(parsed), 1928);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT);
        assert_eq!(part2::solve2(parsed), 2858);
    }
}
