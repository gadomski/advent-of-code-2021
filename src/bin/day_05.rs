use anyhow::{anyhow, Error};
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../../inputs/day_05.txt");
    println!("Part 1: {}", number_of_overlaps(input, false).unwrap());
    println!("Part 2: {}", number_of_overlaps(input, true).unwrap());
}

fn number_of_overlaps(input: &str, include_diagonal_lines: bool) -> Result<usize, Error> {
    let mut counts = HashMap::new();
    for line in input.lines().map(|line| line.parse::<Line>()) {
        let line = line?;
        if include_diagonal_lines || line.is_horizontal() || line.is_vertical() {
            for point in line.iter_points() {
                let entry = counts.entry(point).or_insert(0);
                *entry += 1;
            }
        }
    }
    Ok(counts.iter().filter(|(_, &count)| count >= 2).count())
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn iter_points(&self) -> impl Iterator<Item = Point> + '_ {
        use Ordering::*;
        let x_ordering = self.start.x.cmp(&self.end.x);
        let y_ordering = self.start.y.cmp(&self.end.y);
        let x: Vec<_> = match x_ordering {
            Less => (self.start.x..=self.end.x).collect(),
            Equal => match y_ordering {
                Less | Equal => vec![self.start.x; (self.end.y - self.start.y + 1) as usize],
                Greater => vec![self.start.x; (self.start.y - self.end.y + 1) as usize],
            },
            Greater => (self.end.x..=self.start.x).rev().collect(),
        };
        let y: Vec<_> = match y_ordering {
            Less => (self.start.y..=self.end.y).collect(),
            Equal => match x_ordering {
                Less | Equal => vec![self.start.y; (self.end.x - self.start.x + 1) as usize],
                Greater => vec![self.start.y; (self.start.x - self.end.x + 1) as usize],
            },
            Greater => (self.end.y..=self.start.y).rev().collect(),
        };
        x.into_iter().zip(y.into_iter()).map(Point::from)
    }
}

impl FromStr for Line {
    type Err = Error;
    fn from_str(s: &str) -> Result<Line, Error> {
        let parts: Vec<_> = s.split(" -> ").collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid line: {}", s));
        }
        Ok(Line {
            start: parts[0].parse()?,
            end: parts[1].parse()?,
        })
    }
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Point, Error> {
        let parts: Vec<_> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid point: {}", s));
        }
        Ok(Point {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
        })
    }
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Point {
        Point { x, y }
    }
}

#[test]
fn example() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    assert_eq!(number_of_overlaps(input, false).unwrap(), 5);
    assert_eq!(number_of_overlaps(input, true).unwrap(), 12);
}
