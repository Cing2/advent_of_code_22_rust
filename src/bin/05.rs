use std::collections::VecDeque;

use itertools::enumerate;
use regex::Regex;

fn parse_crates(input: &str) -> Vec<VecDeque<char>> {
    let mut crate_lines: Vec<Vec<char>> = vec![];
    for l in input.lines() {
        if !l.contains("[") {
            break;
        };
        crate_lines.push(
            l.char_indices()
                .filter_map(|(i, c)| if i % 4 == 1 { Some(c) } else { None })
                .collect(),
        )
    }
    // println!("lines: {:?}", crate_lines);

    let mut crates: Vec<VecDeque<char>> = vec![VecDeque::new(); crate_lines[0].len()];
    for l in crate_lines {
        for (i, c) in enumerate(l) {
            if c != ' ' {
                crates[i].push_back(c);
            }
        }
    }
    println!("crates: {:?}", crates);

    return crates;
}

pub fn part_one(input: &str) -> Option<String> {
    let mut crates = parse_crates(input);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for a in re.captures_iter(input) {
        let (nr, from, to) = (
            a[1].parse::<usize>().unwrap(),
            a[2].parse::<usize>().unwrap() - 1,
            a[3].parse::<usize>().unwrap() - 1,
        );

        for _ in 0..(nr) {
            let acrate = crates[from].pop_front().unwrap();
            crates[to].push_front(acrate);
        }
    }
    // println!("{:?}", crates);
    let answer: Vec<char> = crates.iter().map(|c| c[0]).collect();

    Some(answer.into_iter().collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut crates = parse_crates(input);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for a in re.captures_iter(input) {
        let (nr, from, to) = (
            a[1].parse::<usize>().unwrap(),
            a[2].parse::<usize>().unwrap() - 1,
            a[3].parse::<usize>().unwrap() - 1,
        );

        for i in 0..(nr) {
            let acrate = crates[from].pop_front().unwrap();
            crates[to].insert(i, acrate);
        }
    }
    // println!("{:?}", crates);
    let answer: Vec<char> = crates.iter().map(|c| c[0]).collect();

    Some(answer.into_iter().collect::<String>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
