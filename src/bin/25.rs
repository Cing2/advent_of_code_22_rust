use core::panic;

fn snafu_to_int(input: &str) -> i64 {
    input
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let multiple: i64 = match c {
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                '0' => 0,
                _ => panic!("unknown character: {}", c),
            };
            5i64.pow(i as u32) * multiple
        })
        .sum()
}

fn sumup_snafu(input: &str) -> i64 {
    input.lines().map(|l| snafu_to_int(l)).sum()
}

const SNAFU_LENGTH: usize = 25;
#[derive(Debug, Default)]
struct SnafuNumber {
    multiples: [i32; SNAFU_LENGTH], // multiple for each exp of i
}

impl SnafuNumber {
    fn increase_exp(&mut self, exp: usize, increase: i32) {
        self.multiples[exp] += increase;

        self.rechange_multiple(exp);
    }

    fn rechange_multiple(&mut self, exp: usize) {
        if self.multiples[exp] > 2 {
            self.multiples[exp + 1] += 1;
            self.multiples[exp] -= 5;
            self.rechange_multiple(exp + 1);
        } else if self.multiples[exp] < -2 {
            self.multiples[exp + 1] -= 1;
            self.multiples[exp] += 5;
            self.rechange_multiple(exp + 1);
        }
    }

    fn add_number(&mut self, number: i64) {
        for exp in 0..SNAFU_LENGTH {
            // find lowest exp that is higher
            if 5i64.pow(exp as u32) > number {
                // check how often previous base fits into this one
                let (multiple, remainder) = (
                    number / 5i64.pow((exp - 1) as u32),
                    number % 5i64.pow((exp - 1) as u32),
                );

                match multiple {
                    0 => {}
                    1 => self.increase_exp(exp - 1, 1),
                    2 => self.increase_exp(exp - 1, 2),
                    3 => {
                        self.increase_exp(exp, 1);
                        self.increase_exp(exp - 1, -2)
                    }
                    4 => {
                        self.increase_exp(exp, 1);
                        self.increase_exp(exp - 1, -1)
                    }
                    5 => print!("should not get this"),
                    _ => panic!("hekki"),
                }
                if remainder > 0 {
                    self.add_number(remainder);
                }
                break;
            }
        }
    }

    fn to_string(&self) -> String {
        self.multiples
            .iter()
            .rev()
            .map(|a| match a {
                0 => '0',
                1 => '1',
                2 => '2',
                -1 => '-',
                -2 => '=',
                _ => unreachable!(),
            })
            .collect::<String>()
            .trim_start_matches('0')
            .to_owned()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let sum_fuel = sumup_snafu(input);

    let mut snafu: SnafuNumber = SnafuNumber::default();
    snafu.add_number(sum_fuel);

    Some(snafu.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(String::from("2=-1=0")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
