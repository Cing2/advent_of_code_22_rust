use itertools::Itertools;
extern crate ndarray;

use ndarray::prelude::*;

fn parse_input(input: &str) -> (i32, Array2<i8>) {
    let pos: Vec<Vec<(i32, i32)>> = input
        .lines()
        .map(|l| {
            l.split("->")
                .filter_map(|ns| {
                    ns.split_once(',').map(|(a, b)| {
                        (
                            a.trim().parse::<i32>().unwrap(),
                            b.trim().parse::<i32>().unwrap(),
                        )
                    })
                })
                .collect()
        })
        .collect();

    let max_y = pos.iter().flatten().map(|a| a.1).max().unwrap();
    let start_x = 500 - max_y - 10;

    let size_array = Dim([(max_y + 3) as usize, (max_y * 2 + 20) as usize]);

    // make matrix
    let mut scan_matrix = Array2::<i8>::zeros(size_array);

    for row in pos {
        for i in 1..row.len() {
            let range1 = match row[i].1 > row[i - 1].1 {
                true => row[i - 1].1..(row[i].1 + 1),
                false => row[i].1..(row[i - 1].1 + 1),
            };
            let range2 = match row[i].0 > row[i - 1].0 {
                true => (row[i - 1].0 - start_x)..(row[i].0 - start_x + 1),
                false => (row[i].0 - start_x)..(row[i - 1].0 - start_x + 1),
            };

            scan_matrix.slice_mut(s![range1, range2]).fill(1);
        }
    }

    (start_x, scan_matrix)
}

fn simulate_sand(mut cave: Array2<i8>, min_x: i32) -> i32 {
    let spawn_position = (0_usize, 500 - min_x as usize);
    let mut sand_overflow = false;
    let mut sand_count = 0;

    // spawn sand
    while !sand_overflow {
        let mut cur_pos = spawn_position;
        // go lower
        loop {
            if cur_pos == spawn_position && cave[cur_pos] == 1{
                sand_overflow = true;
                break;
            }
            if cur_pos.0 + 1 == cave.nrows() {
                sand_overflow = true;
                break;
            }
            if cave[[cur_pos.0 + 1, cur_pos.1]] == 0 {
                cur_pos.0 += 1;
                continue;
            }
            if cur_pos.1 == 0 {
                // sand overflows to side
                sand_overflow = true;
                break;
            }
            if cave[[cur_pos.0 + 1, cur_pos.1 - 1]] == 0 {
                cur_pos.0 += 1;
                cur_pos.1 -= 1;
                continue;
            }
            if cur_pos.1 + 1 == cave.ncols() {
                // sand overflows to side
                sand_overflow = true;
                break;
            }
            if cave[[cur_pos.0 + 1, cur_pos.1 + 1]] == 0 {
                cur_pos.0 += 1;
                cur_pos.1 += 1;
                continue;
            }
            // not falling thus place sand
            cave[cur_pos] = 1;
            sand_count += 1;
            break;
        }
    }
    // dbg!(&cave);

    sand_count
}

pub fn part_one(input: &str) -> Option<i32> {
    let (min_x, cave) = parse_input(input);

    Some(simulate_sand(cave, min_x))
}

pub fn part_two(input: &str) -> Option<i32> {
    let (min_x, mut cave) = parse_input(input);

    cave.slice_mut(s![cave.nrows() - 1, ..]).fill(1);
    // dbg!(&cave);

    Some(simulate_sand(cave, min_x))
    // None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
