use hashbrown::HashMap;
use std::collections::BTreeMap;
use std::collections::VecDeque;

use hashbrown::HashSet;
use itertools::Itertools;

use regex::Regex;

// type Name = [char; 2];
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Name([char; 2]);

impl Name {
    fn from_str(input: &str) -> Self {
        Name([input.chars().next().unwrap(), input.chars().nth(1).unwrap()])
    }
}

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    leads_to: Vec<Name>,
}

struct Network {
    valves: HashMap<Name, Valve>,
}

impl Network {
    fn list_non_zero_valves(&self) -> Vec<Name> {
        self.valves
            .iter()
            .filter_map(|(n, v)| match v.flow_rate > 0 {
                true => Some(*n),
                false => None,
            })
            .sorted()
            .collect_vec()
    }
}

fn parse_input(input: &str) -> Network {
    let mut valves: HashMap<Name, Valve> = HashMap::new();

    let re = Regex::new(r"Valve (?P<name>\w+) has flow rate=(?P<rate>\d+); tunnels* leads* to valves* (?P<valves>.*)").unwrap();

    for cap in re.captures_iter(input) {
        let to_valves = cap
            .name("valves")
            .unwrap()
            .as_str()
            .split(',')
            .map(str::trim)
            .map(Name::from_str)
            .collect();
        let new_valve = Valve {
            flow_rate: cap.name("rate").unwrap().as_str().parse::<i32>().unwrap(),
            leads_to: to_valves,
        };
        let name = cap.name("name").unwrap().as_str();
        valves.insert(Name::from_str(name), new_valve);
    }

    Network { valves }
}

fn simulate_valves(
    network: &Network,
    path_lengths: &PathLengths,
    pressure_valves: &Vec<Name>,
    min_left: i32,
    at_valve: Name,
    open_valves: &mut Vec<Name>,
    current_pressure: i32,
) -> i32 {
    if min_left == 0 {
        return 0;
    }
    // for each possible valve return the maximum
    let mut max_buildup_pressure = 0;

    // move to next valve and open
    for next_valve in pressure_valves {
        // do not open valve that are already open and check if we still have time to open it
        if !open_valves.contains(next_valve) && path_lengths[&(at_valve, *next_valve)] < min_left {
            // open valve and add pressure
            open_valves.push(*next_valve);
            let time_left = min_left - path_lengths[&(at_valve, *next_valve)] - 1; // n for walking and 1 for opening
            max_buildup_pressure = max_buildup_pressure.max(simulate_valves(
                network,
                path_lengths,
                pressure_valves,
                time_left,
                *next_valve,
                open_valves,
                network.valves[next_valve].flow_rate * time_left,
            ));
            open_valves.pop();
        }
    }

    current_pressure + max_buildup_pressure
}

type PathLengths = BTreeMap<(Name, Name), i32>;

fn length_paths_valves(network: &Network, valves: &Vec<Name>) -> PathLengths {
    let mut path_lengths: PathLengths = Default::default();

    // for each node to breadth first search to get the distance to all nodes
    for root in valves {
        let mut queue: VecDeque<(Name, i32)> = Default::default();
        let mut visited: HashSet<Name> = HashSet::new();
        visited.insert(*root);
        queue.push_back((*root, 0));
        while !queue.is_empty() {
            let valve = queue.pop_front().unwrap();
            for other in &network.valves[&valve.0].leads_to {
                if !visited.contains(other) {
                    visited.insert(*other);
                    queue.push_back((*other, valve.1 + 1));
                    if valves.contains(other) && !path_lengths.contains_key(&(*root, *other)) {
                        path_lengths.insert((*root, *other), valve.1 + 1);
                    }
                }
            }
        }
    }
    path_lengths
}

pub fn factorial(num: i32) -> i32 {
    (1..=num).product()
}

pub fn part_one(input: &str) -> Option<i32> {
    let network = parse_input(input);

    let pressure_valves: Vec<Name> = network.list_non_zero_valves();
    let mut pressure_valves_aa = pressure_valves.clone();
    pressure_valves_aa.push(Name::from_str("AA"));

    let path_lengths: PathLengths = length_paths_valves(&network, &pressure_valves_aa);
    // println!("{:?}", &path_lengths);

    let pressure = simulate_valves(
        &network,
        &path_lengths,
        &pressure_valves,
        30,
        Name::from_str("AA"),
        &mut vec![],
        0,
    );
    Some(pressure)
}

#[allow(clippy::too_many_arguments)]
fn simulate_valves_elephant(
    network: &Network,
    path_lengths: &PathLengths,
    pressure_valves: &Vec<Name>,
    min_left: i32,
    at_valve: Name,
    elephant_min: i32,
    elephant_valve: Name,
    open_valves: &mut Vec<Name>,
    current_pressure: i32,
) -> i32 {
    if min_left == 0 {
        return 0;
    }
    // for each possible value return the maximum
    let mut max_buildup_pressure = 0;

    // which to check first
    if min_left >= elephant_min {
        // move to next valve and open
        for next_valve in pressure_valves {
            // do not open valve that are already open and check if we still have time to open it
            if !open_valves.contains(next_valve)
                && path_lengths[&(at_valve, *next_valve)] < min_left
            {
                // open valve and add pressure
                open_valves.push(*next_valve);
                let time_left = min_left - path_lengths[&(at_valve, *next_valve)] - 1; // n for walking and 1 for opening
                max_buildup_pressure = max_buildup_pressure.max(simulate_valves_elephant(
                    network,
                    path_lengths,
                    pressure_valves,
                    time_left,
                    *next_valve,
                    elephant_min,
                    elephant_valve,
                    open_valves,
                    network.valves[next_valve].flow_rate * time_left,
                ));
                open_valves.pop();
            }
        }
    } else {
        // elephant turn
        // move to next valve and open
        for next_valve in pressure_valves {
            // do not open valve that are already open and check if we still have time to open it
            if !open_valves.contains(next_valve)
                && path_lengths[&(elephant_valve, *next_valve)] < elephant_min
            {
                // open valve and add pressure
                open_valves.push(*next_valve);
                let time_left = elephant_min - path_lengths[&(elephant_valve, *next_valve)] - 1; // n for walking and 1 for opening
                max_buildup_pressure = max_buildup_pressure.max(simulate_valves_elephant(
                    network,
                    path_lengths,
                    pressure_valves,
                    min_left,
                    at_valve,
                    time_left,
                    *next_valve,
                    open_valves,
                    network.valves[next_valve].flow_rate * time_left,
                ));
                open_valves.pop();
            }
        }
    }

    current_pressure + max_buildup_pressure
}

pub fn part_two(input: &str) -> Option<i32> {
    let network = parse_input(input);

    let pressure_valves: Vec<Name> = network.list_non_zero_valves();
    let mut pressure_valves_aa = pressure_valves.clone();
    pressure_valves_aa.push(Name::from_str("AA"));

    // println!("{:?}", &pressure_valves);
    let path_lengths: PathLengths = length_paths_valves(&network, &pressure_valves_aa);

    let pressure = simulate_valves_elephant(
        &network,
        &path_lengths,
        &pressure_valves,
        26,
        Name::from_str("AA"),
        26,
        Name::from_str("AA"),
        &mut vec![],
        0,
    );
    Some(pressure)
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
        assert_eq!(part_two(&input), Some(1707));
    }
}
