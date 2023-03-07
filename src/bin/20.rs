use std::collections::VecDeque;

fn parse_numbers(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

type MixedNumbers = VecDeque<(usize, i64)>;

fn mix_numbers(mut new_numbers: MixedNumbers, numbers: &Vec<i64>) -> MixedNumbers {
    let length_numbers = new_numbers.len() as i64;

    for (i, num) in numbers.iter().enumerate() {
        // find idx of number in list
        let idx_number: i64 = new_numbers
            .iter()
            .position(|a| a.0 == i)
            .unwrap()
            .try_into()
            .unwrap();

        // add them to correct position
        let mut new_idx: i64 = (idx_number + num) % (length_numbers - 1);
        if new_idx <= 0 && idx_number + num != 0 {
            new_idx += length_numbers - 1;
        }

        // remove numbers
        new_numbers.remove(idx_number as usize);

        new_numbers.insert(new_idx as usize, (i, *num));
    }

    new_numbers
}

fn get_key_numbers(new_numbers: MixedNumbers) -> i64 {
    let length_numbers = new_numbers.len() as i64;

    let add_idxs: Vec<usize> = vec![1000, 2000, 3000];
    let idx_zero = new_numbers.iter().position(|n| n.1 == 0).unwrap();
    let outcome: i64 = add_idxs
        .iter()
        .map(|add_idx| new_numbers[(add_idx + idx_zero) % length_numbers as usize].1)
        .sum();

    outcome
}

pub fn part_one(input: &str) -> Option<i64> {
    let numbers = parse_numbers(input);

    // keeping a tupled list of unique ids to prevent mess up with duplicates
    let mut new_numbers: MixedNumbers = numbers.iter().copied().enumerate().collect();
    // do the mixing
    new_numbers = mix_numbers(new_numbers, &numbers);

    Some(get_key_numbers(new_numbers))
}

pub fn part_two(input: &str) -> Option<i64> {
    let decryption_key = 811589153;
    // dbg!(parse_numbers(input));
    let numbers: Vec<i64> = parse_numbers(input)
        .iter()
        .map(|num| num * decryption_key)
        .collect();

    // keeping a tupled list of unique ids to prevent mess up with duplicates
    let mut new_numbers: MixedNumbers = numbers.iter().copied().enumerate().collect();

    for _ in 0..10 {
        new_numbers = mix_numbers(new_numbers, &numbers);
    }

    Some(get_key_numbers(new_numbers))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
