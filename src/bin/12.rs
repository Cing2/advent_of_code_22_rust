use std::{collections::HashMap, iter::Enumerate};

use itertools::{enumerate, zip, Itertools};

fn letter_to_height(c: char) -> i32 {
    if c == 'S' {
        return 0; //a height
    } else if c == 'E' {
        return 25; // z height
    }
    return c as i32 - 97;
}

fn process_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.chars().map(letter_to_height).collect_vec())
        .collect_vec()
}

fn get_start_end(input: &str) -> ((i32, i32), (i32, i32)) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in enumerate(input.lines()) {
        for (j, c) in line.char_indices() {
            if c == 'S' {
                start = (i, j);
            } else if c == 'E' {
                end = (i, j);
            }
        }
    }

    (start, end)
}

#[derive(Debug)]
struct Node<'a> {
    g: i32, // the movement cost to move from the starting point to a given square on the grid, following the path generated to get there.
    h: i32, // the estimated movement cost to move from that given square on the grid to the final destination.
    parent: Option<&'a Node<'a>>,
    pos: (i32, i32),
    height: i32,
}

fn manhatten_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = process_input(input);
    // dbg!(map);
    let (start, end) = get_start_end(input);

    // apply alpha star search
    let mut open: Vec<Node> = vec![Node {
        g: 0,
        h: manhatten_dist(start, end),
        parent: None,
        pos: start,
        height: 0,
    }];
    let mut closed: Vec<Node> = vec![];

    while !open.is_empty() {
        open.sort_by_key(|n| n.g + n.h);
        let node_f = open.pop().unwrap();

        // loop over sucessors
        for dir in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_pos = (node_f.pos.0 + dir.0, node_f.pos.1 + dir.1);
            let height_new = map[new_pos.0 as usize][new_pos.1 as usize];
            // check if position is on map
            if new_pos.0 < 0
                || new_pos.0 >= map.len().try_into().unwrap()
                || new_pos.1 < 0
                || new_pos.1 >= map.len().try_into().unwrap()
                || height_new > node_f.height + 1
            // check if position is not more then 1 heigher
            {
                break;
            }
            let new_node = Node {
                g: node_f.g + 1,
                h: manhatten_dist(new_pos, end),
                parent: Some(&node_f),
                pos: new_pos,
                height: height_new,
            };
            dbg!(new_node);
            // check if we do not already have a node with a lower value in open
            let mut is_lowest = true;
            for n in open {
                if n.pos == new_node.pos && (n.g + n.h) < (new_node.g + new_node.h) {
                    is_lowest = false;
                }
            }
            for n in closed {
                if n.pos == new_node.pos && (n.g + n.h) < (new_node.g + new_node.h) {
                    is_lowest = false;
                }
            }
            if !is_lowest {
                break;
            }
            open.push(new_node);

        }
        closed.push(node_f);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
