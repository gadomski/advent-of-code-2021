use anyhow::{anyhow, Error};
use std::cmp::Ordering;

fn main() {
    let input = include_str!("../../inputs/day_03.txt");
    let report = Report::new(input).unwrap();
    println!("Part 1: {}", report.power_consumption());
    println!("Part 2: {}", report.life_support_rating());
}

#[derive(Debug)]
struct Report {
    gamma_rate: i64,
    epsilon_rate: i64,
    oxygen_generator_rating: i64,
    co2_scrubber_rating: i64,
}

impl Report {
    fn new(input: &str) -> Result<Report, Error> {
        let report: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        if report.is_empty() {
            return Err(anyhow!("Empty input"));
        }
        let bits_len = report[0].len();
        if !report.iter().all(|bits| bits.len() == bits_len) {
            return Err(anyhow!("Not all lines are the same length: {}", input));
        }
        let mut gamma_rate = Vec::new();
        let mut epsilon_rate = Vec::new();
        let mut oxygen_generator_report = report.clone();
        let mut co2_scrubber_report = report.clone();
        for index in 0..bits_len {
            match check_value_at_index('1', &report, index) {
                Ordering::Greater => {
                    gamma_rate.push('1');
                    epsilon_rate.push('0');
                },
                Ordering::Less => {
                    gamma_rate.push('0');
                    epsilon_rate.push('1');
                }
                Ordering::Equal => return Err(anyhow!("Gamma and epsilon rates are not defined when there is not a most-common value (index={})", index)),
            }
            if oxygen_generator_report.len() > 1 {
                let oxygen_filter_value =
                    match check_value_at_index('1', &oxygen_generator_report, index) {
                        Ordering::Greater | Ordering::Equal => '1',
                        Ordering::Less => '0',
                    };
                oxygen_generator_report.retain(|bits| bits[index] == oxygen_filter_value);
            }
            if co2_scrubber_report.len() > 1 {
                let co2_filter_value = match check_value_at_index('1', &co2_scrubber_report, index)
                {
                    Ordering::Greater | Ordering::Equal => '0',
                    Ordering::Less => '1',
                };
                co2_scrubber_report.retain(|bits| bits[index] == co2_filter_value);
            }
        }
        if oxygen_generator_report.len() != 1 {
            return Err(anyhow!(
                "Expected just one oxygen generator report: {:?}",
                oxygen_generator_report
            ));
        }
        if co2_scrubber_report.len() != 1 {
            return Err(anyhow!(
                "Expected just one co2 scrubber report: {:?}",
                co2_scrubber_report
            ));
        }
        Ok(Report {
            gamma_rate: bits_to_i64(&gamma_rate)?,
            epsilon_rate: bits_to_i64(&epsilon_rate)?,
            oxygen_generator_rating: bits_to_i64(&oxygen_generator_report[0])?,
            co2_scrubber_rating: bits_to_i64(&co2_scrubber_report[0])?,
        })
    }

    fn power_consumption(&self) -> i64 {
        self.epsilon_rate * self.gamma_rate
    }

    fn life_support_rating(&self) -> i64 {
        self.oxygen_generator_rating * self.co2_scrubber_rating
    }
}

fn check_value_at_index(value: char, report: &[Vec<char>], index: usize) -> Ordering {
    let cmp_value = if report.len() % 2 == 0 {
        report.len() / 2
    } else {
        (report.len() + 1) / 2
    };
    report
        .iter()
        .filter(|bits| bits[index] == value)
        .count()
        .cmp(&cmp_value)
}

fn bits_to_i64(bits: &[char]) -> Result<i64, Error> {
    let mut value = 0;
    for index in bits
        .iter()
        .enumerate()
        .filter_map(|(i, &bit)| if bit == '1' { Some(i) } else { None })
    {
        let place = bits.len() - index - 1;
        value += 2i64.pow(place.try_into()?);
    }
    Ok(value)
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
    assert_eq!(report.oxygen_generator_rating, 23);
    assert_eq!(report.co2_scrubber_rating, 10);
    assert_eq!(report.life_support_rating(), 230);
}
