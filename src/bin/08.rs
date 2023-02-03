use std::{iter::Rev, ops::Range};

use itertools::{Either, Itertools};
use ndarray::{s, Array, ArrayBase, Dim, NdIndex, OwnedRepr};

type Ttrees = ArrayBase<OwnedRepr<i16>, Dim<[usize; 2]>>;

fn create_trees_array(input: &str) -> Ttrees {
    // let mut trees = Array::new();
    let out: Vec<i16> = input
        .lines()
        .flat_map(|c| {
            c.chars()
                .filter_map(|a| Some((a as i32 - 0x30) as i16))
                .collect::<Vec<i16>>()
        })
        .collect_vec();
    let width: usize = input
        .lines()
        .take(1)
        .map(|a| a.len())
        .collect::<Vec<usize>>()[0];

    Array::from_shape_vec((width, out.len() / width), out).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees: Ttrees = create_trees_array(input);
    println!("{:?}", trees.is_square());

    let mut mask = Array::<i16, _>::zeros(trees.shape());
    // println!("{trees:?}");

    for direction in 0..4 {
        for i in 0..trees.ncols() {
            // from top to bottom
            let mut last_tree_height = -1;
            for j in 0..trees.nrows() {
                let indices = match direction {
                    0 => [i, j],
                    1 => [i, trees.ncols() - j - 1],
                    2 => [j, i],
                    3 => [trees.ncols() - j - 1, i],
                    _ => panic!("help!"),
                };

                if trees[indices] > last_tree_height {
                    mask[indices] = 1;
                    last_tree_height = trees[indices];
                }
            }
        }
    }
    // println!("{mask:?}");

    Some(mask.sum().try_into().unwrap_or_default())
}

fn calc_scenic_score(trees: &Ttrees, pos: (usize, usize)) -> u32 {
    let mut score = 0;
    let tree_height = trees[pos];
    // row to right
    for j in (pos.1 + 1)..trees.ncols() {
        if tree_height <= trees[[pos.0, j]] || j == trees.ncols() - 1 {
            score += j - pos.1;
            break;
        }
    }
    // row to left
    if pos.1 > 0 {
        for j in (0..(pos.1 - 1)).rev() {
            if tree_height <= trees[[pos.0, j]] || j == 0 {
                score += pos.1 - j;
                break;
            }
        }
    }
    // column to bottom
    for j in (pos.0 + 1)..trees.nrows() {
        if tree_height <= trees[[j, pos.1]] || j == trees.nrows() - 1 {
            score += j - pos.0;
            break;
        }
    }
    // row to top
    if pos.0 > 0 {
        for j in (0..(pos.0 - 1)).rev() {
            if tree_height <= trees[[j, pos.1]] || j == 0 {
                score += pos.0 - j;
                break;
            }
        }
    }

    // println!("{pos:?} - {score}");

    score.try_into().unwrap_or_default()
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees = create_trees_array(input);
    let mut scene_score = Array::<u32, _>::zeros(trees.shape());
    println!("{:?}", trees.shape());

    trees
        .indexed_iter()
        .map(|(pos, _)| calc_scenic_score(&trees, pos))
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
