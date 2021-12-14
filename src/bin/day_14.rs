use anyhow::{anyhow, Error};
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day_14.txt");
    let mut polymer = Polymer::new(input).unwrap();
    println!("Part 1: {}", polymer.run(10));
    println!("Part 2: {}", polymer.run(40));
}

#[derive(Debug)]
struct Polymer {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Polymer {
    fn new(input: &str) -> Result<Polymer, Error> {
        let mut in_header = true;
        let mut template = Vec::new();
        let mut rules = HashMap::new();
        for line in input.lines() {
            if line.is_empty() {
                in_header = false;
            } else if in_header {
                template.extend(line.chars());
            } else {
                let parts = line.split(" -> ").collect::<Vec<_>>();
                if parts.len() != 2 {
                    return Err(anyhow!("Invalid rule line: {}", line));
                }
                let from = parts[0].chars().collect::<Vec<_>>();
                if from.len() != 2 {
                    return Err(anyhow!("Invalid 'from' part of rule: {}", parts[0]));
                }
                let to = parts[1].chars().collect::<Vec<_>>();
                if to.len() != 1 {
                    return Err(anyhow!("Invalid 'to' part of rule: {}", parts[1]));
                }
                rules.insert((from[0], from[1]), to[0]);
            }
        }
        Ok(Polymer { template, rules })
    }

    fn run(&mut self, times: usize) -> usize {
        let mut chars = HashMap::new();
        for &c in &self.template {
            let entry = chars.entry(c).or_insert(0);
            *entry += 1;
        }
        let mut counts = HashMap::new();
        for (&a, &b) in self.template.iter().zip(self.template.iter().skip(1)) {
            let entry = counts.entry((a, b)).or_insert(0);
            *entry += 1;
        }
        for _ in 0..times {
            let mut new_counts = HashMap::new();
            for (&(a, b), count) in counts.iter() {
                let c = self.rules[&(a, b)];
                let entry = new_counts.entry((a, c)).or_insert(0);
                *entry += count;
                let entry = new_counts.entry((c, b)).or_insert(0);
                *entry += count;
                let entry = chars.entry(c).or_insert(0);
                *entry += count;
            }
            std::mem::swap(&mut counts, &mut new_counts);
        }
        chars.values().max().unwrap() - chars.values().min().unwrap()
    }
}

#[test]
fn example() {
    let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let mut polymer = Polymer::new(input).unwrap();
    assert_eq!(polymer.run(10), 1588);
    assert_eq!(polymer.run(40), 2188189693529);
}
