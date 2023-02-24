use std::collections::HashMap;

use regex::Regex;

type ValveKey = [char; 2];

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    leads_to: Vec<ValveKey>,
}

fn valvekey_from_str(input: &str) -> ValveKey {
    [input.chars().nth(0).unwrap(), input.chars().nth(1).unwrap()] as ValveKey
}

fn parse_input(input: &str) -> HashMap<ValveKey, Valve> {
    let mut valves: HashMap<ValveKey, Valve> = HashMap::new();

    let re = Regex::new(r"Valve (?P<name>\w+) has flow rate=(?P<rate>\d+); tunnels* leads* to valves* (?P<valves>.*)").unwrap();

    for cap in re.captures_iter(input) {
        let to_valves = cap
            .name("valves")
            .unwrap()
            .as_str()
            .split(",")
            .map(str::trim)
            .map(valvekey_from_str)
            .collect();
        let new_valve = Valve {
            flow_rate: cap.name("rate").unwrap().as_str().parse::<i32>().unwrap(),
            leads_to: to_valves,
        };
        let name = cap.name("name").unwrap().as_str();
        valves.insert(valvekey_from_str(name), new_valve);
    }

    // dbg!(&valves);
    valves
}

fn simulate_valves(
    valves: &HashMap<ValveKey, Valve>,
    max_open_valves: usize,
    min_left: i32,
    at_valve_name: ValveKey,
    open_valves: &mut Vec<ValveKey>,
    visited_valves_since_open: &Vec<ValveKey>,
) -> i32 {
    // do not allow to visited more then 5 files without opening
    if visited_valves_since_open.len() > 5 {
        return 0;
    }
    if min_left == 0 {
        return 0;
    }

    // let current_valve = valves[&at_valve_name];
    // update the current pressure
    let current_pressure: i32 = open_valves
        .iter()
        .map(|v| valves[v].flow_rate)
        .sum();

    // check if we have all valves open
    if open_valves.len() == max_open_valves {
        return current_pressure * min_left;
    }

    // for each possible value return the maximum
    let mut max_buildup_pressure = 0;

    // open current valve if possible
    let mut opening_pressure = 0;
    if !open_valves.contains(&at_valve_name) && valves[&at_valve_name].flow_rate > 0 {
        open_valves.push(at_valve_name);
        opening_pressure = simulate_valves(
            valves,
            max_open_valves,
            min_left - 1,
            at_valve_name,
            open_valves,
            &vec![at_valve_name],
        );
        open_valves.pop();
        
    }
    max_buildup_pressure = max_buildup_pressure.max(opening_pressure);

    // move to new valves
    for next_valve in &valves[&at_valve_name].leads_to {
        // do not allow to loop back to valves if not opening one
        if !visited_valves_since_open.contains(&next_valve) {
            let mut new_visited_valves = visited_valves_since_open.clone();
            new_visited_valves.push(at_valve_name);
            max_buildup_pressure = max_buildup_pressure.max(simulate_valves(
                valves,
                max_open_valves,
                min_left - 1,
                *next_valve,
                open_valves,
                &new_visited_valves,
            ));
        }
    }

    current_pressure + max_buildup_pressure
}

pub fn part_one(input: &str) -> Option<i32> {
    let valves = parse_input(input);

    let nr_valves_with_pressure: usize = valves.iter().filter(|(_, v)| v.flow_rate > 0).count();
    let mut open_valves = vec![];
    Some(simulate_valves(
        &valves,
        nr_valves_with_pressure,
        30,
        valvekey_from_str("AA"),
        &mut open_valves,
        &vec![],
    ))
    // None
}

pub fn part_two(input: &str) -> Option<i32> {
    let valves = parse_input(input);

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
