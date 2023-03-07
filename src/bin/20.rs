use std::collections::VecDeque;

fn parse_numbers(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let numbers = parse_numbers(input);

    let mut new_numbers: VecDeque<i32> = numbers.clone().into();
    // keeping a seperate list of unique ids to prevent mess up with duplicates
    let mut ids_numbers: VecDeque<usize> = numbers.iter().enumerate().map(|(i, _)| i).collect();

    let length_numbers = new_numbers.len() as i32;

    for (i, num) in numbers.iter().enumerate() {
        // find ids of number in other list to prevent duplicates
        let idx_number: i32 = ids_numbers
            .iter()
            .position(|a| a == &i)
            .unwrap()
            .try_into()
            .unwrap();

        // add them to correct position
        let mut new_idx: i32 = (idx_number + num) % (length_numbers - 1);
        if new_idx <= 0 && idx_number + num != 0 {
            new_idx += length_numbers - 1;
        }

        // remove numbers
        new_numbers.remove(idx_number as usize);
        ids_numbers.remove(idx_number as usize);

        new_numbers.insert(new_idx as usize, *num);
        ids_numbers.insert(new_idx as usize, i);
    }

    let add_idxs: Vec<usize> = vec![1000, 2000, 3000];
    let idx_zero = new_numbers.iter().position(|n| n == &0).unwrap();
    let outcome: i32 = add_idxs
        .iter()
        .map(|add_idx| new_numbers[(add_idx + idx_zero) % length_numbers as usize])
        .sum();
    dbg!(outcome);
    Some(outcome)
    // None
}

pub fn part_two(input: &str) -> Option<i32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
