use itertools::{enumerate, Itertools};

fn letter_to_height(c: char) -> i32 {
    if c == 'S' {
        return 0; //a height
    } else if c == 'E' {
        return 25; // z height
    }
    c as i32 - 97
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
                start = (i as i32, j as i32);
            } else if c == 'E' {
                end = (i as i32, j as i32);
            }
        }
    }

    (start, end)
}

#[derive(Debug, Copy, Clone)]
struct Node {
    g: i32, // the movement cost to move from the starting point to a given square on the grid, following the path generated to get there.
    h: i32, // the estimated movement cost to move from that given square on the grid to the final destination.
    // parent: Option<Rc<Node>>,
    pos: (i32, i32),
    height: i32,
}

fn manhatten_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn a_star_search(start: Node, end: (i32, i32), map: &Vec<Vec<i32>>) -> i32 {
    // apply alpha star search
    let mut open: Vec<Node> = vec![start];
    let mut closed: Vec<Node> = vec![];

    while !open.is_empty() {
        open.sort_by_key(|n| -(n.g + n.h));
        let node_f = open.pop().unwrap();
        if node_f.pos == end {
            // dbg!("Found the end!");
            return node_f.g;
        }

        // loop over sucessors
        for dir in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_pos = (node_f.pos.0 + dir.0, node_f.pos.1 + dir.1);
            // check if position is on map
            if new_pos.0 < 0
                || new_pos.0 >= map.len().try_into().unwrap()
                || new_pos.1 < 0
                || new_pos.1 >= map[0].len().try_into().unwrap()
            {
                continue;
            }
            // check if position is not more then 1 heigher
            let height_new = map[new_pos.0 as usize][new_pos.1 as usize];
            if height_new > node_f.height + 1 {
                continue;
            }

            let new_node = Node {
                g: node_f.g + 1,
                h: manhatten_dist(new_pos, end),
                // parent: Some(Rc::new(node_f)),
                pos: new_pos,
                height: height_new,
            };
            // check if we do not already have a node with a lower value in open or closed
            let mut is_lowest = true;
            for n in &open {
                if n.pos == new_node.pos && n.g <= new_node.g {
                    is_lowest = false;
                }
            }
            for n in &closed {
                if n.pos == new_node.pos && n.g <= new_node.g {
                    is_lowest = false;
                }
            }
            if !is_lowest {
                continue;
            }
            open.push(new_node);
        }
        closed.push(node_f);
    }
    0
}

pub fn part_one(input: &str) -> Option<i32> {
    let map = process_input(input);
    let (start, end) = get_start_end(input);

    let start_node = Node {
        g: 0,
        h: manhatten_dist(start, end),
        pos: start,
        height: 0,
    };

    Some(a_star_search(start_node, end, &map))
}

pub fn part_two(input: &str) -> Option<i32> {
    let map = process_input(input);
    let (_, end) = get_start_end(input);

    // apply alpha star search
    let mut open: Vec<Node> = vec![];
    let mut closed: Vec<Node> = vec![];

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                open.push(Node {
                    g: 0,
                    h: manhatten_dist((i as i32, j as i32), end),
                    pos: (i as i32, j as i32),
                    height: 0,
                })
            }
        }
    }

    while !open.is_empty() {
        open.sort_by_key(|n| -(n.g + n.h));
        let node_f = open.pop().unwrap();
        if node_f.pos == end {
            // dbg!("Found the end!");
            return Some(node_f.g);
        }

        // loop over sucessors
        for dir in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_pos = (node_f.pos.0 + dir.0, node_f.pos.1 + dir.1);
            // check if position is on map
            if new_pos.0 < 0
                || new_pos.0 >= map.len().try_into().unwrap()
                || new_pos.1 < 0
                || new_pos.1 >= map[0].len().try_into().unwrap()
            {
                continue;
            }
            // check if position is not more then 1 heigher
            let height_new = map[new_pos.0 as usize][new_pos.1 as usize];
            if height_new > node_f.height + 1 {
                continue;
            }

            let new_node = Node {
                g: node_f.g + 1,
                h: manhatten_dist(new_pos, end),
                // parent: Some(Rc::new(node_f)),
                pos: new_pos,
                height: height_new,
            };
            // check if we do not already have a node with a lower value in open or closed
            let mut is_lowest = true;
            for n in &open {
                if n.pos == new_node.pos && n.g <= new_node.g {
                    is_lowest = false;
                }
            }
            for n in &closed {
                if n.pos == new_node.pos && n.g <= new_node.g {
                    is_lowest = false;
                }
            }
            if !is_lowest {
                continue;
            }
            open.push(new_node);
        }
        closed.push(node_f);
    }
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
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
