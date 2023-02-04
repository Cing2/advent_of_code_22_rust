use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos_h = (0, 0);
    let mut pos_l = (0, 0);
    let mut positions = HashSet::new();
    positions.insert(pos_l);

    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();
        let num = right.parse::<i32>().unwrap_or_default();
        for _ in 0..num {
            match left {
                "R" => {
                    pos_h.0 += 1;
                }
                "L" => {
                    pos_h.0 -= 1;
                }
                "U" => {
                    pos_h.1 += 1;
                }
                "D" => {
                    pos_h.1 -= 1;
                }
                _ => {}
            }
            // update tail position
            let abs_dis = (i32::abs(pos_h.0 - pos_l.0), i32::abs(pos_h.1 - pos_l.1));
            if pos_h == pos_l {
                //  do not do anything
            } else if abs_dis.0 > 1 && abs_dis.1 == 0 {
                // if 2 away in any direction
                pos_l.0 += (pos_h.0 - pos_l.0).clamp(-1, 1);
            } else if abs_dis.1 > 1 && abs_dis.0 == 0 {
                pos_l.1 += (pos_h.1 - pos_l.1).clamp(-1, 1);
            } else if (abs_dis.0 > 1 && abs_dis.1 == 1) || (abs_dis.1 > 1 && abs_dis.0 == 1) {
                // move diagonal
                pos_l.0 += (pos_h.0 - pos_l.0).clamp(-1, 1);
                pos_l.1 += (pos_h.1 - pos_l.1).clamp(-1, 1);
            }
            // println!("{pos_h:?}-{pos_l:?}");
            positions.insert(pos_l);
        }
    }

    Some(positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut knots = [(0, 0); 10];
    let mut positions = HashSet::new();
    positions.insert(knots[9]);

    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();
        let num = right.parse::<i32>().unwrap_or_default();
        for _ in 0..num {
            match left {
                "R" => {
                    knots[0].0 += 1;
                }
                "L" => {
                    knots[0].0 -= 1;
                }
                "U" => {
                    knots[0].1 += 1;
                }
                "D" => {
                    knots[0].1 -= 1;
                }
                _ => {}
            }
            // update tail positions
            for i in 1..10 {
                // let &mut pos_h = knots[i-1];
                // let &mut pos_l = knots[i];
                let abs_dis = (
                    i32::abs(knots[i - 1].0 - knots[i].0),
                    i32::abs(knots[i - 1].1 - knots[i].1),
                );
                if knots[i - 1] == knots[i] {
                    //  do not do anything
                } else if abs_dis.0 > 1 && abs_dis.1 == 0 {
                    // if 2 away in any direction
                    knots[i].0 += (knots[i - 1].0 - knots[i].0).clamp(-1, 1);
                } else if abs_dis.1 > 1 && abs_dis.0 == 0 {
                    knots[i].1 += (knots[i - 1].1 - knots[i].1).clamp(-1, 1);
                } else if (abs_dis.0 > 1 && abs_dis.1 >= 1) || (abs_dis.1 > 1 && abs_dis.0 >= 1) {
                    // move diagonal
                    knots[i].0 += (knots[i - 1].0 - knots[i].0).clamp(-1, 1);
                    knots[i].1 += (knots[i - 1].1 - knots[i].1).clamp(-1, 1);
                }
            }

            // println!("{knots:?}");
            positions.insert(knots[9]);
        }
    }

    Some(positions.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
