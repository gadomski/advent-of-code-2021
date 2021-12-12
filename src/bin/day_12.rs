use anyhow::{anyhow, Error};
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/day_12.txt");
    println!("Part 1: {}", paths(input, false).unwrap().len());
    println!("Part 2: {}", paths(input, true).unwrap().len());
}

fn paths(input: &str, part_2: bool) -> Result<Vec<Path>, Error> {
    let caves = caves(input)?;
    let start = caves.get("start").ok_or(anyhow!("No cave named 'start'"))?;
    let path = Path::new(start.name.as_str(), part_2);
    let paths = find_paths(&caves, path);
    Ok(paths)
}

fn caves(input: &str) -> Result<HashMap<String, Cave>, Error> {
    let mut caves = HashMap::new();
    for line in input.lines() {
        let names = line.split('-').collect::<Vec<_>>();
        if names.len() != 2 {
            return Err(anyhow!("Should only be two caves: {}", line));
        }
        for (name, other) in [(names[0], names[1]), (names[1], names[0])].iter() {
            let entry = caves.entry(name.to_string()).or_insert(Cave::new(name));
            entry.connections.push(other.to_string());
        }
    }
    Ok(caves)
}

fn find_paths(caves: &HashMap<String, Cave>, path: Path) -> Vec<Path> {
    let mut new_paths = vec![];
    let last = path.last();
    if last == "end" {
        new_paths.push(path);
    } else {
        for connection in &caves.get(last).unwrap().connections {
            if let Ok(path) = path.with(connection) {
                new_paths.extend(find_paths(caves, path));
            }
        }
    }
    new_paths
}

#[derive(Debug)]
struct Cave {
    name: String,
    connections: Vec<String>,
}

#[derive(Clone, Debug)]
struct Path {
    visited: Vec<String>,
    seen: HashSet<String>,
    part_2: bool,
    has_visited_small_room_twice: bool,
}

impl Cave {
    fn new<S: ToString>(name: S) -> Cave {
        Cave {
            name: name.to_string(),
            connections: Vec::new(),
        }
    }
}

impl Path {
    fn new<S: ToString>(name: S, part_2: bool) -> Path {
        Path {
            visited: vec![name.to_string()],
            seen: HashSet::from([name.to_string()]),
            part_2,
            has_visited_small_room_twice: false,
        }
    }

    fn last(&self) -> &str {
        self.visited.last().unwrap()
    }

    fn with(&self, name: &str) -> Result<Path, Error> {
        if (self.part_2 && self.is_illegal_part_2_path(name))
            || (!self.part_2 && self.is_illegal_part_1_path(name))
        {
            return Err(anyhow!("Illegal: {}", name));
        }
        let mut path = self.clone();
        let seen_before = !path.seen.insert(name.to_string());
        if name.chars().all(|c| c.is_ascii_lowercase()) && seen_before {
            path.has_visited_small_room_twice = true;
        }
        path.visited.push(name.to_string());
        Ok(path)
    }

    fn is_illegal_part_1_path(&self, name: &str) -> bool {
        name.chars().all(|c| c.is_ascii_lowercase()) && self.seen.contains(name)
    }

    fn is_illegal_part_2_path(&self, name: &str) -> bool {
        name == "start"
            || (name.chars().all(|c| c.is_ascii_lowercase())
                && self.seen.contains(name)
                && self.has_visited_small_room_twice)
    }
}

#[test]
fn example() {
    assert_eq!(
        paths(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
            false
        )
        .unwrap()
        .len(),
        10
    );
    assert_eq!(
        paths(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
            true
        )
        .unwrap()
        .len(),
        36
    );
}
