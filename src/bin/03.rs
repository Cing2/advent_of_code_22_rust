use itertools::Itertools;

fn line_to_priority(line: &str) -> u32 {
    char_containers(line)
        .iter()
        .map(|c| {
            if (c.to_owned() as i32) <= 90 {
                // if uppercase
                c.to_owned() as u32 - 38
            } else {
                c.to_owned() as u32 - 96
            }
        })
        .sum()
}
fn char_containers(line: &str) -> Vec<char> {
    let (left, right) = line.split_at(line.len() / 2);
    left.chars()
        .filter(|c| right.chars().contains(c))
        .dedup()
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(line_to_priority).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .tuples()
            .map(|(s1, s2, s3)| -> u32 {
                s1.chars()
                    .filter(|c| s2.chars().contains(c) && s3.chars().contains(c))
                    .dedup()
                    .map(|c| -> u32 {
                        if (c.to_owned() as i32) <= 90 {
                            // if uppercase
                            c.to_owned() as u32 - 38
                        } else {
                            c.to_owned() as u32 - 96
                        }
                    })
                    .sum()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
