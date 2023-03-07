use std::cell::Cell;

use hashbrown::HashMap;

#[derive(Debug)]
struct MonkeyTask {
    monkeys: (String, String),
    operation: Operation,
    has_human_node: Cell<bool>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Monkey {
    Number(i64),
    Operation(MonkeyTask),
}

type MonkeysMap = HashMap<String, Monkey>;

fn parse_monkeys(input: &str) -> MonkeysMap {
    let mut monkeys: MonkeysMap = Default::default();

    for line in input.lines() {
        let (mut monkey, task) = line.split_once(": ").unwrap();

        let operation: Option<Operation> = if task.contains('+') {
            Some(Operation::Add)
        } else if task.contains(" / ") {
            Some(Operation::Divide)
        } else if task.contains(" * ") {
            Some(Operation::Multiply)
        } else if task.contains(" - ") {
            Some(Operation::Subtract)
        } else {
            None
        };

        let task = if let Some(op) = operation {
            let monkeys_string: (&str, &str) =
                task.split_once(['+', '-', '*', '/']).unwrap().into();
            let with_monkeys = (
                monkeys_string.0.trim().to_owned(),
                monkeys_string.1.trim().to_owned(),
            );

            Monkey::Operation(MonkeyTask {
                monkeys: with_monkeys,
                operation: op,
                has_human_node: Cell::new(false),
            })
        } else {
            let num = task.parse::<i64>().unwrap();
            Monkey::Number(num)
        };

        monkeys.insert(monkey.to_owned(), task);
    }

    monkeys
}

fn monkey_says(monkeys: &MonkeysMap, name_monkey: &String) -> i64 {
    match &monkeys[name_monkey] {
        Monkey::Number(num) => *num,
        Monkey::Operation(task) => {
            let output_monkeys = (
                monkey_says(monkeys, &task.monkeys.0),
                monkey_says(monkeys, &task.monkeys.1),
            );
            match task.operation {
                Operation::Add => output_monkeys.0 + output_monkeys.1,
                Operation::Subtract => output_monkeys.0 - output_monkeys.1,
                Operation::Multiply => output_monkeys.0 * output_monkeys.1,
                Operation::Divide => output_monkeys.0 / output_monkeys.1,
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let monkeys = parse_monkeys(input);

    Some(monkey_says(&monkeys, &String::from("root")))
}

fn annotate_nodes_with_human(monkeys: &MonkeysMap, name_monkey: &String) -> bool {
    if name_monkey == "humn" {
        return true;
    }

    match &monkeys[name_monkey] {
        Monkey::Number(_) => false,
        Monkey::Operation(operation) => {
            let value = annotate_nodes_with_human(monkeys, &operation.monkeys.0)
                || annotate_nodes_with_human(monkeys, &operation.monkeys.1);
            operation.has_human_node.set(value);
            value
        }
    }
}

fn reverse_human_value(monkeys: &MonkeysMap, monkey: &String, value: i64) -> i64 {
    // value of monkey should be same as value

    // monkey has human in it
    if monkey == "humn" {
        return value;
    }

    match &monkeys[monkey] {
        Monkey::Number(_) => todo!(),
        Monkey::Operation(node) => {
            // depending on which child has human in it

            let child_one_human: bool = if let Monkey::Operation(child) = &monkeys[&node.monkeys.0]
            {
                child.has_human_node.get()
            } else {
                // if child is only a number than it does not have human, except if it is the humand
                node.monkeys.0 == "humn"
            };

            if child_one_human {
                // if left has human in it
                let value_other = monkey_says(&monkeys, &node.monkeys.1);
                let new_value = match node.operation {
                    Operation::Add => value - value_other,
                    Operation::Subtract => value + value_other,
                    Operation::Multiply => value / value_other,
                    Operation::Divide => value * value_other,
                };
                return reverse_human_value(&monkeys, &node.monkeys.0, new_value);
            } else {
                // if right has human in it
                let value_other = monkey_says(&monkeys, &node.monkeys.0);
                let new_value = match node.operation {
                    Operation::Add => value - value_other,
                    Operation::Subtract => value_other - value,
                    Operation::Multiply => value / value_other,
                    Operation::Divide => value_other / value,
                };

                return reverse_human_value(&monkeys, &node.monkeys.1, new_value);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let monkeys = parse_monkeys(input);

    annotate_nodes_with_human(&monkeys, &String::from("root"));

    // for mon in &monkeys {
    //     println!("{:?}", mon);
    // }
    // dbg!(&monkeys);

    if let Monkey::Operation(node) = &monkeys[&String::from("root")] {
        if let Monkey::Operation(child) = &monkeys[&node.monkeys.0] {
            let human_value = if child.has_human_node.get() {
                reverse_human_value(
                    &monkeys,
                    &node.monkeys.0,
                    monkey_says(&monkeys, &node.monkeys.1),
                )
            } else {
                reverse_human_value(
                    &monkeys,
                    &node.monkeys.1,
                    monkey_says(&monkeys, &node.monkeys.0),
                )
            };
            return Some(human_value);
        }
    }

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
        assert_eq!(part_two(&input), Some(301));
    }
}
