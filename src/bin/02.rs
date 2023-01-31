fn rock_paper_scissors(a: &str, b: &str) -> u32 {
    let mut score: u32 = match b {
        "X" => 1, // rock
        "Y" => 2, // paper
        "Z" => 3, // scissor
        _ => 0,
    };
    score += match b {
        "X" => match a {
            // rock
            "A" => 3,
            "C" => 6,
            _ => 0,
        },
        "Y" => match a {
            // paper
            "A" => 6,
            "B" => 3,
            _ => 0,
        },
        "Z" => match a {
            // scissor
            "B" => 6,
            "C" => 3,
            _ => 0,
        },
        _ => 0,
    };

    score
}

fn rps_determined(a: &str, b: &str) -> u32 {
    let mut score = match b {
        "X"=> 0,
        "Y"=> 3,
        "Z"=> 6,
        _=> 0,
    };
    score +=    match (a, b) {
        ("A", "X") | ("B", "Z") | ("C", "Y") => 3, // you playing scissor
        ("A", "Y") | ("B", "X") | ("C", "Z") => 1, // you playing rock
        ("A", "Z") | ("B", "Y") | ("C", "X") => 2, // you playing paper
        _ => 0,
    };
    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let output: Vec<Vec<&str>> = input
        .lines()
        .map(|l: &str| l.split(' ').collect())
        .collect();
    let mut score: u32 = 0;
    for game in output {
        score += rock_paper_scissors(game[0], game[1]);
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let output: Vec<Vec<&str>> = input
        .lines()
        .map(|l: &str| l.split(' ').collect())
        .collect();
    let mut score: u32 = 0;
    for game in output {
        score += rps_determined(game[0], game[1]);
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
