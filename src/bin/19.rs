use std::{
    ops::{Add, AddAssign},
    time::Instant,
};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct BluePrint {
    ore_robot: u32,             // ore needed for ore robot
    clay_robot: u32,            // ore
    obsidian_robot: (u32, u32), // ore and clay
    geode_robot: (u32, u32),    // ore, obsidian
}

impl BluePrint {
    fn parse_line(line: &str) -> Option<BluePrint> {
        let re = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

        match re.captures(line) {
            Some(cap) => Some(BluePrint {
                ore_robot: cap[1].parse::<u32>().unwrap(),
                clay_robot: cap[2].parse::<u32>().unwrap(),
                obsidian_robot: (
                    cap[3].parse::<u32>().unwrap(),
                    cap[4].parse::<u32>().unwrap(),
                ),
                geode_robot: (
                    cap[5].parse::<u32>().unwrap(),
                    cap[6].parse::<u32>().unwrap(),
                ),
            }),
            _ => None,
        }
    }
}

fn parse_blueprint(input: &str) -> Vec<BluePrint> {
    input.lines().filter_map(BluePrint::parse_line).collect()
}

#[derive(Debug, Default, Clone, Copy)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geodes: self.geodes + rhs.geodes,
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, other: Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geodes += other.geodes;
    }
}

#[derive(Debug, Clone, Copy)]
struct Factory {
    robots: Resources,
    resource: Resources,
    blueprint: BluePrint,
    min_left: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Actions {
    NOOP,
    BuildOre,
    BuildClay,
    BuildObsidian,
    BuildGeode,
}

impl Default for Actions {
    fn default() -> Self {
        Self::NOOP
    }
}

impl Factory {
    fn new(blueprint: BluePrint, min_left: u32) -> Self {
        Self {
            robots: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geodes: 0,
            },
            resource: Default::default(),
            blueprint,
            min_left,
        }
    }

    fn step(&mut self, action: &Actions) {
        // perform a minute step
        self.min_left -= 1;
        // add new ore
        self.resource += self.robots;

        // println!("{:?}- {:?}", &self.robots, &self.resource);
        // build robot if action
        match action {
            Actions::NOOP => (),
            Actions::BuildOre => {
                self.robots.ore += 1;
                self.resource.ore -= self.blueprint.ore_robot;
            }
            Actions::BuildClay => {
                self.robots.clay += 1;
                self.resource.ore -= self.blueprint.clay_robot;
            }
            Actions::BuildObsidian => {
                self.robots.obsidian += 1;
                self.resource.ore -= self.blueprint.obsidian_robot.0;
                self.resource.clay -= self.blueprint.obsidian_robot.1;
            }
            Actions::BuildGeode => {
                self.robots.geodes += 1;
                self.resource.ore -= self.blueprint.geode_robot.0;
                self.resource.obsidian -= self.blueprint.geode_robot.1;
            }
        }
    }

    fn list_possible_actions(&self) -> Vec<Actions> {
        let mut options = vec![];
        if self.min_left == 1 {
            return vec![Actions::NOOP];
        }

        if self.resource.ore >= self.blueprint.geode_robot.0
            && self.resource.obsidian >= self.blueprint.geode_robot.1
        {
            // if possible always build geode node
            return vec![Actions::BuildGeode];
        }

        let max_ore_cost = self
            .blueprint
            .clay_robot
            .max(self.blueprint.obsidian_robot.0)
            .max(self.blueprint.geode_robot.0);

        // only make robot if below max otherwise this is not usefull
        if self.robots.ore < max_ore_cost && self.resource.ore >= self.blueprint.ore_robot {
            // check if making robot has a benefit, new robot should be able to make 1 resource
            if self.min_left > (self.blueprint.ore_robot + 2) {
                options.push(Actions::BuildOre);
            }
        }
        // same only make clay robot if below max
        if self.robots.clay < self.blueprint.obsidian_robot.1
            && self.resource.ore >= self.blueprint.clay_robot
        {
            if self.min_left > (self.blueprint.clay_robot + 2) {
                options.push(Actions::BuildClay);
            }
        }
        if self.robots.obsidian < self.blueprint.geode_robot.1
            && self.resource.ore >= self.blueprint.obsidian_robot.0
            && self.resource.clay >= self.blueprint.obsidian_robot.1
        {
            options.push(Actions::BuildObsidian);
        }

        // add noop latter to prioritize making a robot
        options.push(Actions::NOOP);

        options
    }

    fn simulate_factory(self) -> u32 {
        if self.min_left == 0 {
            return self.resource.geodes;
        }
        let mut best_geodes = 0;

        for action in self.list_possible_actions() {
            let mut next_state = self.clone();
            // println!("{:?}", &next_state.resource);
            next_state.step(&action);
            // println!("{:?}", &next_state.resource);
            // branch and bound, to only consider branches that can possible be better
            if next_state.bound() > best_geodes {
                best_geodes = best_geodes.max(next_state.simulate_factory());
            }
            // println!("Branch skipped {}", self.min_left);
        }

        best_geodes
    }

    fn bound(self) -> u32 {
        // with infinite money and clay how many geodes could we produce
        (0..self.min_left)
            .into_iter()
            .rev()
            .fold(
                (
                    self.resource.obsidian,
                    self.robots.obsidian,
                    self.resource.geodes + (self.robots.geodes * self.min_left),
                ),
                |(obsidian, rate, geodes), min_left| {
                    // build geode robot if possible
                    if obsidian >= self.blueprint.geode_robot.1 {
                        (
                            obsidian + rate - self.blueprint.geode_robot.1,
                            rate,
                            geodes.saturating_add(min_left), // number of new geodes robot can make is number of min left
                        )
                    } else {
                        // build obsidian robot
                        (obsidian + rate, rate + 1, geodes)
                    }
                },
            )
            .2
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_blueprint(input);
    let factories: Vec<Factory> = blueprints
        .iter()
        .copied()
        .map(|blue| Factory::new(blue, 24))
        .collect();
    // let timer = Instant::now();
    // let scores = factories[0].simulate_factory();
    // let elapsed = timer.elapsed();
    // dbg!(scores, elapsed);

    let timer = Instant::now();
    let scores: Vec<u32> = factories.iter().map(|f| f.simulate_factory()).collect();
    let elapsed = timer.elapsed();
    dbg!(&scores, &elapsed);
    let final_score: u32 = scores
        .iter()
        .enumerate()
        .map(|(i, s)| s * ((i + 1) as u32))
        .sum();

    Some(final_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = parse_blueprint(input);
    let factories: Vec<Factory> = blueprints
        .iter()
        .take(3)
        .copied()
        .map(|blue| Factory::new(blue, 32))
        .collect();
    dbg!(&factories);


    let timer = Instant::now();
    let scores: Vec<u32> = factories.iter().map(|f| f.simulate_factory()).collect();
    let elapsed = timer.elapsed();
    dbg!(&scores, &elapsed);
    let final_score: u32 = scores.iter().product();

    Some(final_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
