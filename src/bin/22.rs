#[macro_use]
extern crate impl_ops;
use std::ops;

use itertools::enumerate;
use ndarray::prelude::*;

type Maze = Array2<i8>;

fn parse_maze(input: &str) -> Maze {
    let characters: Vec<Vec<char>> = input
        .lines()
        .take_while(|p| !p.is_empty())
        .map(|a| a.chars().collect())
        .collect();
    let longest_line = characters.iter().map(|a| a.len()).max().unwrap();
    let size = (characters.len(), longest_line);
    let mut maze = Array2::<i8>::zeros(size);

    for (i, line) in enumerate(characters) {
        for (j, char) in enumerate(line) {
            match char {
                ' ' => (),
                '.' => maze[[i, j]] = 1,
                '#' => maze[[i, j]] = 2,
                _ => println!("unknown char: {char}"),
            };
        }
    }
    maze
}

#[derive(Debug)]
enum Instruction {
    Steps(i32),
    Direction(Direction),
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let last_line = input.lines().last().unwrap();

    let mut instructions: Vec<Instruction> = vec![];
    let mut last_nums: Vec<char> = vec![];
    for char in last_line.chars() {
        if char == 'R' || char == 'L' {
            let num = last_nums
                .clone()
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            last_nums.clear();
            instructions.push(Instruction::Steps(num));
            match char {
                'R' => instructions.push(Instruction::Direction(Direction::Right)),
                'L' => instructions.push(Instruction::Direction(Direction::Left)),
                _ => (),
            };
        } else {
            last_nums.push(char);
        }
    }
    if !last_nums.is_empty() {
        let num = last_nums
            .clone()
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        instructions.push(Instruction::Steps(num));
    }

    instructions
}

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: i32,
    y: i32,
}

impl_op_ex!(+ |a: &Coords, b: &Coords| -> Coords { Coords { x: a.x + b.x, y: a.y + b.y }});

impl Coords {
    fn coords_in_array(&self, array: &Array2<i8>) -> bool {
        self.x >= 0
            && (self.x as usize) < array.dim().0
            && self.y >= 0
            && (self.y as usize) < array.dim().1
    }

    fn modulo(&self, dim: &Coords) -> Self {
        Self {
            x: self.x % dim.x,
            y: self.y % dim.y,
        }
    }

    fn rotate(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Right => Self {
                x: self.y,
                y: -self.x,
            },
            Direction::Left => Self {
                x: -self.y,
                y: self.x,
            },
        }
    }
}

fn find_position(
    maze: &Maze,
    position: &Coords,
    direction: &Coords,
    maze_size: &Coords,
) -> Option<Coords> {
    let mut new_position: Coords = position.clone();
    // dbg!(new_position, position, direction);

    loop {
        new_position = new_position + direction;
        // dbg!(&new_position);
        if !new_position.coords_in_array(maze) {
            new_position = (new_position + maze_size).modulo(maze_size);
        }

        match maze[[new_position.x as usize, new_position.y as usize]] {
            2 => return None,
            1 => return Some(new_position),
            _ => (),
        }
    }
    None
}

fn print_maze(maze: &Maze) {
    for row in maze.axis_iter(Axis(0)) {
        for element in row.iter() {
            match element {
                0 => print!(" "),
                1 => print!("."),
                2 => print!("#"),
                _ => (),
            }
        }
        print!("\n");
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let maze = parse_maze(input);
    let instructions = parse_instructions(input);

    // do simulation
    let mut position = Coords {
        x: 0,
        y: maze.slice(s![0, ..]).iter().position(|n| n == &1).unwrap() as i32,
    };
    let mut direction = Coords { x: 0, y: 1 };
    let array_size = Coords {
        x: maze.dim().0 as i32,
        y: maze.dim().1 as i32,
    };

    dbg!(position, direction, array_size);

    for instruction in instructions {
        match instruction {
            Instruction::Steps(n) => {
                for _ in 0..n {
                    if let Some(new_pos) = find_position(&maze, &position, &direction, &array_size)
                    {
                        position = new_pos;
                    } else {
                        // is blocked thus can break
                        break;
                    }
                }
            }
            Instruction::Direction(dir) => {
                direction = direction.rotate(&dir);
            }
        }
    }
    let direction_score = match direction {
        Coords { x: 0, y: 1 } => 0,
        Coords { x: 1, y: 0 } => 1,
        Coords { x: 0, y: -1 } => 2,
        Coords { x: -1, y: 0 } => 3,
        _ => todo!(),
    };
    dbg!(direction, direction_score, position);
    let outcome = 1000 * (position.x + 1) + 4 * (position.y + 1) + direction_score;

    Some(outcome)
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
