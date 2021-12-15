use anyhow::{anyhow, Error};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

type Point = (i64, i64);

fn main() {
    let input = include_str!("../../inputs/day_15.txt");
    let mut map = Map::new(input).unwrap();
    let path = map.least_risky_path();
    println!("Part 1: {}", path.risk());
    map.grow(5);
    let path = map.least_risky_path();
    println!("Part 2: {}", path.risk());
}

#[derive(Debug)]
struct Map {
    map: HashMap<Point, i64>,
    end_point: Point,
}

#[derive(Debug, Eq)]
struct Path {
    risk: i64,
    location: Point,
}

impl Map {
    fn new(input: &str) -> Result<Map, Error> {
        let mut map = HashMap::new();
        let mut end_point = (0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                end_point = (x.try_into()?, y.try_into()?);
                map.insert(
                    end_point,
                    c.to_digit(10)
                        .ok_or_else(|| {
                            anyhow!("Could not turn character into base 10 digit: {}", c)
                        })?
                        .try_into()?,
                );
            }
        }
        Ok(Map { map, end_point })
    }

    fn grow(&mut self, times: i64) {
        let original = self.map.clone();
        let x_size = self.end_point.0 + 1;
        let y_size = self.end_point.1 + 1;
        for x in 0..(x_size * times) {
            for y in 0..(y_size * times) {
                let xref = x % x_size;
                let yref = y % y_size;
                let xadd = x / x_size;
                let yadd = y / y_size;
                let risk = original
                    .get(&(xref, yref))
                    .expect("Shouldn't go out-of-bounds here");
                self.map.insert((x, y), ((risk + xadd + yadd - 1) % 9) + 1);
                self.end_point = (x, y);
            }
        }
    }

    fn least_risky_path(&self) -> Path {
        let mut paths = BinaryHeap::new();
        paths.push(Path {
            risk: 0,
            location: (0, 0),
        });
        let mut seen = HashSet::new();
        seen.insert((0, 0));
        loop {
            let path = paths.pop().unwrap();
            if self.path_is_complete(&path) {
                return path;
            } else {
                paths.extend(self.paths(path, &mut seen));
            }
        }
    }

    fn path_is_complete(&self, path: &Path) -> bool {
        path.location == self.end_point
    }

    fn paths(&self, path: Path, seen: &mut HashSet<Point>) -> Vec<Path> {
        path.neighbors()
            .filter_map(|point| {
                if seen.contains(&point) {
                    None
                } else {
                    seen.insert(point);
                    self.map.get(&point).map(|risk| Path {
                        location: point,
                        risk: path.risk + risk,
                    })
                }
            })
            .collect()
    }
}

impl Path {
    fn risk(&self) -> i64 {
        self.risk
    }

    fn neighbors(&self) -> impl Iterator<Item = Point> + '_ {
        [
            (self.location.0 - 1, self.location.1),
            (self.location.0 + 1, self.location.1),
            (self.location.0, self.location.1 - 1),
            (self.location.0, self.location.1 + 1),
        ]
        .into_iter()
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        self.risk == other.risk
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Path) -> Ordering {
        Reverse(self.risk).cmp(&Reverse(other.risk))
    }
}

#[test]
fn example() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let mut map = Map::new(input).unwrap();
    let path = map.least_risky_path();
    assert_eq!(path.risk(), 40);
    map.grow(5);
    let path = map.least_risky_path();
    assert_eq!(path.risk(), 315);
}
