use anyhow::{anyhow, Error};
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/day_09.txt");
    let map = Map::new(input).unwrap();
    println!("Part 1: {}", map.risk_level());
    println!(
        "Part 2: {}",
        map.three_largest_basin_sizes_multiplied().unwrap()
    );
}

#[derive(Debug)]
struct Map(HashMap<Point, u32>);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Map {
    fn new(input: &str) -> Result<Map, Error> {
        let mut map = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            let y = y.try_into()?;
            for (x, c) in line.chars().enumerate() {
                let x = x.try_into()?;
                map.insert(
                    Point { x, y },
                    c.to_digit(10)
                        .ok_or_else(|| anyhow!("Could not convert {} to a base 10 digit", c))?,
                );
            }
        }
        Ok(Map(map))
    }

    fn risk_level(&self) -> u32 {
        let mut risk_level = 0;
        for (point, &height) in &self.0 {
            if self.is_low_point(height, point) {
                risk_level += height + 1;
            }
        }
        risk_level
    }

    fn is_low_point(&self, height: u32, point: &Point) -> bool {
        self.adjacent(point).iter().all(|&other| other > height)
    }

    fn adjacent(&self, point: &Point) -> Vec<u32> {
        point
            .adjacent()
            .iter()
            .filter_map(|point| self.0.get(point).cloned())
            .collect()
    }

    fn three_largest_basin_sizes_multiplied(&self) -> Result<usize, Error> {
        let mut basins = self.basins();
        if basins.len() < 3 {
            return Err(anyhow!("Too few basins: {}", basins.len()));
        }
        basins.sort_unstable();
        Ok(basins.iter().rev().take(3).product())
    }

    fn basins(&self) -> Vec<usize> {
        let mut basins = Vec::new();
        let mut seen = HashSet::new();
        for (&point, &height) in self.0.iter() {
            if !seen.contains(&point) {
                seen.insert(point);
                if height != 9 {
                    let mut basin = HashSet::new();
                    basin.insert(point);
                    self.grow(point, &mut basin, &mut seen);
                    basins.push(basin);
                }
            }
        }
        basins.into_iter().map(|basin| basin.len()).collect()
    }

    fn grow(&self, point: Point, basin: &mut HashSet<Point>, seen: &mut HashSet<Point>) {
        for point in point.adjacent() {
            if !seen.contains(&point) {
                if let Some(&height) = self.0.get(&point) {
                    seen.insert(point);
                    if height != 9 {
                        basin.insert(point);
                        self.grow(point, basin, seen);
                    }
                }
            }
        }
    }
}

impl Point {
    fn adjacent(&self) -> [Point; 4] {
        [
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

#[test]
fn example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let map = Map::new(input).unwrap();
    assert_eq!(map.risk_level(), 15);
    assert_eq!(map.three_largest_basin_sizes_multiplied().unwrap(), 1134);
}
