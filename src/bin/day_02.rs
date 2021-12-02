use anyhow::{anyhow, Error};
use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/day_02.txt");
    let mut submarine = BasicSubmarine::default();
    submarine.execute(input).unwrap();
    println!("Part 1: {}", submarine.multiplied_position());
    let mut submarine = AimedSubmarine::default();
    submarine.execute(input).unwrap();
    println!("Part 2: {}", submarine.multiplied_position());
}

#[derive(Debug, Default)]
struct BasicSubmarine {
    horizontal_position: i64,
    depth: i64,
}

#[derive(Debug, Default)]
struct AimedSubmarine {
    horizontal_position: i64,
    depth: i64,
    aim: i64,
}

trait Submarine {
    fn execute(&mut self, input: &str) -> Result<(), Error> {
        for line in input.lines() {
            let instruction = line.parse()?;
            self.execute_instruction(&instruction);
        }
        Ok(())
    }

    fn execute_instruction(&mut self, instruction: &Instruction);
    fn multiplied_position(&self) -> i64;
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: i64,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

impl Submarine for BasicSubmarine {
    fn execute_instruction(&mut self, instruction: &Instruction) {
        use Direction::*;
        match instruction.direction {
            Up => self.depth -= instruction.amount,
            Down => self.depth += instruction.amount,
            Forward => self.horizontal_position += instruction.amount,
        }
    }

    fn multiplied_position(&self) -> i64 {
        self.depth * self.horizontal_position
    }
}

impl Submarine for AimedSubmarine {
    fn execute_instruction(&mut self, instruction: &Instruction) {
        use Direction::*;
        match instruction.direction {
            Up => self.aim -= instruction.amount,
            Down => self.aim += instruction.amount,
            Forward => {
                self.horizontal_position += instruction.amount;
                self.depth += self.aim * instruction.amount;
            }
        }
    }

    fn multiplied_position(&self) -> i64 {
        self.horizontal_position * self.depth
    }
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Instruction, Error> {
        let words: Vec<_> = s.split(' ').collect();
        if words.len() != 2 {
            Err(anyhow!("Invalid number of words (expected 2): {}", s))
        } else {
            let direction = words[0].parse()?;
            let amount = words[1].parse()?;
            Ok(Instruction { direction, amount })
        }
    }
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Direction, Error> {
        use Direction::*;
        match s {
            "forward" => Ok(Forward),
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(anyhow!("Invalid direction: {}", s)),
        }
    }
}

#[test]
fn example() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let mut submarine = BasicSubmarine::default();
    submarine.execute(input).unwrap();
    assert_eq!(submarine.horizontal_position, 15);
    assert_eq!(submarine.depth, 10);
    assert_eq!(submarine.multiplied_position(), 150);

    let mut submarine = AimedSubmarine::default();
    submarine.execute(input).unwrap();
    assert_eq!(submarine.horizontal_position, 15);
    assert_eq!(submarine.depth, 60);
    assert_eq!(submarine.multiplied_position(), 900);
}
