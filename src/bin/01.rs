use itertools::Itertools;
use std::cmp::Reverse;

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let sums: Vec<u32> = parse_input(input);

    // println!("{}, {}", sums[0], sums[1]);
    sums.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let sums: Vec<u32> = parse_input(input);
    Some(sums.iter().sorted_by_key(|w| Reverse(*w)).take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
