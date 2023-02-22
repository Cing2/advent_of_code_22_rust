extern crate ndarray;

use regex::Regex;
// use std::collections::HashSet;
use hashbrown::HashSet;

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

fn manhatten_dist(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn part_one(input: &str) -> Option<i32> {
    let sensors = parse_sensors(input);
    let mut set_positions = HashSet::new();
    let mut beacon_positions = HashSet::new();

    let check_line = if cfg!(test) { 10 } else { 2000000 };

    for (sensor, beacon) in sensors {
        // if beacon is on line add it to set
        if beacon.1 == check_line {
            beacon_positions.insert(beacon);
        }
        let dist_beacon = manhatten_dist(&sensor, &beacon);
        // get closest postion to the line to check, which is the x position on the line
        let line_pos = (sensor.0, check_line);
        let dist_line = manhatten_dist(&sensor, &line_pos);
        if dist_line > dist_beacon {
            // beacon is not close enough to check line
            continue;
        }
        // go over position on the line to add to set
        set_positions.insert(line_pos);
        let mut off_set = 0;
        loop {
            off_set += 1;
            if manhatten_dist(&sensor, &(line_pos.0 + off_set, line_pos.1)) > dist_beacon {
                break;
            }
            // add both sides
            set_positions.insert((line_pos.0 + off_set, line_pos.1));
            set_positions.insert((line_pos.0 - off_set, line_pos.1));
        }
    }
    // dbg!(set_positions.len());
    // dbg!(beacon_positions.len());

    Some((set_positions.len() - beacon_positions.len()) as i32)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sensors = parse_sensors(input);

    // add sensor distances
    let sensors_dist: Vec<((i32, i32), i32)> = sensors
        .iter()
        .map(|(s, b)| (*s, manhatten_dist(s, b)))
        .collect();

    let max_range = if cfg!(test) { 20 } else { 4_000_000 };

    for y in 0..max_range {
        let mut x = 0;
        while x < max_range {
            // check if position is in range of a sensor
            let mut in_range = false;
            for (sensor, dist) in &sensors_dist {
                if manhatten_dist(&(x, y), sensor) <= *dist {
                    in_range = true;
                    // calculate how far we can skip ahead,
                    // start from x position of sensor and distance to the right accounted for height
                    // println!("y={y}: Skipping to {}", sensor.0 + dist - (y-sensor.1).abs());
                    x = sensor.0 + dist - (y - sensor.1).abs();

                    break;
                }
            }
            if !in_range {
                // found place no sensor reaches
                // dbg!("Found it!", y, x);
                return Some(x as i64 * 4_000_000 + y as i64);
            }
            x += 1;
        }
    }

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
        assert_eq!(part_two(&input), Some(56000011));
    }
}
