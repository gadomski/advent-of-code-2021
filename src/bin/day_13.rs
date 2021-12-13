use anyhow::{anyhow, Error};
use std::{
    collections::{HashSet, VecDeque},
    fmt,
    str::FromStr,
};

fn main() {
    let input = include_str!("../../inputs/day_13.txt");
    let mut instructions = Instructions::new(input).unwrap();
    instructions.fold_one().unwrap();
    println!("Part 1: {}", instructions.visible_dots());
    instructions.fold();
    println!("Part 2:\n{}", instructions);
}

#[derive(Debug)]
struct Instructions {
    dots: HashSet<Dot>,
    folds: VecDeque<Fold>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Dot {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum Fold {
    Up(i64),
    Left(i64),
}

impl Instructions {
    fn new(input: &str) -> Result<Instructions, Error> {
        let mut dots = HashSet::new();
        let mut folds = VecDeque::new();
        let mut in_header = true;
        for line in input.lines() {
            if line.is_empty() {
                in_header = false;
            } else if in_header {
                dots.insert(line.parse::<Dot>()?);
            } else {
                folds.push_back(line.parse::<Fold>()?);
            }
        }
        Ok(Instructions { dots, folds })
    }

    fn fold(&mut self) {
        while let Some(fold) = self.folds.pop_front() {
            self.execute(fold);
        }
    }

    fn fold_one(&mut self) -> Result<(), Error> {
        if let Some(fold) = self.folds.pop_front() {
            self.execute(fold);
            Ok(())
        } else {
            Err(anyhow!("No more folds!"))
        }
    }

    fn execute(&mut self, fold: Fold) {
        let mut new_dots = HashSet::new();
        for dot in &self.dots {
            new_dots.insert(match fold {
                Fold::Left(x) => {
                    if dot.x > x {
                        Dot {
                            x: 2 * x - dot.x,
                            y: dot.y,
                        }
                    } else {
                        *dot
                    }
                }
                Fold::Up(y) => {
                    if dot.y > y {
                        Dot {
                            x: dot.x,
                            y: 2 * y - dot.y,
                        }
                    } else {
                        *dot
                    }
                }
            });
        }
        self.dots = new_dots;
    }

    fn visible_dots(&self) -> usize {
        self.dots.len()
    }

    fn max(&self) -> (i64, i64) {
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;
        for dot in &self.dots {
            if dot.x > max_x {
                max_x = dot.x;
            }
            if dot.y > max_y {
                max_y = dot.y;
            }
        }
        (max_x, max_y)
    }
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (max_x, max_y) = self.max();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let c = if self.dots.contains(&Dot { x, y }) {
                    '#'
                } else {
                    ' '
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Dot {
    type Err = Error;
    fn from_str(s: &str) -> Result<Dot, Error> {
        let parts = s
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<i64>, _>>()?;
        if parts.len() != 2 {
            Err(anyhow!("Invalid dot: {}", s))
        } else {
            Ok(Dot {
                x: parts[0],
                y: parts[1],
            })
        }
    }
}

impl FromStr for Fold {
    type Err = Error;
    fn from_str(s: &str) -> Result<Fold, Error> {
        let parts: Vec<_> = s.split(' ').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid fold line: {}", s));
        }
        let parts: Vec<_> = parts[2].split('=').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid last fold part: {}", parts[2]));
        }
        match parts[0] {
            "x" => Ok(Fold::Left(parts[1].parse()?)),
            "y" => Ok(Fold::Up(parts[1].parse()?)),
            _ => Err(anyhow!("Invalid fold instruction: {}", parts[0])),
        }
    }
}

#[test]
fn example() {
    let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let mut instructions = Instructions::new(input).unwrap();
    instructions.fold_one().unwrap();
    assert_eq!(instructions.visible_dots(), 17);
}
