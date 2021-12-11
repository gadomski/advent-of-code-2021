use anyhow::{anyhow, Error};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

fn main() {
    let input = include_str!("../../inputs/day_11.txt");
    let mut map = Map::new(input).unwrap();
    map.step(100);
    println!("Part 1: {}", map.number_of_flashes());
    let mut map = Map::new(input).unwrap();
    println!("Part 2: {}", map.first_step_when_all_flash());
}

#[derive(Debug)]
struct Map {
    map: HashMap<(i64, i64), u32>,
    number_of_flashes: usize,
}

impl Map {
    fn new(input: &str) -> Result<Map, Error> {
        let mut map = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.insert(
                    (x.try_into()?, y.try_into()?),
                    c.to_digit(10)
                        .ok_or_else(|| anyhow!("Unexpected digit: {}", c))?,
                );
            }
        }
        Ok(Map {
            map,
            number_of_flashes: 0,
        })
    }

    fn step(&mut self, times: usize) {
        for _ in 0..times {
            self.step_one();
        }
    }

    fn first_step_when_all_flash(&mut self) -> usize {
        let mut steps = 0;
        let number_of_octopodes = self.map.keys().len();
        loop {
            steps += 1;
            if self.step_one() == number_of_octopodes {
                return steps;
            }
        }
    }

    fn step_one(&mut self) -> usize {
        for value in self.map.values_mut() {
            *value += 1;
        }
        let mut flashes = HashSet::new();
        while self.flash(&mut flashes) {}
        self.number_of_flashes += flashes.len();
        for key in &flashes {
            let value = self.map.get_mut(key).unwrap();
            *value = 0;
        }
        flashes.len()
    }

    fn flash(&mut self, flashes: &mut HashSet<(i64, i64)>) -> bool {
        let mut flashed = false;
        let mut to_increase = Vec::new();
        for (key, value) in self.map.iter() {
            if *value > 9 && flashes.insert(*key) {
                flashed = true;
                to_increase.extend(self.neighbors(*key));
            }
        }
        for key in to_increase {
            let value = self.map.get_mut(&key).unwrap();
            *value += 1;
        }
        flashed
    }

    fn neighbors(&self, (x, y): (i64, i64)) -> impl Iterator<Item = (i64, i64)> + '_ {
        [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .into_iter()
        .filter(|key| self.map.contains_key(key))
    }

    fn number_of_flashes(&self) -> usize {
        self.number_of_flashes
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;
        for &(x, y) in self.map.keys() {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                write!(f, "{}", self.map[&(x, y)])?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn example() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let mut map = Map::new(input).unwrap();
    map.step(100);
    assert_eq!(map.number_of_flashes(), 1656);
    let mut map = Map::new(input).unwrap();
    assert_eq!(map.first_step_when_all_flash(), 195);
}
