use itertools::Itertools;
use serde_json::Value;



#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Value(i32),
}

fn parse_line(line: &str) -> Packet {
    dbg!(line);
    // dbg!(line[1..(line.len()-1)]);
    // lazy_static! {
    //     static ref RE: Regex = Regex::new("(\[[0-9]+\]),").unwrap();
    // }
    if line.starts_with('[') {
        if line.contains(',') {
            // has number in it
            Packet::List(
                line[1..(line.len() - 1)]
                    .split(',')
                    .map(parse_line)
                    .collect_vec(),
            )
        } else {
            // empty sequence
            Packet::List(vec![])
        }
    } else {
        // should be digit
        Packet::Value(line.parse::<i32>().unwrap())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let v: Value = serde_json::from_str(lines[]).unwrap();
    dbg!(v);

    // let packets: Vec<Packet> = input.lines().filter(|l| l != &"").map(parse_line).collect();
    // dbg!(packets);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
