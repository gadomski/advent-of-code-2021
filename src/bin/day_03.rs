use anyhow::{anyhow, Error};

fn main() {
    let input = include_str!("../../inputs/day_03.txt");
    let report = Report::new(input).unwrap();
    println!("Part 1: {}", report.power_consumption());
}

#[derive(Debug)]
struct Report {
    gamma_rate: i64,
    epsilon_rate: i64,
}

impl Report {
    fn new(input: &str) -> Result<Report, Error> {
        let bits: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        if bits.is_empty() {
            return Err(anyhow!("Empty input"));
        }
        let length = bits[0].len();
        if !bits
            .iter()
            .map(|bits| bits.len())
            .all(|other| other == length)
        {
            return Err(anyhow!("Not all lines are the same length: {}", input));
        }
        let lines = bits.len();
        let mut gamma_rate = 0;
        let mut epsilon_rate = 0;
        for place in 0..length {
            let num_ones = bits
                .iter()
                .filter(|bits| bits[length - place - 1] == '1')
                .count();
            let value = 2i64.pow(place.try_into()?);
            if num_ones >= lines / 2 {
                gamma_rate += value
            } else {
                epsilon_rate += value;
            }
        }
        Ok(Report {
            gamma_rate,
            epsilon_rate,
        })
    }

    fn power_consumption(&self) -> i64 {
        self.epsilon_rate * self.gamma_rate
    }
}

#[test]
fn example() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let report = Report::new(input).unwrap();
    assert_eq!(report.gamma_rate, 22);
    assert_eq!(report.epsilon_rate, 9);
    assert_eq!(report.power_consumption(), 198);
}
