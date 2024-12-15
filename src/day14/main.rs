#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    position_x: i64,
    position_y: i64,
    velocity_x: i64,
    velocity_y: i64,
}

mod parser {
    use nom::character::complete::{i64 as parse_i64, newline, space1};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;
    use nom::{bytes::complete::tag, sequence::preceded, IResult};

    use super::Robot;

    // p=0,4 v=3,-3
    pub fn parse_input(input: &str) -> Vec<Robot> {
        let parsed = separated_list1(
            newline,
            map(
                separated_pair(parse_position, space1, parse_velocity),
                |(p, v)| Robot {
                    position_x: p.0,
                    position_y: p.1,
                    velocity_x: v.0,
                    velocity_y: v.1,
                },
            ),
        )(input)
        .unwrap();
        assert!(parsed.0.is_empty());

        parsed.1
    }

    /// p=0,4
    fn parse_position(input: &str) -> IResult<&str, (i64, i64)> {
        preceded(tag("p="), separated_pair(parse_i64, tag(","), parse_i64))(input)
    }

    /// v=3,-3
    fn parse_velocity(input: &str) -> IResult<&str, (i64, i64)> {
        preceded(tag("v="), separated_pair(parse_i64, tag(","), parse_i64))(input)
    }
}

fn solve1(input: &[Robot], grid_x: i64, grid_y: i64) -> i64 {
    let mut quadrants = [0; 4];
    for robot in input {
        let mut robot = *robot;
        for _seconds in 0..100 {
            robot.position_x += robot.velocity_x;
            robot.position_y += robot.velocity_y;

            if robot.position_x < 0 {
                robot.position_x += grid_x;
            } else if robot.position_x >= grid_x {
                robot.position_x -= grid_x;
            }

            if robot.position_y < 0 {
                robot.position_y += grid_y;
            } else if robot.position_y >= grid_y {
                robot.position_y -= grid_y;
            }
        }

        let (x, y) = (robot.position_x, robot.position_y);
        if x < grid_x / 2 && y < grid_y / 2 {
            quadrants[0] += 1;
        } else if x > grid_x / 2 && y < grid_y / 2 {
            quadrants[1] += 1;
        } else if x < grid_x / 2 && y > grid_y / 2 {
            quadrants[2] += 1;
        } else if x > grid_x / 2 && y > grid_y / 2 {
            quadrants[3] += 1;
        }
    }

    quadrants.iter().fold(1, |a, r| a * *r)
}

fn solve2(input: &[Robot]) -> i64 {
    const GRID_X: i64 = 101;
    const GRID_Y: i64 = 103;

    let mut best_score = i64::MAX;
    let mut best_seconds = 0;
    let mut robots = input.to_owned();
    for seconds in 0..100000 {
        let mut quadrants = [0; 4];

        let mut frame = [[' '; GRID_X as usize]; GRID_Y as usize];
        for robot in robots.iter_mut() {
            robot.position_x += robot.velocity_x;
            robot.position_y += robot.velocity_y;

            if robot.position_x < 0 {
                robot.position_x += GRID_X;
            } else if robot.position_x >= GRID_X {
                robot.position_x -= GRID_X;
            }

            if robot.position_y < 0 {
                robot.position_y += GRID_Y;
            } else if robot.position_y >= GRID_Y {
                robot.position_y -= GRID_Y;
            }

            let (x, y) = (robot.position_x, robot.position_y);
            if x < GRID_X / 2 && y < GRID_Y / 2 {
                quadrants[0] += 1;
            } else if x > GRID_X / 2 && y < GRID_Y / 2 {
                quadrants[1] += 1;
            } else if x < GRID_X / 2 && y > GRID_Y / 2 {
                quadrants[2] += 1;
            } else if x > GRID_X / 2 && y > GRID_Y / 2 {
                quadrants[3] += 1;
            }

            frame[robot.position_y as usize][robot.position_x as usize] = '#';
        }

        let score = quadrants.iter().fold(1, |a, r| a * *r);
        if score < best_score {
            best_score = score;
            best_seconds = seconds + 1;
            println!("seconds: {}, score: {score}", seconds + 1);
            for row in frame {
                for col in row {
                    print!("{col}");
                }
                println!();
            }
            println!();
        }
    }

    best_seconds
}

fn main() {
    let parsed = parser::parse_input(include_str!("input.txt"));

    println!("Part 1: {}", solve1(&parsed, 101, 103));
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
            Robot {
                position_x: 0,
                position_y: 4,
                velocity_x: 3,
                velocity_y: -3
            }
        );

        assert_eq!(
            parsed[11],
            Robot {
                position_x: 9,
                position_y: 5,
                velocity_x: -3,
                velocity_y: -3
            }
        );
    }

    #[test]
    fn test1() {
        let parsed = parser::parse_input(INPUT);
        assert_eq!(solve1(&parsed, 11, 7), 12);
    }
}
