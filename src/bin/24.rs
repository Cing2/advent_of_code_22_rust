use std::cell::RefCell;

use hashbrown::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position(i32, i32);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Default)]
struct Blizzards {
    storms: HashMap<Position, Vec<Direction>>,
}

#[derive(Debug)]
struct Maze {
    blizzards: Blizzards,
    maze_size: Position,
    start: Position,
    exit: Position,
    future_blizzards: RefCell<HashMap<i32, Blizzards>>,
}

impl Default for Maze {
    fn default() -> Self {
        Maze {
            blizzards: Default::default(),
            maze_size: Position(0, 0),
            start: Position(0, 1),
            exit: Position(0, 0),
            future_blizzards: Default::default(),
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

        for blizzards in &self.storms {
            for dir in blizzards.1 {
                // for each blizzard compute next position and wrap around
                let mut next_pos = match dir {
                    Direction::Left => Position(blizzards.0 .0, blizzards.0 .1 - 1),
                    Direction::Right => Position(blizzards.0 .0, blizzards.0 .1 + 1),
                    Direction::Up => Position(blizzards.0 .0 - 1, blizzards.0 .1),
                    Direction::Down => Position(blizzards.0 .0 + 1, blizzards.0 .1),
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
                new_blizzards.add_direction(next_pos, dir.clone());
            }
        }

        new_blizzards
    }
}

impl Maze {
    fn position_on_map(&self, pos: &Position) -> bool {
        if pos == &self.start || pos == &self.exit {
            return true;
        }
        !(pos.0 < 1 || pos.1 < 1 || pos.0 > self.maze_size.0 - 2 || pos.1 > self.maze_size.1)
    }

    fn make_minute(&self, minute: i32) {
        let next_blizzards =  if minute == 1 {
            self.blizzards.next_minute(&self.maze_size)
        } else {
            if !self.future_blizzards.borrow().contains_key(&(minute - 1)) {
                self.make_minute(minute - 1);
            }
    
            self.future_blizzards.borrow()[&(minute - 1)].next_minute(&self.maze_size)
        };

        self.future_blizzards
            .borrow_mut()
            .insert(minute, next_blizzards);
    }

    fn future_contains_blizzard(&self, minute: i32, pos: &Position) -> bool {
        if minute == 0 {
            return self.blizzards.storms.contains_key(pos);
        } else {
            if !self.future_blizzards.borrow().contains_key(&minute) {
                self.make_minute(minute);
            }
            return self.future_blizzards.borrow()[&minute].storms.contains_key(pos);
        }
    }
}

fn parse_maze(input: &str) -> Maze {
    let mut maze: Maze = Default::default();
    maze.maze_size.1 = input.lines().last().unwrap().len() as i32;

    for (i, line) in input.lines().enumerate() {
        maze.maze_size.0 += 1;
        for (j, c) in line.char_indices() {
            match c {
                '>' => maze
                    .blizzards
                    .add_direction(Position(i as i32, j as i32), Direction::Right),
                '<' => maze
                    .blizzards
                    .add_direction(Position(i as i32, j as i32), Direction::Left),
                '^' => maze
                    .blizzards
                    .add_direction(Position(i as i32, j as i32), Direction::Up),
                'v' => maze
                    .blizzards
                    .add_direction(Position(i as i32, j as i32), Direction::Down),
                _ => (),
            };
        }
    }

    // is last row, fartest right
    maze.exit = Position(maze.maze_size.0 - 1, maze.maze_size.1 - 2);

    maze
}

struct Node {
    pos: Position,
    g: i32,
    h: i32,
    minute: i32,
}

fn manhatten_dist(a: Position, b: Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn a_star_search(start: Node, maze: &Maze) -> i32 {
    // apply alpha star search
    let mut open: Vec<Node> = vec![start];
    let mut closed: Vec<Node> = vec![];

    while !open.is_empty() {
        open.sort_by_key(|n| -(n.g + n.h + n.minute));
        let node_f = open.pop().unwrap();
        if node_f.pos == maze.exit {
            // dbg!("Found the end!");
            return node_f.minute;
        }

        // loop over sucessors
        for dir in &[(0, 1), (1, 0), (0, -1), (-1, 0), (0, 0)] {
            let new_pos = Position(node_f.pos.0 + dir.0, node_f.pos.1 + dir.1);
            // check if position is on map and no blizzards there
            if !maze.position_on_map(&new_pos)
                || maze.future_contains_blizzard(node_f.minute+1, &new_pos)
            {
                continue;
            }

            let new_node = Node {
                g: node_f.g + 1,
                h: manhatten_dist(new_pos, maze.exit),
                pos: new_pos,
                minute: node_f.minute + 1,
            };
            // // check if we do not already have a node with a lower value in open or closed
            // let mut is_lowest = true;
            // for n in &open {
            //     if n.pos == new_node.pos && n.g <= new_node.g {
            //         is_lowest = false;
            //     }
            // }
            // for n in &closed {
            //     if n.pos == new_node.pos && n.g <= new_node.g {
            //         is_lowest = false;
            //     }
            // }
            // if !is_lowest {
            //     continue;
            // }
            open.push(new_node);
        }
        closed.push(node_f);
    }
    dbg!("help");
    0
}

pub fn part_one(input: &str) -> Option<i32> {
    let maze = parse_maze(input);
    // println!("{:?}", maze);
    // println!("New: {:?}", maze.next_minute());

    // apply alpha star search
    let start = Node {
        pos: maze.start,
        g: 0,
        h: manhatten_dist(maze.start, maze.exit),
        minute: 0,
    };

    let steps = a_star_search(start, &maze);
    dbg!(steps);
    Some(steps)
}

pub fn part_two(input: &str) -> Option<i32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
