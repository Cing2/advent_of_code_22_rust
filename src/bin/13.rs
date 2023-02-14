use std::cmp::Ordering;

use serde_json::Value;

// returns true if to packets are in the right order
fn packets_in_order(packet1: &Value, packet2: &Value) -> Ordering {
    // dbg!(packet1, packet2);

    if packet1.is_number() {
        if packet2.is_number() {
            if packet1.as_i64() < packet2.as_i64() {
                return Ordering::Less;
            } else if packet1.as_i64() > packet2.as_i64() {
                return Ordering::Greater;
            }
        }
        if packet2.is_array() {
            return packets_in_order(&Value::Array(vec![packet1.clone()]), packet2);
        }
    }

    if packet1.is_array() {
        if packet2.is_number() {
            return packets_in_order(packet1, &Value::Array(vec![packet2.clone()]));
        } else if packet2.is_array() {
            let a = packet1.as_array().unwrap();
            let b = packet2.as_array().unwrap();
            for i in 0..(a.len()).max(b.len()) {
                if i >= b.len() {
                    // b is short thus not in order
                    return Ordering::Greater;
                } else if i >= a.len() {
                    return Ordering::Less;
                }

                match packets_in_order(&a[i], &b[i]) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => (),
                    Ordering::Greater => return Ordering::Greater,
                }
            }
        }
    }

    Ordering::Equal
}

pub fn part_one(input: &str) -> Option<i32> {
    let sum_result: i32 = input
        .split("\n\n")
        .enumerate()
        .map(|(i, lines)| -> i32 {
            let (left, right) = lines.split_once('\n').unwrap();
            // dbg!(left, right);
            let a: Value = serde_json::from_str(left).unwrap();
            let b: Value = serde_json::from_str(right).unwrap();

            if Ordering::Less == packets_in_order(&a, &b) {
                return (i + 1).try_into().unwrap();
            }
            0_i32
        })
        .sum();

    Some(sum_result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut packets: Vec<Value> = input
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let dividers: Vec<Value> = vec![
        serde_json::from_str("[[2]]").unwrap(),
        serde_json::from_str("[[6]]").unwrap(),
    ];
    packets.push(dividers[0].clone());
    packets.push(dividers[1].clone());

    packets.sort_by(packets_in_order);
    // dbg!("Sorted", &packets);
    // for p in packets {
    //     dbg!(serde_json::to_string(&p));
    // }

    packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| -> Option<i32> {
            if dividers.contains(p) {
                return Some((i + 1).try_into().unwrap());
            }
            None
        })
        .reduce(|a, b| a * b)

    // None
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
        assert_eq!(part_two(&input), Some(140));
    }
}
