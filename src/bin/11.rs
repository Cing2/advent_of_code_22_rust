use closure::closure;
use itertools::Itertools;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    fmt,
};

struct Monkey {
    items: RefCell<VecDeque<i64>>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: (i64, i32, i32), // divisible by, true to monkey, false to monkey
    nr_inspect: Cell<i64>,
}
impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey, items: {:?}, test: {:?}, inspections: {:?}",
            self.items, self.test, self.nr_inspect
        )
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
                nr_inspect: Cell::new(0),
            });
            current_monkey = monkeys.len() - 1;
        }
        if line.contains("Starting items:") {
            let (_, right) = line.split_once(':').unwrap();
            monkeys[current_monkey].items = Into::<VecDeque<i64>>::into(
                right
                    .split(',')
                    .map(|c| c.trim().parse::<i64>().unwrap_or_default())
                    .collect_vec(),
            )
            .into();
        } else if line.contains("Operation") {
            //  create operation function
            if line.contains('+') {
                let (_, right) = line.split_once('+').unwrap();
                let num = right.trim().parse::<i64>().unwrap();
                monkeys[current_monkey].operation = Box::new(closure!(move num, |old| {old + num}));
            } else if line.contains("old * old") {
                monkeys[current_monkey].operation = Box::new(|old| old * old);
            } else if line.contains('*') {
                let (_, right) = line.split_once('*').unwrap();
                let num = right.trim().parse::<i64>().unwrap();
                monkeys[current_monkey].operation = Box::new(closure!(move num, |old| {old * num}));
            }
        } else if line.contains("divisible by") {
            let (_, right) = line.split_once("divisible by ").unwrap();
            monkeys[current_monkey].test.0 = right.parse::<i64>().unwrap();
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

pub fn part_one(input: &str) -> Option<i64> {
    let mut monkeys = create_monkeys(input);
    // println!("{monkeys:?}");

    for _ in 0..20 {
        // simulate round
        for i in 0..monkeys.len() {
            let current_monkey = &monkeys[i];
            let mut items = current_monkey.items.borrow_mut();
            for _ in 0..items.len() {
                current_monkey
                    .nr_inspect
                    .set(current_monkey.nr_inspect.get() + 1);
                let mut item = items.pop_front().unwrap();
                item = (current_monkey.operation)(item) / 3;
                // println!("{item}");
                if item % current_monkey.test.0 == 0 {
                    let other_idx = current_monkey.test.1 as usize;
                    monkeys[other_idx].items.borrow_mut().push_back(item);
                } else {
                    monkeys[current_monkey.test.2 as usize]
                        .items
                        .borrow_mut()
                        .push_back(item);
                }
            }
        }
    }
    // println!("{monkeys:?}");

    monkeys.sort_by_key(|c| -c.nr_inspect.get());
    Some(monkeys[0].nr_inspect.get() * monkeys[1].nr_inspect.get())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut monkeys = create_monkeys(input);
    // println!("{monkeys:?}");
    let worry_divider = monkeys
        .iter()
        .map(|m| m.test.0)
        .reduce(|a, b| a * b)
        .unwrap();
    for _ in 0..10000 {
        // simulate round
        for i in 0..monkeys.len() {
            let current_monkey = &monkeys[i];
            let mut items = current_monkey.items.borrow_mut();
            for _ in 0..items.len() {
                current_monkey
                    .nr_inspect
                    .set(current_monkey.nr_inspect.get() + 1);
                let mut item = items.pop_front().unwrap();
                item = (current_monkey.operation)(item);
                item %= worry_divider;
                // println!("{item}");
                if item % current_monkey.test.0 == 0 {
                    monkeys[current_monkey.test.1 as usize]
                        .items
                        .borrow_mut()
                        .push_back(item);
                } else {
                    // item =
                    monkeys[current_monkey.test.2 as usize]
                        .items
                        .borrow_mut()
                        .push_back(item);
                }
            }
        }
    }

    monkeys.sort_by_key(|c| -c.nr_inspect.get());
    // println!("{monkeys:?}");

    Some(monkeys[0].nr_inspect.get() * monkeys[1].nr_inspect.get())
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
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
