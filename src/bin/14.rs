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

    let min_max = pos
        .iter()
        .flatten()
        .map(|a| a.0)
        .minmax()
        .into_option()
        .unwrap();
    let max_y = pos.iter().flatten().map(|a| a.1).max().unwrap();

    dbg!(&min_max);
    let size_array = Dim([(max_y + 1) as usize, (min_max.1 - min_max.0 + 1) as usize]);
    // make matrix
    dbg!(&size_array);
    let mut scan_matrix = Array2::<i8>::zeros(size_array);

    for row in pos {
        for i in 1..row.len() {
            let range1 = match row[i].1 > row[i - 1].1 {
                true => row[i - 1].1..(row[i].1 + 1),
                false => row[i].1..(row[i - 1].1 + 1),
            };
            let range2 = match row[i].0 > row[i - 1].0 {
                true => (row[i - 1].0 - min_max.0)..(row[i].0 - min_max.0 + 1),
                false => (row[i].0 - min_max.0)..(row[i - 1].0 - min_max.0 + 1),
            };

            // dbg!(&row[i-1..i+1], &range1, &range2);
            scan_matrix.slice_mut(s![range1, range2]).fill(1);
            // dbg!(a);
        }
    }

    println!("{:?}", &scan_matrix);
    println!("{:?}", &scan_matrix[[9, 0]]);
    (min_max.0, scan_matrix)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (min_x, mut cave) = parse_input(input);

    let spawn_position = (0 as usize, 500 - min_x as usize);
    let mut sand_overflow = false;
    let mut sand_count = 0;

    // spawn sand
    while !sand_overflow {
        let mut cur_pos = spawn_position.clone();
        // go lower
        while true {
            if cur_pos.0 + 1 > cave.nrows() {
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
            if cur_pos.1 + 1 > cave.ncols() {
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
    dbg!(&cave);
    dbg!(&sand_count);

    Some(sand_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
