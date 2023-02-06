use closure::closure;
use std::{borrow::Borrow, cell::RefCell, collections::VecDeque, fmt};

use itertools::{enumerate, Itertools};

struct Monkey {
    items: RefCell<VecDeque<i32>>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: (i32, i32, i32), // divisible by, true to monkey, false to monkey
}
impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "items: {:?}, test: {:?}", self.items, self.test)
    }
}

fn create_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut current_monkey = 0;

    for line in input.lines() {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey {
                items: VecDeque::new().into(),
                operation: Box::new(|a| a),
                test: (0, 0, 0),
            });
            current_monkey = monkeys.len() - 1;
        }
        if line.contains("Starting items:") {
            let (_, right) = line.split_once(':').unwrap();
            monkeys[current_monkey].items = Into::<VecDeque<i32>>::into(
                right
                    .split(",")
                    .map(|c| c.trim().parse::<i32>().unwrap_or_default())
                    .collect_vec(),
            )
            .into();
        } else if line.contains("operation") {
            //  create operation function
            if line.contains("+") {
                let (_, right) = line.split_once("to monkey ").unwrap();
                let num = right.trim().parse::<i32>().unwrap();
                monkeys[current_monkey].operation =
                    Box::new(closure!(move num, |old| {old + num.clone()}));
            }
            println!("{}", line);
        } else if line.contains("divisible by") {
            let (_, right) = line.split_once("divisible by ").unwrap();
            monkeys[current_monkey].test.0 = right.parse::<i32>().unwrap();
        } else if line.contains("true") {
            let (_, right) = line.split_once("to monkey ").unwrap();
            monkeys[current_monkey].test.1 = right.parse::<i32>().unwrap();
        } else if line.contains("false") {
            let (_, right) = line.split_once("to monkey ").unwrap();
            monkeys[current_monkey].test.2 = right.parse::<i32>().unwrap();
        }
    }

    monkeys
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = create_monkeys(input);
    println!("{:?}", monkeys);

    for _ in 0..20 {
        // simulate round
        for i in 0..monkeys.len() {
            let current_monkey = &monkeys[i];
            for _ in 0..current_monkey.items.borrow().len() {
                let mut item = current_monkey.items.borrow_mut().pop_front().unwrap();
                item = (current_monkey.operation)(item);
                if item & current_monkey.test.0 == 0 {
                    let other_idx = current_monkey.test.1 as usize;
                    monkeys[other_idx].items.borrow_mut().push_back(item);
                } else {
                    monkeys[current_monkey.test.2 as usize]
                        .items.borrow_mut()
                        .push_back(item);
                }
            }
        }
        println!("{:?}", monkeys);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
