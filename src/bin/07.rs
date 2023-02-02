use std::collections::HashMap;

fn create_filesystem(input: &str) -> HashMap<Vec<String>, i32> {
    let mut filesystem: HashMap<Vec<String>, i32> = HashMap::new();
    let mut cwd: Vec<String> = vec![];

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir = line.replace("$ cd ", "");
            if dir == "/" {
                cwd = vec!["/".to_owned()];
            } else if dir == ".." {
                cwd.pop();
            } else {
                cwd.push(dir);
            }
            // println!("{:?}", cwd);
        } else if line.starts_with("dir") || line.starts_with("$ ls") {
        } else {
            //  new file and size
            let (left, _) = line.split_once(' ').unwrap();
            let size = left.parse::<i32>().unwrap_or(0);
            for i in 1..cwd.len() + 1 {
                // println!("{}", i);
                filesystem
                    .entry(cwd[0..i].to_vec())
                    .and_modify(|mana| *mana += size)
                    .or_insert(size);
            }
        }
    }
    filesystem
}

pub fn part_one(input: &str) -> Option<i32> {
    let filesystem: HashMap<Vec<String>, i32> = create_filesystem(input);

    // compute total size of every directory at most size 10000
    let mut total_size = 0;
    for item in filesystem {
        if item.1 <= 100000 {
            total_size += item.1;
        }
    }

    Some(total_size)
}

pub fn part_two(input: &str) -> Option<i32> {
    let filesystem: HashMap<Vec<String>, i32> = create_filesystem(input);

    let space_needed: i32 = 30000000 - (70000000 - filesystem.get(&vec!["/".to_string()]).unwrap_or(&0));
    let mut closest_dir = 70000000;
    for item in filesystem {
        if item.1 > space_needed && item.1 - space_needed < closest_dir - space_needed {
            closest_dir = item.1;
        }
    }

    Some(closest_dir)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
