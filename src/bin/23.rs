#[macro_use]
extern crate impl_ops;

use std::{cell::Cell, collections::VecDeque, ops};

use hashbrown::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl Position {
    fn max(&self, other: &Self) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }
    fn min(&self, other: &Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }
}

impl_op_ex!(+ |a: &Position, b: &Position| -> Position { Position(a.0+b.0, a.1+b.1)});

type ElvesPositions = HashMap<Position, Position>;
// type ElvesPosMap = HashMap<Position, Position>;

fn parse_elves_position(input: &str) -> ElvesPositions {
    let postitions = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter(|(_, a)| a == &'#')
                .map(|(j, _)| Position(i as i32, j as i32))
                .collect::<Vec<Position>>()
        })
        .collect::<Vec<Position>>();

    let mut output: ElvesPositions = Default::default();
    for pos in postitions {
        output.insert(pos, pos);
    }

    output
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn positions_around(&self) -> Vec<Position> {
        match self {
            Direction::North => vec![Position(-1, -1), Position(-1, 0), Position(-1, 1)],
            Direction::South => vec![Position(1, -1), Position(1, 0), Position(1, 1)],
            Direction::West => vec![Position(-1, -1), Position(0, -1), Position(1, -1)],
            Direction::East => vec![Position(-1, 1), Position(0, 1), Position(1, 1)],
        }
    }

    fn get_position(&self) -> Position {
        match self {
            Direction::North => Position(-1, 0),
            Direction::South => Position(1, 0),
            Direction::West => Position(0, -1),
            Direction::East => Position(0, 1),
        }
    }
}

fn round_elve_positions(positions: ElvesPositions, iteration: i32) -> (ElvesPositions, bool) {
    // first predict new position of each elf
    let mut new_positions: ElvesPositions = Default::default();
    let mut duplicates = vec![];

    let mut changed = false;

    for pos in positions.keys() {
        // check if any elf around
        let mut elves_around: VecDeque<(Direction, Cell<bool>)> = vec![
            (Direction::North, Cell::new(false)),
            (Direction::South, Cell::new(false)),
            (Direction::West, Cell::new(false)),
            (Direction::East, Cell::new(false)),
        ]
        .into();
        for _ in 0..iteration {
            let elem = elves_around.pop_front().unwrap();
            elves_around.push_back(elem);
        }

        for (dirs, value) in &elves_around {
            for dir in dirs.positions_around() {
                let new_pos = pos + dir;
                if positions.contains_key(&new_pos) {
                    value.set(true);
                    break;
                }
            }
        }
        if elves_around.iter().all(|a| !a.1.get()) {
            // if no elves around do not move
            new_positions.insert(*pos, *pos);
            continue;
        }

        // consider each direction if it is valid
        let mut pos_added = false;
        for (dir, value) in &elves_around {
            if value.get() {
                continue;
            }
            let new_pos = pos + dir.get_position();

            if new_positions.contains_key(&new_pos) {
                // if already contained in new_position remove it from list and from new pos
                duplicates.push(new_pos);
                // insert value twice to make previous one keep old position
                new_positions.insert(new_positions[&new_pos], new_positions[&new_pos]);
                new_positions.remove(&new_pos);
                // keep old position
                new_positions.insert(*pos, *pos);
            } else if duplicates.contains(&new_pos) {
                // keep old position
                new_positions.insert(*pos, *pos);
            } else {
                new_positions.insert(new_pos, *pos);
                changed = true;
            }
            pos_added = true;
            break;
        }
        if !pos_added {
            // keep old position
            new_positions.insert(*pos, *pos);
        }
    }

    (new_positions, changed)
}

#[allow(dead_code)]
fn print_maze(positions: &ElvesPositions) {
    let min_size = positions.keys().copied().reduce(|a, b| a.min(&b)).unwrap();
    let max_size = positions.keys().copied().reduce(|a, b| a.max(&b)).unwrap();
    for i in min_size.0..(max_size.0 + 1) {
        for j in min_size.1..(max_size.1 + 1) {
            let pos = Position(i, j);
            if positions.contains_key(&pos) {
                // dbg!(pos);
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut positions: ElvesPositions = parse_elves_position(input);

    for i in 0..10 {
        (positions, _) = round_elve_positions(positions, i);
    }

    // get size grid
    let min_size = positions.keys().copied().reduce(|a, b| a.min(&b)).unwrap();
    let max_size = positions.keys().copied().reduce(|a, b| a.max(&b)).unwrap();
    let size_square = (max_size.0 - min_size.0 + 1) * (max_size.1 - min_size.1 + 1);
    let empty_squares = size_square - positions.len() as i32;

    Some(empty_squares)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut positions: ElvesPositions = parse_elves_position(input);

    #[allow(unused_assignments)]
    let mut changed = true;
    for i in 0..10000 {
        (positions, changed) = round_elve_positions(positions, i);

        if !changed {
            return Some(i + 1);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
