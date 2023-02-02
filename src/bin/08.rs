use itertools::Itertools;
use ndarray::{s, Array, ArrayBase, DataMut, Dim, IxDynImpl, OwnedRepr};

fn create_trees_array(input: &str) -> ArrayBase<OwnedRepr<i16>, Dim<[usize; 2]>> {
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
    let trees = Array::from_shape_vec((width, out.len() / width), out).unwrap();
    trees
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = create_trees_array(input);
    let mut mask = Array::<i16, _>::zeros(trees.shape());
    println!("{trees:?}");

    for c in 0..(trees.ncols()) {
        // from top to bottom
        let mut last_tree_height = -1;
        for i in 0..(trees.nrows()) {
            if trees[[i, c]] > last_tree_height {
                mask[[i, c]] = 1;
                last_tree_height = trees[[i, c]];
            } else {
            }
        }
        // from bottom to top
        last_tree_height = -1;
        for i in (0..(trees.nrows())).rev() {
            if trees[[i, c]] > last_tree_height {
                mask[[i, c]] = 1;
                last_tree_height = trees[[i, c]];
            } else {
            }
        }
    }
    for i in 0..(trees.nrows()) {
        // from top to bottom
        let mut last_tree_height = -1;
        for c in 0..(trees.ncols()) {
            if trees[[i, c]] > last_tree_height {
                mask[[i, c]] = 1;
                last_tree_height = trees[[i, c]];
            } else {
            }
        }
        // from bottom to top
        last_tree_height = -1;
        for c in (0..(trees.ncols())).rev() {
            if trees[[i, c]] > last_tree_height {
                mask[[i, c]] = 1;
                last_tree_height = trees[[i, c]];
            } else {
            }
        }
    }

    // println!("{:?}", mask.slice(s![0, ..]));

    println!("{mask:?}");

    Some(mask.sum().try_into().unwrap_or_default())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
