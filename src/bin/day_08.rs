use anyhow::{anyhow, Error};
use std::collections::{BTreeSet, HashMap};

fn main() {
    let input = include_str!("../../inputs/day_08.txt");
    println!("Part 1: {}", number_of_easy_digits(input).unwrap());
    println!("Part 2: {}", sum_of_output_values(input).unwrap());
}

fn number_of_easy_digits(input: &str) -> Result<usize, Error> {
    let mut count = 0;
    for line in input.lines() {
        let parts: Vec<_> = line.split(" | ").collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid line: {}", line));
        }
        count += parts[1]
            .split_whitespace()
            .filter(|letters| letters.len() <= 4 || letters.len() == 7)
            .count();
    }
    Ok(count)
}

fn sum_of_output_values(input: &str) -> Result<u64, Error> {
    let mut sum = 0;
    for line in input.lines() {
        sum += output_value(line)?;
    }
    Ok(sum)
}

fn output_value(line: &str) -> Result<u64, Error> {
    let parts: Vec<_> = line.split(" | ").collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid line: {}", line));
    }
    let examples: Vec<_> = parts[0].split_whitespace().collect();
    if examples.len() != 10 {
        return Err(anyhow!("Invalid examples: {}", parts[0]));
    }
    let map = Map::new(&examples)?;
    let digits: Vec<_> = parts[1].split_whitespace().collect();
    if digits.len() != 4 {
        return Err(anyhow!("Invalid number of digits: {}", parts[1]));
    }
    Ok(map.decode(digits[0]) * 1000
        + map.decode(digits[1]) * 100
        + map.decode(digits[2]) * 10
        + map.decode(digits[3]))
}

#[derive(Debug)]
struct Map(HashMap<BTreeSet<char>, u64>);

impl Map {
    fn new(examples: &[&str]) -> Result<Map, Error> {
        let mut map = HashMap::new();
        let mut examples: Vec<BTreeSet<char>> = examples
            .iter()
            .map(|example| example.chars().collect())
            .collect();
        examples.sort_by_key(|example| example.len());
        let one = examples.remove(0);
        let seven = examples.remove(0);
        let four = examples.remove(0);
        let eight = examples.pop().unwrap();
        let (mut five_len, mut six_len): (Vec<_>, Vec<_>) =
            examples.into_iter().partition(|example| example.len() == 5);

        let nine_index = six_len
            .iter()
            .position(|example| example.is_superset(&four))
            .unwrap();
        let nine = six_len.remove(nine_index);
        let zero_index = six_len
            .iter()
            .position(|example| example.is_superset(&one))
            .unwrap();
        let zero = six_len.remove(zero_index);
        let six = six_len.remove(0);
        assert!(six_len.is_empty());

        let three_index = five_len
            .iter()
            .position(|example| example.is_superset(&seven))
            .unwrap();
        let three = five_len.remove(three_index);
        let five_index = five_len
            .iter()
            .position(|example| nine.is_superset(&example))
            .unwrap();
        let five = five_len.remove(five_index);
        let two = five_len.remove(0);
        assert!(five_len.is_empty());

        map.insert(zero, 0);
        map.insert(one, 1);
        map.insert(two, 2);
        map.insert(three, 3);
        map.insert(four, 4);
        map.insert(five, 5);
        map.insert(six, 6);
        map.insert(seven, 7);
        map.insert(eight, 8);
        map.insert(nine, 9);
        Ok(Map(map))
    }

    fn decode(&self, digit: &str) -> u64 {
        let set: BTreeSet<char> = digit.chars().collect();
        self.0[&set]
    }
}

#[test]
fn example() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
    assert_eq!(number_of_easy_digits(input).unwrap(), 26);

    let one_line =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    assert_eq!(output_value(one_line).unwrap(), 5353);
    assert_eq!(sum_of_output_values(input).unwrap(), 61229);
}
