use anyhow::Error;
use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/day_06.txt").trim();
    let mut model = Model::new(input).unwrap();
    model.run(80);
    println!("Part 1: {}", model.number_of_fish());
    let mut model = Model::new(input).unwrap();
    model.run(256);
    println!("Part 2: {}", model.number_of_fish());
}

#[derive(Debug)]
struct Model {
    fish: Vec<Fish>,
}

#[derive(Debug)]
struct Fish {
    timer: u8,
    count: usize,
}

impl Model {
    fn new(input: &str) -> Result<Model, Error> {
        let fish = input
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<Fish>, _>>()?;
        Ok(Model { fish })
    }

    fn run(&mut self, times: usize) {
        for _ in 0..times {
            self.run_one();
        }
    }

    fn run_one(&mut self) {
        let mut count = 0;
        for fish in &mut self.fish {
            if fish.timer == 0 {
                fish.timer = 6;
                count += fish.count;
            } else {
                fish.timer -= 1;
            }
        }
        if count > 0 {
            self.fish.push(Fish { timer: 8, count });
        }
    }

    fn number_of_fish(&self) -> usize {
        self.fish.iter().map(|fish| fish.count).sum()
    }
}

impl FromStr for Fish {
    type Err = Error;
    fn from_str(s: &str) -> Result<Fish, Error> {
        Ok(Fish {
            timer: s.parse()?,
            count: 1,
        })
    }
}

#[test]
fn example() {
    let input = "3,4,3,1,2";
    let mut model = Model::new(input).unwrap();
    model.run(18);
    assert_eq!(model.number_of_fish(), 26);
    let mut model = Model::new(input).unwrap();
    model.run(80);
    assert_eq!(model.number_of_fish(), 5934);
    let mut model = Model::new(input).unwrap();
    model.run(256);
    assert_eq!(model.number_of_fish(), 26984457539);
}
