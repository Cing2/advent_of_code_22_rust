#[macro_use]
extern crate impl_ops;
use std::{collections::VecDeque, ops};

use hashbrown::HashSet;
use itertools::Itertools;
use ndarray::Array3;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl_op_ex!(-|a: &Cube, b: &Cube| -> Cube {
    Cube {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl Cube {
    fn from_line(line: &str) -> Option<Cube> {
        let nums: Vec<i32> = line
            .trim()
            .split(',')
            .filter_map(|a| a.parse::<i32>().ok())
            .collect();
        if nums.len() < 3 {
            None
        } else {
            Some(Cube {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            })
        }
    }

    fn from_tuple(idx: (usize, usize, usize)) -> Cube {
        Cube {
            x: idx.0 as i32,
            y: idx.1 as i32,
            z: idx.2 as i32,
        }
    }

    fn list_adjacent_cubes_nr(&self, cube_array: &CubesArray) -> Vec<Cube> {
        let mut output = vec![];
        for dir in 0..6 {
            let mut new_cube = self.clone();
            match dir {
                0 => new_cube.x -= 1,
                1 => new_cube.x += 1,
                2 => new_cube.y -= 1,
                3 => new_cube.y += 1,
                4 => new_cube.z -= 1,
                5 => new_cube.z += 1,
                _ => (),
            };
            if cube_in_array(&new_cube, cube_array) {
                output.push(new_cube);
            }
        }
        output
    }

    fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
    fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn sum(self) -> i32 {
        self.x + self.y + self.z
    }

    fn touch_sides(&self, other: &Cube) -> bool {
        (self - other).abs().sum() == 1
    }
}

fn parse_cubes(input: &str) -> Vec<Cube> {
    input.lines().filter_map(Cube::from_line).collect()
}

fn get_nr_sides(cubes: &Vec<Cube>) -> i32 {
    let mut nr_sides = cubes.len() as i32 * 6;

    for cube in cubes {
        for other in cubes {
            if cube == other {
                break;
            }
            // if two cubes touch sides
            if cube.touch_sides(other) {
                nr_sides -= 2;
            }
        }
    }

    nr_sides
}

pub fn part_one(input: &str) -> Option<i32> {
    let cubes = parse_cubes(input);
    let nr_sides = get_nr_sides(&cubes);
    Some(nr_sides)
}

type CubesArray = Array3<i8>;

fn create_array(cubes: &Vec<Cube>) -> CubesArray {
    let max_cube = cubes.iter().copied().reduce(|a, b| a.max(&b)).unwrap();
    let mut array = Array3::<i8>::zeros((
        max_cube.x as usize + 2,
        max_cube.y as usize + 2,
        max_cube.z as usize + 2,
    ));

    for cube in cubes {
        array[[cube.x as usize, cube.y as usize, cube.z as usize]] = 1;
    }

    array
}

fn cube_in_array(cube: &Cube, array: &CubesArray) -> bool {
    cube.x >= 0
        && (cube.x as usize) < array.dim().0
        && cube.y >= 0
        && (cube.y as usize) < array.dim().1
        && cube.z >= 0
        && (cube.z as usize) < array.dim().2
}

pub fn part_two(input: &str) -> Option<i32> {
    let cubes = parse_cubes(input);
    // let nr_sides = get_nr_sides(&cubes);

    let drop_array = create_array(&cubes);
    // dbg!(&drop_array);

    // do breath first search on start to find all sides that can be found from outside
    let mut queue: VecDeque<Cube> = Default::default();
    let mut visited: HashSet<Cube> = Default::default();

    let first_empty_index: (usize, usize, usize) = drop_array
        .indexed_iter()
        .find_or_first(|(_, cube)| **cube == 0)
        .unwrap()
        .0;
    let start = Cube::from_tuple(first_empty_index);

    queue.push_back(start);
    visited.insert(start);

    let mut nr_sides = 0;

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();

        for other in next.list_adjacent_cubes_nr(&drop_array) {
            //check if full then add sides
            if !visited.contains(&other) {
                if drop_array[[other.x as usize, other.y as usize, other.z as usize]] == 0 {
                    queue.push_back(other);
                    visited.insert(other);
                } else {
                    nr_sides += 1;
                }
            }
        }
    }

    // Some(nr_sides - sides_inside)
    Some(nr_sides)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
