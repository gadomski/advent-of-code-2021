use anyhow::{anyhow, Error};
use std::str::FromStr;

fn main() {
    let input = "target area: x=34..67, y=-215..-186";
    println!("Part 1: {}", highest_possible_position(input).unwrap());
    println!("Part 2: {}", number_of_possible_velocities(input).unwrap());
}

#[derive(Debug)]
struct Map {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

#[derive(Debug)]
struct Simulation {
    hit: bool,
    path: Vec<(i64, i64)>,
}

fn highest_possible_position(input: &str) -> Result<i64, Error> {
    let map: Map = input.parse()?;
    let simulations = map.successful_simulations();
    simulations
        .iter()
        .map(|simulation| simulation.highest_position())
        .max()
        .ok_or_else(|| anyhow!("No successful simulations found"))
}

fn number_of_possible_velocities(input: &str) -> Result<usize, Error> {
    let map: Map = input.parse()?;
    Ok(map.successful_simulations().len())
}

impl Map {
    fn successful_simulations(&self) -> Vec<Simulation> {
        let mut successful_simulations = Vec::new();
        for vx in 1..=(self.max_x + 1) {
            for vy in self.min_y..=(self.min_y.abs()) {
                let simulation = self.simulate(vx, vy);
                if simulation.hit {
                    successful_simulations.push(simulation);
                }
            }
        }
        successful_simulations
    }

    fn simulate(&self, mut vx: i64, mut vy: i64) -> Simulation {
        let mut path = Vec::new();
        let mut x = 0;
        let mut y = 0;
        path.push((x, y));
        loop {
            x += vx;
            y += vy;
            path.push((x, y));
            vx -= vx.signum();
            vy -= 1;
            if y < self.min_y {
                return Simulation { path, hit: false };
            } else if x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y {
                return Simulation { path, hit: true };
            }
        }
    }
}

impl Simulation {
    fn highest_position(&self) -> i64 {
        self.path.iter().cloned().map(|(_, y)| y).max().unwrap()
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Map, Error> {
        let parts: Vec<_> = s.split_whitespace().collect();
        if parts.len() != 4 {
            return Err(anyhow!("Invalid target area description: {}", s));
        }
        let (min_x, max_x) = parse_range(&parts[2][0..parts[2].len() - 1])?;
        let (min_y, max_y) = parse_range(parts[3])?;
        Ok(Map {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
}

fn parse_range(s: &str) -> Result<(i64, i64), Error> {
    let parts: Vec<_> = s.split('=').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid range: {}", s));
    }
    let parts: Vec<_> = parts[1].split("..").collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid range: {}", parts[1]));
    }
    Ok((parts[0].parse()?, parts[1].parse()?))
}

#[test]
fn example() {
    let input = "target area: x=20..30, y=-10..-5";
    assert_eq!(highest_possible_position(input).unwrap(), 45);
    assert_eq!(number_of_possible_velocities(input).unwrap(), 112);
}
