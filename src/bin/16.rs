use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: i32,
    leads_to: Vec<&'a str>,
}

fn parse_input(input: &str) -> HashMap<&str, Valve> {
    let mut valves = HashMap::new();

    let re = Regex::new(r"Valve (?P<name>\w+) has flow rate=(?P<rate>\d+); tunnels* leads* to valves* (?P<valves>.*)").unwrap();

    for cap in re.captures_iter(input) {
        let to_valves = cap
            .name("valves")
            .unwrap()
            .as_str()
            .split(",")
            .map(str::trim)
            .collect();
        let new_valve = Valve {
            flow_rate: cap.name("rate").unwrap().as_str().parse::<i32>().unwrap(),
            leads_to: to_valves,
        };
        let name = cap.name("name").unwrap().as_str();
        valves.insert(name, new_valve);
    }

    // dbg!(&valves);
    valves
}

fn simulate_valves(
    valves: &HashMap<&str, Valve>,
    max_open_valves: usize,
    min_left: i32,
    at_valve_name: &str,
    open_valves: &Vec<&str>,
    visited_valves_since_open: &Vec<&str>,
) -> i32 {
    // do not allow to visited more then 5 files without opening
    if visited_valves_since_open.len() > 4 {
        return 0;
    }
    if min_left == 0 {
        return 0;
    }

    let current_valve = valves.get(at_valve_name).unwrap();
    // update the current pressure
    let current_pressure: i32 = open_valves
        .iter()
        .map(|v| valves.get(v).unwrap().flow_rate)
        .sum();

    // check if we have all valves open
    if open_valves.len() == max_open_valves {
        return current_pressure * min_left;
    }

    // for each possible value return the maximum
    let mut max_buildup_pressure = 0;

    // open current valve if possible
    let opening_pressure = if !open_valves.contains(&at_valve_name) && current_valve.flow_rate > 0 {
        let mut new_open_valves = open_valves.clone();
        new_open_valves.push(at_valve_name);
        simulate_valves(
            valves,
            max_open_valves,
            min_left - 1,
            at_valve_name,
            &new_open_valves,
            &vec![at_valve_name],
        )
    } else {
        0
    };
    max_buildup_pressure = max_buildup_pressure.max(opening_pressure);

    // move to new valves
    for next_valve in &current_valve.leads_to {
        // do not allow to loop back to valves if not opening one
        if !visited_valves_since_open.contains(next_valve) {
            let mut new_visited_valves = visited_valves_since_open.clone();
            new_visited_valves.push(at_valve_name);
            max_buildup_pressure = max_buildup_pressure.max(simulate_valves(
                valves,
                max_open_valves,
                min_left - 1,
                next_valve,
                &open_valves,
                &new_visited_valves,
            ));
        }
    }

    current_pressure + max_buildup_pressure
}

pub fn part_one(input: &str) -> Option<i32> {
    let valves = parse_input(input);

    let nr_valves_with_pressure: usize = valves.iter().filter(|(_, v)| v.flow_rate > 0).count();

    let workers = vec![ValveWorker{ at_valve: "JJ", visited_valves_since_open: vec![] }];


    Some(simulate_valves_elephant(
        &valves,
        nr_valves_with_pressure,
        30,
        &vec![],
        workers
    ))
}

#[derive(Debug, Clone)]
struct ValveWorker<'a> {
    at_valve: &'a str,
    visited_valves_since_open: Vec<&'a str>,
}


fn simulate_valves_elephant(
    valves: &HashMap<&str, Valve>,
    max_open_valves: usize,
    min_left: i32,
    open_valves: &Vec<&str>,
    workers: Vec<ValveWorker>,
) -> i32 {
    if min_left == 0 {
        return 0;
    }
    // building pressure on this minute
    let current_pressure: i32 = open_valves
        .iter()
        .map(|v| valves.get(v).unwrap().flow_rate)
        .sum();

    // check if we have all valves open
    if open_valves.len() == max_open_valves {
        return current_pressure * min_left;
    }

    // for each possible value return the maximum
    let mut max_buildup_pressure = 0;

    for i in 0..workers.len() {
        let worker = &workers[i];
        // do not allow to visited more then 5 files without opening
        if worker.visited_valves_since_open.len() > 4 {
            return 0;
        }

        let current_valve = valves.get(worker.at_valve).unwrap();

        // open current valve if possible
        let opening_pressure =
            if !open_valves.contains(&worker.at_valve) && current_valve.flow_rate > 0 {
                let mut new_open_valves = open_valves.clone();
                new_open_valves.push(worker.at_valve);
                simulate_valves_elephant(
                    valves,
                    max_open_valves,
                    min_left - 1,
                    &new_open_valves,
                    workers.clone(),
                )
            } else {
                0
            };
        max_buildup_pressure = max_buildup_pressure.max(opening_pressure);

        // move to new valves
        for next_valve in &current_valve.leads_to {
            // do not allow to loop back to valves if not opening one
            if !worker.visited_valves_since_open.contains(next_valve) {
                let mut new_workers = workers.clone();
                new_workers[i].visited_valves_since_open.push(&next_valve);
                new_workers[i].at_valve = next_valve;

                max_buildup_pressure = max_buildup_pressure.max(simulate_valves_elephant(
                    valves,
                    max_open_valves,
                    min_left - 1,
                    &open_valves,
                    new_workers,
                ));
            }
        }
    }

    current_pressure + max_buildup_pressure
}

pub fn part_two(input: &str) -> Option<i32> {
    let valves = parse_input(input);

    let nr_valves_with_pressure: usize = valves.iter().filter(|(_, v)| v.flow_rate > 0).count();

    let workers = vec![ValveWorker{ at_valve: "JJ", visited_valves_since_open: vec![] }];


    Some(simulate_valves_elephant(
        &valves,
        nr_valves_with_pressure,
        30,
        &vec![],
        workers
    ))
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
