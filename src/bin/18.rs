#[macro_use]
extern crate impl_ops;
use std::{borrow::Borrow, ops};

use ndarray::{Array3, Axis};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
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
        let nums: Vec<isize> = line
            .trim()
            .split(',')
            .filter_map(|a| a.parse::<isize>().ok())
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

    fn sum(self) -> isize {
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

fn create_array(cubes: &Vec<Cube>) -> CubesArray {
    let max_cube = cubes.iter().copied().reduce(|a, b| a.max(&b)).unwrap();
    let mut array = Array3::<i8>::zeros((
        max_cube.x as usize + 1,
        max_cube.y as usize + 1,
        max_cube.z as usize + 1,
    ));

    for cube in cubes {
        array[[cube.x as usize, cube.y as usize, cube.z as usize]] = 1;
    }

    array
}

fn index_in_array(x: usize, y: usize, z: usize, array: &CubesArray) -> bool {
    x >= 0 && x < array.dim().0 && y >= 0 && y < array.dim().1 && z >= 0 && z < array.dim().2
}

fn cube_in_array(cube: &Cube, array: &CubesArray) -> bool {
    cube.x >= 0
        && (cube.x as usize) < array.dim().0
        && cube.y >= 0
        && (cube.y as usize) < array.dim().1
        && cube.z >= 0
        && (cube.z as usize) < array.dim().2
}

fn contained_adjancend_cubes(drop_array: &CubesArray, x: usize, y: usize, z: usize) -> Option<i32> {
    // for every direction check if we find a block
    let mut nr_adjacend_cubes = 0;
    for dir in 0..6 {
        let mut current = Cube {
            x: x as isize,
            y: y as isize,
            z: z as isize,
        };
        let mut first_cube = true;
        loop {
            match dir {
                0 => current.x -= 1,
                1 => current.x += 1,
                2 => current.y -= 1,
                3 => current.y += 1,
                4 => current.z -= 1,
                5 => current.z += 1,
                _ => (),
            };
            // check if index falls in array
            if !cube_in_array(&current, drop_array) {
                return None;
            }

            if drop_array[[current.x as usize, current.y as usize, current.z as usize]] > 0 {
                if first_cube {
                    nr_adjacend_cubes += 1
                }

                //found block
                break;
            }
            first_cube = false;
        }
    }

    // if no exited early block is contained
    return Some(nr_adjacend_cubes);
}

type CubesArray = Array3<i8>;

pub fn part_two(input: &str) -> Option<i32> {
    let cubes = parse_cubes(input);
    let mut nr_sides = get_nr_sides(&cubes);

    let drop_array = create_array(&cubes);
    // dbg!(&drop_array);

    // get sum of inside adjanced squares
    let sides_inside: i32 = drop_array
        .indexed_iter()
        .filter(|(_, cube)| **cube == 0)
        .filter_map(|(idx, _)| contained_adjancend_cubes(&drop_array, idx.0, idx.1, idx.2))
        .sum();
    dbg!(sides_inside);

    // scan every square if it is fully contained
    // for x in 0..drop_array.dim().0 {
    //     for y in 0..drop_array.dim().1 {
    //         let mut last_z = drop_array.dim().2;
    //         for z in 0..drop_array.dim().2 {

    //         }
    //     }
    // }

    // Some((sides_x + sides_y + sides_z) as i32)
    Some(nr_sides - sides_inside)
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
