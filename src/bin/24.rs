use std::{cmp::Ordering, collections::BinaryHeap};

use hashbrown::{HashMap, HashSet};
use num::integer::lcm;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position(i32, i32);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Default, Clone)]
struct Blizzards {
    storms: HashMap<Position, Vec<Direction>>,
}

#[derive(Debug)]
struct Maze {
    blizzards_times: Vec<Blizzards>,
    maze_size: Position,
    start: Position,
    exit: Position,
    repeats_at: i32,
}

impl Default for Maze {
    fn default() -> Self {
        Maze {
            blizzards_times: vec![],
            maze_size: Position(0, 0),
            start: Position(0, 1),
            exit: Position(0, 0),
            repeats_at: 0,
        }
    }
}

impl Blizzards {
    fn add_direction(&mut self, position: Position, dir: Direction) {
        if self.storms.contains_key(&position) {
            self.storms.get_mut(&position).unwrap().push(dir);
        } else {
            self.storms.insert(position, vec![dir]);
        }
    }

    fn next_minute(&self, maze_size: &Position) -> Blizzards {
        let mut new_blizzards: Blizzards = Default::default();

        for (pos, dirs) in &self.storms {
            for dir in dirs {
                // for each blizzard compute next position and wrap around
                let mut next_pos = match dir {
                    Direction::Left => Position(pos.0, pos.1 - 1),
                    Direction::Right => Position(pos.0, pos.1 + 1),
                    Direction::Up => Position(pos.0 - 1, pos.1),
                    Direction::Down => Position(pos.0 + 1, pos.1),
                };
                // wrap next position aroudn to stay in board
                if next_pos.0 == 0 {
                    next_pos.0 = maze_size.0 - 2;
                } else if next_pos.0 == maze_size.0 - 1 {
                    next_pos.0 = 1;
                }
                if next_pos.1 == 0 {
                    next_pos.1 = maze_size.1 - 2;
                } else if next_pos.1 == maze_size.1 - 1 {
                    next_pos.1 = 1;
                }
                new_blizzards.add_direction(next_pos, *dir);
            }
        }

        new_blizzards
    }
}

impl Maze {
    fn precompute_blizzards(&mut self) {
        // precompute list of blizzards to lcm of height and width maze
        for i in 0..self.repeats_at {
            let next_blizzards = self.blizzards_times[i as usize].next_minute(&self.maze_size);
            self.blizzards_times.push(next_blizzards);
        }
    }

    fn position_on_map(&self, pos: &Position) -> bool {
        if pos == &self.start || pos == &self.exit {
            return true;
        }
        // exclude border because there are walls
        !(pos.0 < 1 || pos.1 < 1 || pos.0 > self.maze_size.0 - 2 || pos.1 > self.maze_size.1 - 2)
    }

    fn future_contains_blizzard(&self, minute: i32, pos: &Position) -> bool {
        self.blizzards_times[minute as usize]
            .storms
            .contains_key(pos)
    }

    #[allow(dead_code)]
    fn display(&self, minute: usize) {
        println!("Maze at minute: {}", minute);
        for i in 0..self.maze_size.0 {
            for j in 0..self.maze_size.1 {
                if let Some(dirs) = self.blizzards_times[minute].storms.get(&Position(i, j)) {
                    if dirs.len() == 1 {
                        match dirs[0] {
                            Direction::Left => print!("<"),
                            Direction::Right => print!(">"),
                            Direction::Up => print!("^"),
                            Direction::Down => print!("v"),
                        }
                    } else {
                        print!("{}", dirs.len());
                    }
                } else if i == 0
                    || j == 0
                    || i == (self.maze_size.0 - 1)
                    || j == (self.maze_size.1 - 1)
                {
                    if Position(i, j) == self.start {
                        print!("E");
                    } else if Position(i, j) == self.exit {
                        print!("F");
                    } else {
                        print!("#");
                    }
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
}

fn parse_maze(input: &str) -> Maze {
    let mut maze: Maze = Default::default();
    let mut blizzards: Blizzards = Default::default();
    maze.maze_size.1 = input.lines().last().unwrap().len() as i32;

    for (i, line) in input.lines().enumerate() {
        maze.maze_size.0 += 1;
        for (j, c) in line.char_indices() {
            match c {
                '>' => blizzards.add_direction(Position(i as i32, j as i32), Direction::Right),
                '<' => blizzards.add_direction(Position(i as i32, j as i32), Direction::Left),
                '^' => blizzards.add_direction(Position(i as i32, j as i32), Direction::Up),
                'v' => blizzards.add_direction(Position(i as i32, j as i32), Direction::Down),
                _ => (),
            };
        }
    }

    maze.blizzards_times.push(blizzards);

    // exit is last row, fartest right
    maze.exit = Position(maze.maze_size.0 - 1, maze.maze_size.1 - 2);
    // least common multiple for height and width for when blizzards loop around to same position
    maze.repeats_at = lcm(maze.maze_size.0 - 2, maze.maze_size.1 - 2);

    maze
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: Position,
    heuristic: i32,
    minute: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_total = self.minute + self.heuristic;
        let other_total = other.minute + other.heuristic;
        other_total.cmp(&self_total)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Position {
    fn manhatten_dist(&self, other: &Position) -> i32 {
        (other.0.abs_diff(self.0) + other.1.abs_diff(self.1)) as i32
    }
}

fn a_star_search(maze: &Maze, start_pos: &Position, end_pos: &Position, start_time: i32) -> i32 {
    // apply alpha star search
    // let mut open: Vec<Node> = vec![start];
    let mut open = BinaryHeap::new();
    let mut closed = HashSet::new();

    open.push(Node {
        pos: *start_pos,
        heuristic: start_pos.manhatten_dist(end_pos),
        minute: start_time,
    });
    closed.insert((*start_pos, start_time));

    while let Some(node) = open.pop() {
        if node.pos == *end_pos {
            // dbg!("Found the end!");
            return node.minute;
        }
        let next_minute = node.minute + 1;

        // loop over sucessors
        for dir in &[(0, 1), (1, 0), (0, -1), (-1, 0), (0, 0)] {
            let new_pos = Position(node.pos.0 + dir.0, node.pos.1 + dir.1);
            // check if position is on map and no blizzards there
            let time_looped = next_minute % maze.repeats_at;
            if !maze.position_on_map(&new_pos)
                || maze.future_contains_blizzard(time_looped, &new_pos)
            {
                continue;
            }

            // check if we do not already have a node with a lower value in open or closed
            if closed.insert((new_pos, time_looped)) {
                open.push(Node {
                    pos: new_pos,
                    minute: next_minute,
                    heuristic: node.pos.manhatten_dist(&new_pos),
                });
            }
        }
    }
    dbg!("help");
    0
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut maze = parse_maze(input);
    // dbg!(maze.repeats_at, maze.maze_size);
    maze.precompute_blizzards();

    let steps = a_star_search(&maze, &maze.start, &maze.exit, 0);
    Some(steps)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut maze = parse_maze(input);
    // dbg!(maze.repeats_at, maze.maze_size);
    maze.precompute_blizzards();

    let mut steps = a_star_search(&maze, &maze.start, &maze.exit, 0);
    steps = a_star_search(&maze, &maze.exit, &maze.start, steps);
    steps = a_star_search(&maze, &maze.start, &maze.exit, steps);
    Some(steps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
