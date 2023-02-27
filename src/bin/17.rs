use num::Integer;
use ndarray::prelude::*;

fn check_rock_blocked(
    cave: &Array2<i8>,
    rock_format: &Vec<(isize, isize)>,
    rock_offset: (isize, isize),
) -> bool {
    for pos in rock_format {
        let new_pos = (pos.0 + rock_offset.0, pos.1 + rock_offset.1);
        // check if position is in tunnel, between 0 and 6 and above floor
        if new_pos.0 < 0 || new_pos.0 > 6 || new_pos.1 < 0 {
            return true;
        }
        // if already rock in cave, gets blocked
        if cave[[new_pos.1 as usize, new_pos.0 as usize]] == 1 {
            return true;
        }
    }

    false
}

fn simulate_falling_rocks(input: &str, nr_rocks: usize) -> i32 {
    let jet_directions: Vec<char> = input.trim().chars().collect();

    let rocks_formations: Vec<Vec<(isize, isize)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let lcm = jet_directions.len().lcm(&rocks_formations.len());
    let mut cave = Array2::<i8>::zeros((64, 7));
    let mut jet_count = 0;
    //start at floor level
    let mut heighest_rock_position: isize = 0;
    // for 2022 rocks to fall
    for i in 0..nr_rocks {
        // make sure cave does not get to high
        if heighest_rock_position > 10_isize.pow(6) {
            // find heighest row that is totally filled
            let k = cave
                .axis_iter(Axis(0))
                .rev()
                .enumerate()
                .find(|(_, row)| !row.iter().any(|a| *a == 0))
                .unwrap()
                .0;
            let mut new_cave = Array2::<i8>::zeros(cave.raw_dim());
            heighest_rock_position -= k as isize;
            new_cave.slice_mut(s![0..(cave.nrows()-k), ..]).assign(&cave.slice(s![k..cave.nrows(), ..]));

        }
        
        let current_rock = &rocks_formations[i % 5];
        let mut rock_offset: (isize, isize) = (2, heighest_rock_position + 3);
        // if rock is heigher, extend cave
        if rock_offset.1 + 5 > cave.nrows().try_into().unwrap() {
            // double cave size
            let new_size = (cave.nrows() * 2, cave.ncols());
            let mut new_cave = Array2::<i8>::zeros(new_size);
            new_cave.slice_mut(s![0..cave.nrows(), ..]).assign(&cave);
            cave = new_cave;
        }

        loop {
            // move rock with yet and check if stopped
            let direction: isize = match jet_directions[jet_count % jet_directions.len()] {
                '>' => 1,
                '<' => -1,
                _ => {
                    println!(
                        "error!, {}",
                        jet_directions[jet_count % jet_directions.len()]
                    );
                    0
                }
            };
            jet_count += 1;
            rock_offset.0 += direction;
            if check_rock_blocked(&cave, current_rock, rock_offset) {
                // reset offset
                rock_offset.0 -= direction;
            }

            // move rock down and check if blocked
            rock_offset.1 -= 1;
            if check_rock_blocked(&cave, current_rock, rock_offset) {
                // reset offset
                rock_offset.1 += 1;
                // but rock down and end loop
                for pos in current_rock {
                    let new_pos = (pos.0 + rock_offset.0, pos.1 + rock_offset.1);
                    cave[[new_pos.1 as usize, new_pos.0 as usize]] = 1;

                    heighest_rock_position = heighest_rock_position.max(new_pos.1 + 1);
                }
                break;
            }
        }
    }

    heighest_rock_position.try_into().unwrap()
}

pub fn part_one(input: &str) -> Option<i32> {
    let heighest_rock_position = simulate_falling_rocks(input, 2022);
    Some(heighest_rock_position as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let heighest_rock_position = simulate_falling_rocks(input, 10_usize.pow(7));
    dbg!(Some(heighest_rock_position as i32));

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
