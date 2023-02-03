pub fn part_one(input: &str) -> Option<u32> {
    let mut pos_h = (0, 0);
    let mut pos_l = (0, 0);
    pos_h-pos_l;
    for line in input.lines(){
        let (left, right) = line.split_once(' ').unwrap();
        let num = right.parse::<i32>().unwrap_or_default();
        for i in 0..num {
            match left {
                "R" => {
                    pos_h.0 += 1;
                }
                _ => {

                }
            }
            // update tail position
            let abs_dis = ((pos_h.0-pos_l.0).abs(), pos_h.1-pos_l.1)
            if pos_h == pos_l {
                //  do not do anything
            }else if pos_h.0 == pos_l.0 || pos_h.1 == pos_l.1{
                // if 2 away in any direction
            }
        }
     
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
