use std::collections::VecDeque;

use itertools::Itertools;

fn parse_numbers(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let numbers = parse_numbers(input);
    // let dups: Vec<i32> = numbers.into_iter().duplicates().collect();
    // dbg!(dups);
    // println!("{:?}", &numbers);

    let mut new_numbers: VecDeque<i32> = numbers.clone().into();

    let length_numbers = new_numbers.len() as i32;

    for (i, num) in numbers.iter().enumerate() {
        // find number
        let idx_number: i32 = new_numbers
            .iter()
            .position(|a| a == num)
            .unwrap()
            .try_into()
            .unwrap();

        let mut number_swaps = num.abs();
        let mut j = 0;
        loop {
            // move up number 1 slot
            if num > &0 {
                let from_idx = (idx_number + j) % length_numbers;
                let to_idx = (idx_number + j + 1) % length_numbers;
                if from_idx == (length_numbers - 1) {
                    // if moving to end of file, pop and add
                    let moving_element = new_numbers.pop_back().unwrap();
                    new_numbers.push_front(moving_element);
                    number_swaps += 1;
                    // new_numbers.insert((1) as usize, moving_element);
                } else {
                    new_numbers.swap(from_idx as usize, to_idx as usize);
                }
            } else {
                let from_idx = (idx_number - j + length_numbers * 10) % length_numbers;
                let to_idx = (idx_number - j - 1 + length_numbers * 10) % length_numbers;
                if from_idx == 0 {
                    // if wrapping arround pop and add to back
                    let moving_element = new_numbers.pop_front().unwrap();
                    new_numbers.push_back(moving_element);
                    number_swaps += 1;
                    // new_numbers.insert((length_numbers-2) as usize, moving_element);
                } else {
                    new_numbers.swap(from_idx as usize, to_idx as usize);
                }
            }

            j += 1;
            if j >= number_swaps {
                break;
            }
        }
        println!("{:?}", &new_numbers);

        // // remove old number
        // // let idx_old_number = match new_idx < idx_number {
        // //     true => idx_number + 1,
        // //     false => idx_number,
        // // };
        // new_numbers.remove(idx_number);

        // // move number to other position
        // let mut new_idx = idx_number as i32 + (*num + length_numbers);
        // if num < &0 {
        //     print!("a");
        //     new_idx = (new_idx - 1) % length_numbers;
        // } else{
        //     new_idx = new_idx % length_numbers
        // }
        // new_numbers.insert(new_idx as usize , *num);

        // println!("Moved num: {num} from {} to {}", idx_number, new_idx);
    }

    let add_idxs: Vec<usize> = vec![1000, 2000, 3000];
    let idx_zero = new_numbers.iter().position(|n| n == &0).unwrap();
    let outcome: i32 = add_idxs
        .iter()
        .map(|add_idx| new_numbers[(add_idx + idx_zero) % length_numbers as usize])
        .sum();
    dbg!(outcome);
    // Some(outcome)
    None
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
