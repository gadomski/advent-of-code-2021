use anyhow::Error;
use std::i64;

fn main() {
    let input = include_str!("../../inputs/day_07.txt").trim();
    println!("Part 1: {}", least_fuel_to_align(input, false).unwrap());
    println!("Part 2: {}", least_fuel_to_align(input, true).unwrap());
}

fn least_fuel_to_align(input: &str, advanced: bool) -> Result<i64, Error> {
    let mut min = i64::MAX;
    let mut max = i64::MIN;
    let mut positions = Vec::new();
    for result in input.split(',').map(|s| s.parse::<i64>()) {
        let position = result?;
        if position < min {
            min = position;
        } else if position > max {
            max = position;
        }
        positions.push(position);
    }
    let mut min_fuel = i64::MAX;
    for target in min..=max {
        let fuel = if advanced {
            positions.iter().fold(0, |acc, position| {
                let difference = (position - target).abs();
                acc + (1..=difference).sum::<i64>()
            })
        } else {
            positions
                .iter()
                .fold(0, |acc, position| (position - target).abs() + acc)
        };
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    Ok(min_fuel)
}

#[test]
fn example() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(least_fuel_to_align(input, false).unwrap(), 37);
    assert_eq!(least_fuel_to_align(input, true).unwrap(), 168);
}
