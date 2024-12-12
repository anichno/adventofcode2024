use std::collections::HashSet;

struct Farm {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    plots: Vec<Vec<char>>,
}

fn parse_input(input: &str) -> Farm {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut plots = Vec::new();

    for (y, line) in input.lines().enumerate() {
        max_y = y;
        if max_x == 0 {
            max_x = line.chars().count() - 1;
        }
        plots.push(line.chars().collect());
    }

    Farm {
        min_x: 0,
        max_x,
        min_y: 0,
        max_y,
        plots,
    }
}

mod part1 {
    use super::*;

    fn find_all_in_plot(
        x: usize,
        y: usize,
        in_plot: &mut HashSet<(usize, usize)>,
        perimeter_size: &mut usize,
        farm: &Farm,
    ) {
        in_plot.insert((x, y));
        let cur_plant = farm.plots[y][x];

        for diff in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let Some(new_x) = x.checked_add_signed(diff.0) {
                if let Some(new_y) = y.checked_add_signed(diff.1) {
                    if new_x <= farm.max_x
                        && new_y <= farm.max_y
                        && farm.plots[new_y][new_x] == cur_plant
                    {
                        if !in_plot.contains(&(new_x, new_y)) {
                            find_all_in_plot(new_x, new_y, in_plot, perimeter_size, farm);
                        }
                    } else {
                        *perimeter_size += 1;
                    }
                } else {
                    *perimeter_size += 1;
                }
            } else {
                *perimeter_size += 1;
            }
        }
    }

    pub fn solve1(input: &Farm) -> usize {
        let mut total = 0;
        let mut consumed_plots = HashSet::new();
        for (y, row) in input.plots.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if !consumed_plots.contains(&(x, y)) {
                    let mut new_consumed = HashSet::new();
                    let mut perimeter = 0;
                    find_all_in_plot(x, y, &mut new_consumed, &mut perimeter, input);
                    total += new_consumed.len() * perimeter;
                    consumed_plots.extend(new_consumed);
                }
            }
        }
        total
    }
}

mod part2 {
    use super::*;

    fn find_all_in_plot(
        x: usize,
        y: usize,
        in_plot: &mut HashSet<(usize, usize)>,
        perimeter_plots: &mut HashSet<(i64, i64)>,
        farm: &Farm,
    ) {
        in_plot.insert((x, y));
        let cur_plant = farm.plots[y][x];

        for diff in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let Some(new_x) = x.checked_add_signed(diff.0) {
                if let Some(new_y) = y.checked_add_signed(diff.1) {
                    if new_x <= farm.max_x
                        && new_y <= farm.max_y
                        && farm.plots[new_y][new_x] == cur_plant
                    {
                        if !in_plot.contains(&(new_x, new_y)) {
                            find_all_in_plot(new_x, new_y, in_plot, perimeter_plots, farm);
                        }
                    } else {
                        perimeter_plots.insert((new_x as i64, new_y as i64));
                    }
                } else {
                    perimeter_plots.insert((new_x as i64, y as i64 - 1));
                }
            } else {
                perimeter_plots.insert((x as i64 - 1, y as i64));
            }
        }
    }

    fn find_convex_corners(plant_plots: &HashSet<(usize, usize)>, farm: &Farm) -> usize {
        let mut corners = 0;

        for (x, y) in plant_plots {
            // look for corners
            for corner_pair in [
                ((-1, 0), (0, -1), (-1, -1)), // left, up
                ((0, -1), (1, 0), (1, -1)),   // up, right
                ((1, 0), (0, 1), (1, 1)),     // right, down
                ((0, 1), (-1, 0), (-1, 1)),   // down, left
            ] {
                let (side1, side2, corner) = corner_pair;
                let side1_same_plant = if let Some(s1_x) = x.checked_add_signed(side1.0) {
                    if let Some(s1_y) = y.checked_add_signed(side1.1) {
                        s1_x <= farm.max_x
                            && s1_y <= farm.max_y
                            && plant_plots.contains(&(s1_x, s1_y))
                    } else {
                        false
                    }
                } else {
                    false
                };

                let side2_same_plant = if let Some(s2_x) = x.checked_add_signed(side2.0) {
                    if let Some(s2_y) = y.checked_add_signed(side2.1) {
                        s2_x <= farm.max_x
                            && s2_y <= farm.max_y
                            && plant_plots.contains(&(s2_x, s2_y))
                    } else {
                        false
                    }
                } else {
                    false
                };

                let corner_same_plant = if let Some(c_x) = x.checked_add_signed(corner.0) {
                    if let Some(c_y) = y.checked_add_signed(corner.1) {
                        c_x <= farm.max_x && c_y <= farm.max_y && plant_plots.contains(&(c_x, c_y))
                    } else {
                        false
                    }
                } else {
                    false
                };

                if !side1_same_plant && !side2_same_plant && !corner_same_plant {
                    corners += 1;
                }
            }
        }

        corners
    }

    fn find_concave_corners(
        plant_plots: &HashSet<(usize, usize)>,
        perimeter_plots: &HashSet<(i64, i64)>,
        farm: &Farm,
    ) -> usize {
        let mut corners = 0;

        for (x, y) in perimeter_plots {
            // look for corners
            for corner_pair in [
                ((-1, 0), (0, -1)), // left, up
                ((0, -1), (1, 0)),  // up, right
                ((1, 0), (0, 1)),   // right, down
                ((0, 1), (-1, 0)),  // down, left
            ] {
                let (side1, side2) = corner_pair;

                let s1_x = *x + side1.0;
                let s1_y = *y + side1.1;
                let side1_same_plant = s1_x >= farm.min_x as i64
                    && s1_x <= farm.max_x as i64
                    && s1_y >= farm.min_y as i64
                    && s1_y <= farm.max_y as i64
                    && plant_plots.contains(&(s1_x as usize, s1_y as usize));

                let s2_x = *x + side2.0;
                let s2_y = *y + side2.1;
                let side2_same_plant = s2_x >= farm.min_x as i64
                    && s2_x <= farm.max_x as i64
                    && s2_y >= farm.min_y as i64
                    && s2_y <= farm.max_y as i64
                    && plant_plots.contains(&(s2_x as usize, s2_y as usize));

                if side1_same_plant && side2_same_plant {
                    corners += 1;
                }
            }
        }

        corners
    }

    pub fn solve2(input: &Farm) -> usize {
        let mut total = 0;
        let mut consumed_plots = HashSet::new();
        for (y, row) in input.plots.iter().enumerate() {
            for (x, _plant) in row.iter().enumerate() {
                if !consumed_plots.contains(&(x, y)) {
                    let mut new_consumed = HashSet::new();
                    let mut perimeter_plots = HashSet::new();
                    find_all_in_plot(x, y, &mut new_consumed, &mut perimeter_plots, input);
                    let convex_corners = find_convex_corners(&new_consumed, input);
                    let concave_corners =
                        find_concave_corners(&new_consumed, &perimeter_plots, input);

                    total += new_consumed.len() * (convex_corners + concave_corners);
                    consumed_plots.extend(new_consumed);
                }
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

    const INPUT1: &str = include_str!("test1.txt");
    const INPUT2: &str = include_str!("test2.txt");
    const INPUT3: &str = include_str!("test3.txt");
    const INPUT4: &str = include_str!("test4.txt");
    const INPUT5: &str = include_str!("test5.txt");

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT1);
        assert_eq!(part1::solve1(&parsed), 140);

        let parsed = parse_input(INPUT2);
        assert_eq!(part1::solve1(&parsed), 772);

        let parsed = parse_input(INPUT3);
        assert_eq!(part1::solve1(&parsed), 1930);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT1);
        assert_eq!(part2::solve2(&parsed), 80);

        let parsed = parse_input(INPUT2);
        assert_eq!(part2::solve2(&parsed), 436);

        let parsed = parse_input(INPUT3);
        assert_eq!(part2::solve2(&parsed), 1206);

        let parsed = parse_input(INPUT4);
        assert_eq!(part2::solve2(&parsed), 236);

        let parsed = parse_input(INPUT5);
        assert_eq!(part2::solve2(&parsed), 368);
    }
}
