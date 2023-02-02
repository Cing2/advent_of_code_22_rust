use itertools::Itertools;

fn no_duplicates(chars: Vec<char>) -> bool {
    if chars.len() == 1{
        return true;
    }
    for i in 1..(chars.len()){
        if chars[0] == chars[i] {
            return false;
        }
    }

    no_duplicates(chars[1..].to_vec())
}
fn calculate_start_packet(input: &str, nr_chars_dup:usize) -> Option<i32> {
    let chars: Vec<char> = input.chars().collect_vec();

    for i in 0..input.len() {
        if no_duplicates(chars[i..i+nr_chars_dup].to_vec())
        {
            return Some((i + nr_chars_dup).try_into().unwrap());
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<i32> {
    calculate_start_packet(input, 4)
}

pub fn part_two(input: &str) -> Option<i32> {
    calculate_start_packet(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
