use itertools::Itertools;

fn check_containment(line: &str) -> u32 {
    let output: Option<bool> = line
        .split(',')
        .map(|elf| {
            elf.split('-')
                .map(|num| num.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>()
        })
        .tuples()
        .map(|(e1, e2)| (e1[0] <= e2[0] && e1[1] >= e2[1]) || (e1[0] >= e2[0] && e1[1] <= e2[1]))
        .reduce(|a, b| a & b);

    output.unwrap_or(false) as u32
}

fn check_overlap(line: &str) -> u32 {
    let output: Option<bool> = line
        .split(',')
        .map(|elf| {
            elf.split('-')
                .map(|num| num.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>()
        })
        .tuples()
        .map(|(e1, e2)| e1[1] >= e2[0] && e1[0] <=e2[1])
        .reduce(|a, b| a & b);
    output.unwrap_or(false) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| check_containment(l)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| check_overlap(l)).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
