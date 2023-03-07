use std::ops::{Add, AddAssign};

use rayon::prelude::*;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct BluePrint {
    id: u32,
    ore_robot: u32,             // ore needed for ore robot
    clay_robot: u32,            // ore
    obsidian_robot: (u32, u32), // ore and clay
    geode_robot: (u32, u32),    // ore, obsidian
}

impl BluePrint {
    fn parse_line(line: &str) -> Option<BluePrint> {
        let re = Regex::new(r"Blueprint (?P<id>\d+): Each ore robot costs (?P<o>\d+) ore. Each clay robot costs (?P<c>\d+) ore. Each obsidian robot costs (?P<obo>\d+) ore and (?P<obc>\d+) clay. Each geode robot costs (?P<go>\d+) ore and (?P<gob>\d+) obsidian.").unwrap();

        re.captures(line).map(|cap| BluePrint {
            id: cap["id"].parse::<u32>().unwrap(),
            ore_robot: cap["o"].parse::<u32>().unwrap(),
            clay_robot: cap["c"].parse::<u32>().unwrap(),
            obsidian_robot: (
                cap["obo"].parse::<u32>().unwrap(),
                cap["obc"].parse::<u32>().unwrap(),
            ),
            geode_robot: (
                cap["go"].parse::<u32>().unwrap(),
                cap["gob"].parse::<u32>().unwrap(),
            ),
        })
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
    // blueprint: BluePrint,
    min_left: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Actions {
    Noop,
    BuildOre,
    BuildClay,
    BuildObsidian,
    BuildGeode,
}

impl Default for Actions {
    fn default() -> Self {
        Self::Noop
    }
}

impl Factory {
    fn new(min_left: u32) -> Self {
        Self {
            robots: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geodes: 0,
            },
            resource: Default::default(),
            min_left,
        }
    }

    fn step(&mut self, action: &Actions, blueprint: &BluePrint) {
        // perform a minute step
        self.min_left -= 1;
        // add new ore
        self.resource += self.robots;

        // println!("{:?}- {:?}", &self.robots, &self.resource);
        // build robot if action
        match action {
            Actions::Noop => (),
            Actions::BuildOre => {
                self.robots.ore += 1;
                self.resource.ore -= blueprint.ore_robot;
            }
            Actions::BuildClay => {
                self.robots.clay += 1;
                self.resource.ore -= blueprint.clay_robot;
            }
            Actions::BuildObsidian => {
                self.robots.obsidian += 1;
                self.resource.ore -= blueprint.obsidian_robot.0;
                self.resource.clay -= blueprint.obsidian_robot.1;
            }
            Actions::BuildGeode => {
                self.robots.geodes += 1;
                self.resource.ore -= blueprint.geode_robot.0;
                self.resource.obsidian -= blueprint.geode_robot.1;
            }
        }
    }

    fn list_possible_actions(
        &self,
        blueprint: &BluePrint,
        previous_action: &Actions,
    ) -> Vec<Actions> {
        let mut options = vec![];
        if self.min_left == 1 {
            return vec![Actions::Noop];
        }

        if self.resource.ore >= blueprint.geode_robot.0
            && self.resource.obsidian >= blueprint.geode_robot.1
        {
            // if possible always build geode node
            return vec![Actions::BuildGeode];
        }

        let max_ore_cost = blueprint
            .clay_robot
            .max(blueprint.obsidian_robot.0)
            .max(blueprint.geode_robot.0);

        // only make robot if below max otherwise this is not usefull
        if self.robots.ore < max_ore_cost && self.resource.ore >= blueprint.ore_robot {
            // check if making robot has a benefit, new robot should be able to make 1 resource
            if self.min_left > (blueprint.ore_robot + 2) {
                // only make robot if we could not make robot last step
                if !(previous_action == &Actions::Noop
                    && (self.resource.ore - self.robots.ore) >= blueprint.ore_robot)
                {
                    options.push(Actions::BuildOre);
                }
            }
        }
        // same only make clay robot if below max
        if self.robots.clay < blueprint.obsidian_robot.1
            && self.resource.ore >= blueprint.clay_robot
            && self.min_left > (blueprint.clay_robot + 2)
        {
            // only make robot if we could not make robot last step

            if !(previous_action == &Actions::Noop
                && (self.resource.ore - self.robots.ore) >= blueprint.clay_robot)
            {
                options.push(Actions::BuildClay);
            }
        }
        if self.robots.obsidian < blueprint.geode_robot.1
            && self.resource.ore >= blueprint.obsidian_robot.0
            && self.resource.clay >= blueprint.obsidian_robot.1
        {
            // only make robot if we could not make robot last step
            if !(previous_action == &Actions::Noop
                && (self.resource.ore - self.robots.ore) >= blueprint.obsidian_robot.0
                && (self.resource.clay - self.robots.clay >= blueprint.obsidian_robot.1))
            {
                options.push(Actions::BuildObsidian);
            }
        }

        // add noop latter to prioritize making a robot
        options.push(Actions::Noop);

        options
    }

    fn simulate_factory(self, blueprint: &BluePrint, previous_action: &Actions) -> u32 {
        if self.min_left == 0 {
            return self.resource.geodes;
        }
        let mut best_geodes = 0;

        for action in self.list_possible_actions(blueprint, previous_action) {
            let mut next_state = self;
            next_state.step(&action, blueprint);

            // branch and bound, to only consider branches that can possible be better
            if next_state.bound(blueprint) > best_geodes {
                best_geodes = best_geodes.max(next_state.simulate_factory(blueprint, &action));
            }
        }

        best_geodes
    }

    fn bound(self, blueprint: &BluePrint) -> u32 {
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
                    if obsidian >= blueprint.geode_robot.1 {
                        (
                            obsidian + rate - blueprint.geode_robot.1,
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

    let scores: Vec<u32> = blueprints
        .par_iter()
        .map(|blueprint| {
            Factory::new(24).simulate_factory(blueprint, &Actions::Noop) * blueprint.id
        })
        .collect();
    let final_score: u32 = scores.into_iter().sum();

    Some(final_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = parse_blueprint(input);

    let score: u32 = blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| Factory::new(32).simulate_factory(blueprint, &Actions::Noop))
        .product();

    Some(score)
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
        assert_eq!(part_two(&input), Some(56 * 62));
    }
}
