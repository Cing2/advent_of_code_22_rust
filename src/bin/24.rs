use hashbrown::HashMap;
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

#[derive(Debug, Default)]
struct Blizzards {
    storms: HashMap<Position, Vec<Direction>>,
}

#[derive(Debug)]
struct Maze {
    blizzards_times: Vec<Blizzards>,
    maze_size: Position,
    start: Position,
    exit: Position,
    lcm: i32,
}

impl Default for Maze {
    fn default() -> Self {
        Maze {
            blizzards_times: vec![],
            maze_size: Position(0, 0),
            start: Position(0, 1),
            exit: Position(0, 0),
            lcm: 0,
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
    fn precompute_blizzards(&mut self) {
        // precompute list of blizzards to lcm of height and width maze
        for i in 0..(self.lcm - 1) {
            let next_blizzards = self.blizzards_times[i as usize].next_minute(&self.maze_size);
            self.blizzards_times.push(next_blizzards);
        }
    }

    fn position_on_map(&self, pos: &Position) -> bool {
        if pos == &self.start || pos == &self.exit {
            return true;
        }
        !(pos.0 < 1 || pos.1 < 1 || pos.0 > self.maze_size.0 - 2 || pos.1 > self.maze_size.1)
    }

    fn future_contains_blizzard(&self, minute: i32, pos: &Position) -> bool {
        self.blizzards_times[minute as usize]
            .storms
            .contains_key(pos)
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
    maze.lcm = lcm(maze.maze_size.0 - 2, maze.maze_size.1 - 2);

    maze
}

struct Node {
    pos: Position,
    g: i32,
    h: i32,
    minute: i32,
}

impl Node {
    fn looped_time(&self, lcm: i32) -> i32 {
         self.minute % lcm
    }
}

fn manhatten_dist(a: Position, b: Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

type NodeList = HashMap<(Position, i32), Node>;

fn a_star_search(start: Node, maze: &Maze) -> i32 {
    // apply alpha star search
    let mut open: Vec<Node> = vec![start];
    let mut closed: NodeList = Default::default();

    while !open.is_empty() {
        open.sort_by_key(|n| -(n.g + n.h + n.minute));
        let node_f = open.pop().unwrap();
        if node_f.pos == maze.exit {
            // dbg!("Found the end!");
            return node_f.minute;
        }
        let time_looped = node_f.looped_time(maze.lcm);

        // loop over sucessors
        for dir in &[(0, 1), (1, 0), (0, -1), (-1, 0), (0, 0)] {
            let new_pos = Position(node_f.pos.0 + dir.0, node_f.pos.1 + dir.1);
            // check if position is on map and no blizzards there
            let new_time = (node_f.minute + 1) % maze.lcm;
            if !maze.position_on_map(&new_pos)
                || maze.future_contains_blizzard(new_time, &new_pos)
            {
                continue;
            }

            let new_node = Node {
                g: node_f.g + 1,
                h: manhatten_dist(new_pos, maze.exit),
                pos: new_pos,
                minute: node_f.minute + 1,
            };
            // check if we do not already have a node with a lower value in open or closed
            let mut is_lowest = true;
            for n in &open {
                if n.pos == new_node.pos && n.looped_time(maze.lcm) == new_time && n.g <= new_node.g {
                    is_lowest = false;
                }
            }
            if let Some(n) = closed.get(&(new_pos, new_time)) {
                if n.g <= new_node.g {
                    is_lowest = false;
                }
            }
            
            if !is_lowest {
                continue;
            }
            open.push(new_node);
        }
        closed.insert((node_f.pos, time_looped),node_f);
    }
    dbg!("help");
    0
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut maze = parse_maze(input);
    // println!("{:?}", maze);
    // println!("New: {:?}", maze.next_minute());
    dbg!(maze.lcm, maze.maze_size);
    maze.precompute_blizzards();


    println!("Starting a star");

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
