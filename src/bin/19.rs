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
        let re = Regex::new(r"Blueprint .: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

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

#[derive(Debug, Clone, Copy)]
struct Robots {
    nr_ore_robots: u32,
    nr_clay_robots: u32,
    nr_obsidian_robots: u32,
    nr_geode_robots: u32,
}

impl Default for Robots {
    fn default() -> Self {
        Self {
            nr_ore_robots: 1,
            nr_clay_robots: 0,
            nr_obsidian_robots: 0,
            nr_geode_robots: 0,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

#[derive(Debug, Clone, Copy)]
struct Factory {
    robots: Robots,
    resource: Resources,
    blueprint: BluePrint,
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
    fn new(blueprint: BluePrint) -> Self {
        Self {
            robots: Default::default(),
            resource: Default::default(),
            blueprint,
        }
    }

    fn step(&mut self, action: &Actions) {
        // perform a minute step
        // add new ore

        self.resource.ore += self.robots.nr_ore_robots;
        self.resource.clay += self.robots.nr_clay_robots;
        self.resource.obsidian += self.robots.nr_obsidian_robots;
        self.resource.geodes += self.robots.nr_geode_robots;

        // println!("{:?}- {:?}", &self.robots, &self.resource);
        // build robot if action
        match action {
            Actions::NOOP => (),
            Actions::BuildOre => {
                self.robots.nr_ore_robots += 1;
                self.resource.ore -= self.blueprint.ore_robot;
            }
            Actions::BuildClay => {
                self.robots.nr_clay_robots += 1;
                self.resource.ore -= self.blueprint.clay_robot;
            }
            Actions::BuildObsidian => {
                self.robots.nr_obsidian_robots += 1;
                self.resource.ore -= self.blueprint.obsidian_robot.0;
                self.resource.clay -= self.blueprint.obsidian_robot.1;
            }
            Actions::BuildGeode => {
                self.robots.nr_geode_robots += 1;
                self.resource.ore -= self.blueprint.geode_robot.0;
                self.resource.obsidian -= self.blueprint.geode_robot.1;
            }
        }
    }

    fn revert_action(&mut self, action: &Actions) {
        // add new ore
        // println!("Revert: {:?}- {:?}", &self.robots, &self.resource);

        // build robot if action
        match action {
            Actions::NOOP => (),
            Actions::BuildOre => {
                self.robots.nr_ore_robots -= 1;
                self.resource.ore += self.blueprint.ore_robot;
            }
            Actions::BuildClay => {
                self.robots.nr_clay_robots -= 1;
                self.resource.ore += self.blueprint.clay_robot;
            }
            Actions::BuildObsidian => {
                self.robots.nr_obsidian_robots -= 1;
                self.resource.ore += self.blueprint.obsidian_robot.0;
                self.resource.clay += self.blueprint.obsidian_robot.1;
            }
            Actions::BuildGeode => {
                self.robots.nr_geode_robots -= 1;
                self.resource.ore += self.blueprint.geode_robot.0;
                self.resource.obsidian += self.blueprint.geode_robot.1;
            }
        }

        self.resource.ore -= self.robots.nr_ore_robots;
        self.resource.clay -= self.robots.nr_clay_robots;
        self.resource.obsidian -= self.robots.nr_obsidian_robots;
        self.resource.geodes -= self.robots.nr_geode_robots;
    }

    fn list_possible_actions(&self, min_left: u32) -> Vec<Actions> {
        let mut options = vec![Actions::NOOP];
        if min_left == 1 {
            return options;
        }

        if self.resource.ore >= self.blueprint.ore_robot {
            // check if making robot has a benefit
            if min_left > (self.blueprint.ore_robot + 2) {
                // new robot should be able to make 1 robot
                options.push(Actions::BuildOre);
            }
        }
        if self.resource.ore >= self.blueprint.clay_robot {
            if min_left > (self.blueprint.clay_robot + 2) {
                options.push(Actions::BuildClay);
            }
        }
        if self.resource.ore >= self.blueprint.obsidian_robot.0
            && self.resource.clay >= self.blueprint.obsidian_robot.1
        {

            options.push(Actions::BuildObsidian);
        }
        if self.resource.ore >= self.blueprint.geode_robot.0
            && self.resource.obsidian >= self.blueprint.geode_robot.1
        {
            return vec![Actions::BuildGeode];
        }

        options
    }

    fn simulate_factory(&mut self, min_left: u32) -> u32 {
        if min_left == 0 {
            return self.resource.geodes;
        }
        let mut geodes = 0;

        for action in self.list_possible_actions(min_left) {
            // if action == Actions::BuildGeode {
            //     dbg!(min_left, &action);
            // }
            self.step(&action);
            geodes = geodes.max(self.simulate_factory(min_left - 1));

            self.revert_action(&action);
        }

        geodes
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_blueprint(input);
    dbg!(&blueprints);
    let mut factories: Vec<Factory> = blueprints.iter().copied().map(Factory::new).collect();
    let score = factories[0].simulate_factory(24);
    dbg!(score);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
