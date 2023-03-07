use std::fmt;

use hashbrown::HashMap;

struct MonkeyOperation<'a> {
    monkeys: (&'a str, &'a str),
    operation: Box<dyn Fn(i64, i64) -> i64>,
}

impl fmt::Debug for MonkeyOperation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkeys: {:?}", self.monkeys)
    }
}

#[derive(Debug)]
enum MonkeyTask<'a> {
    Num(i64),
    Operation(MonkeyOperation<'a>),
}

type Monkeys<'a> = HashMap<&'a str, MonkeyTask<'a>>;

fn parse_monkeys(input: &str) -> Monkeys {
    let mut monkeys: Monkeys = Default::default();

    for line in input.lines() {
        let (monkey, task) = line.split_once(": ").unwrap();
        let task: MonkeyTask = if task.contains('+') {
            let monkeys: (&str, &str) = task.split_once(" + ").unwrap().into();
            MonkeyTask::Operation(MonkeyOperation {
                monkeys,
                operation: Box::new(|a, b| a + b),
            })
        } else if task.contains(" / ") {
            let monkeys: (&str, &str) = task.split_once(" / ").unwrap().into();
            MonkeyTask::Operation(MonkeyOperation {
                monkeys,
                operation: Box::new(|a, b| a / b),
            })
        } else if task.contains(" * ") {
            let monkeys: (&str, &str) = task.split_once(" * ").unwrap().into();
            MonkeyTask::Operation(MonkeyOperation {
                monkeys,
                operation: Box::new(|a, b| a * b),
            })
        } else if task.contains(" - ") {
            let monkeys: (&str, &str) = task.split_once(" - ").unwrap().into();
            MonkeyTask::Operation(MonkeyOperation {
                monkeys,
                operation: Box::new(|a, b| a - b),
            })
        } else {
            let num = task.parse::<i64>().unwrap();
            MonkeyTask::Num(num)
        };
        monkeys.insert(monkey, task);
    }

    monkeys
}

fn monkey_says(monkeys: &Monkeys, name_monkey: &str) -> i64 {
    match &monkeys[name_monkey] {
        MonkeyTask::Num(num) => *num,
        MonkeyTask::Operation(task) => (task.operation)(monkey_says(monkeys, task.monkeys.0), monkey_says(monkeys, task.monkeys.1)),
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let monkeys = parse_monkeys(input);
    // dbg!(&monkeys);

    Some(monkey_says(&monkeys, "root"))
    // None
}

pub fn part_two(input: &str) -> Option<i64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
