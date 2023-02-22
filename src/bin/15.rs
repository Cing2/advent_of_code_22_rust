use std::collections::HashSet;

use ndarray::Array2;
use regex::Regex;

fn parse_sensors(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let re =
        Regex::new(r"Sensor at x=(-*\d+), y=(-*\d+): closest beacon is at x=(-*\d+), y=(-*\d+)")
            .unwrap();
    let sensors: Vec<((i32, i32), (i32, i32))> = re
        .captures_iter(input)
        .map(|cap| {
            let mut nums = vec![];
            for i in 1..5 {
                nums.push(cap.get(i).unwrap().as_str().parse::<i32>().unwrap());
            }
            ((nums[0], nums[1]), (nums[2], nums[3]))
        })
        .collect();

    sensors
}

fn manhatten_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn part_one(input: &str) -> Option<i32> {
    let sensors = parse_sensors(input);
    let mut set_positions = HashSet::new();
    let mut beacon_positions = HashSet::new();

    let check_line = 2000000;
    // let check_line = 10;

    for (sensor, beacon) in sensors {
        // if beacon is on line add it to set
        if beacon.1 == check_line {
            beacon_positions.insert(beacon);
        }
        let dist_beacon = manhatten_dist(sensor, beacon);
        // get closest postion to the line to check, which is the x position on the line
        let line_pos = (sensor.0, check_line);
        let dist_line = manhatten_dist(sensor, line_pos);
        if dist_line > dist_beacon {
            // beacon is not close enough to check line
            continue;
        }
        // go over position on the line to add to set
        set_positions.insert(line_pos);
        let mut off_set = 0;
        loop {
            off_set += 1;
            if manhatten_dist(sensor, (line_pos.0 + off_set, line_pos.1)) > dist_beacon {
                break;
            }
            // add both sides
            set_positions.insert((line_pos.0 + off_set, line_pos.1));
            set_positions.insert((line_pos.0 - off_set, line_pos.1));
        }
    }
    dbg!(set_positions.len());
    dbg!(beacon_positions.len());

    Some((set_positions.len() - beacon_positions.len()) as i32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sensors = parse_sensors(input);

    let max_range = 400000;
    let max_range = 20;

    // create array of sensor ranges
    let mut beacon_converage = Array2::<bool>::new();

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
