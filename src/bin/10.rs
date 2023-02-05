pub fn part_one(input: &str) -> Option<i64> {
    let mut signal_strength: i64 = 1;
    let mut total_strength: i64 = 0;
    let mut cycle: i64 = 0;
    for line in input.lines() {
        // println!("{cycle}-{signal_strength}-{total_strength}");
        if line == "noop" {
            cycle += 1;
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                // println!("adding: {cycle}*{signal_strength}={} - {total_strength}", cycle*signal_strength);
                total_strength += signal_strength * cycle;
            }
            continue;
        }
        let (_, right) = line.split_once(' ').unwrap();
        let num = right.parse::<i64>().unwrap_or_default();
        for _ in 0..2 {
            cycle += 1;
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                // println!("adding: {cycle}*{signal_strength}={} - {total_strength}", cycle*signal_strength);
                total_strength += signal_strength * cycle;
            }
        }
        signal_strength += num;
    }

    Some(total_strength)
}

fn print_pixel(s_strength: i64, cycle: i64) {
    if cycle % 40 == 0 && cycle != 0 {
        println!();
    }
    
    // determine if s_strength is close to current pixel drawn
    if ((cycle % 40) - s_strength) <= 1 && ((cycle % 40) - s_strength) >= -1 {
        print!("#");
    } else {
        print!(".")
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut signal_strength: i64 = 1;
    let mut cycle: i64 = 0;
    for line in input.lines() {
        if line == "noop" {
            print_pixel(signal_strength, cycle);
            cycle += 1;
            continue;
        }
        let (_, right) = line.split_once(' ').unwrap();
        let num = right.parse::<i64>().unwrap_or_default();
        for _ in 0..2 {
            print_pixel(signal_strength, cycle);
            cycle += 1;
        }
        signal_strength += num;
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
