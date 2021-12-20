use anyhow::{anyhow, Error};
use std::{fmt, iter::Peekable, ops::Add, str::FromStr};

type OptionalI64Pair = (Option<i64>, Option<i64>);

fn main() {
    let input = include_str!("../../inputs/day_18.txt");
    println!("Part 1: {}", magnitude(input).unwrap());
    println!("Part 2: {}", largest_magnitude(input).unwrap());
}

fn largest_magnitude(input: &str) -> Result<i64, Error> {
    let numbers = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Number>, Error>>()?;
    let mut largest_magnitude = i64::MIN;
    for (i, a) in numbers.iter().enumerate() {
        for b in numbers.iter().skip(i + 1) {
            for (a, b) in [(a, b), (b, a)].into_iter() {
                let mut sum = a.clone() + b.clone();
                sum.reduce()?;
                let magnitude = sum.magnitude();
                if magnitude > largest_magnitude {
                    largest_magnitude = magnitude;
                }
            }
        }
    }
    Ok(largest_magnitude)
}

fn magnitude(input: &str) -> Result<i64, Error> {
    let number = sum(input)?;
    Ok(number.magnitude())
}

fn sum(input: &str) -> Result<Number, Error> {
    let mut results = input.lines().map(|line| line.parse::<Number>());
    let mut number = results.next().ok_or_else(|| anyhow!("Empy input"))??;
    for result in results {
        let rhs = result?;
        number = number + rhs;
        number.reduce()?;
    }
    Ok(number)
}

#[derive(Debug, PartialEq, Clone)]
struct Number {
    x: Element,
    y: Element,
}

#[derive(Debug, PartialEq, Clone)]
enum Element {
    I64(i64),
    Number(Box<Number>),
}

#[derive(Debug)]
struct Reader<I>(I);

impl Number {
    fn reduce(&mut self) -> Result<(), Error> {
        while self.explode()? || self.split()? {}
        Ok(())
    }

    fn explode(&mut self) -> Result<bool, Error> {
        self.explode_at_level(0).map(|option| option.is_some())
    }

    fn explode_at_level(&mut self, level: usize) -> Result<Option<OptionalI64Pair>, Error> {
        if let Some((x, mut y)) = self.x.explode_at_level(level)? {
            if let Some(n) = y {
                y = self.y.add_right(n);
            }
            Ok(Some((x, y)))
        } else if let Some((mut x, y)) = self.y.explode_at_level(level)? {
            if let Some(n) = x {
                x = self.x.add_left(n);
            }
            Ok(Some((x, y)))
        } else {
            Ok(None)
        }
    }

    fn split(&mut self) -> Result<bool, Error> {
        Ok(self.x.split()? || self.y.split()?)
    }

    fn magnitude(&self) -> i64 {
        3 * self.x.magnitude() + 2 * self.y.magnitude()
    }
}

impl FromStr for Number {
    type Err = Error;
    fn from_str(s: &str) -> Result<Number, Error> {
        let mut reader = Reader(s.chars().peekable());
        reader.read_number()
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        Number {
            x: Element::Number(Box::new(self)),
            y: Element::Number(Box::new(rhs)),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Element {
    fn as_i64(&self) -> Result<i64, Error> {
        match self {
            Element::I64(n) => Ok(*n),
            Element::Number(number) => Err(anyhow!("Expected i64, found number: {:?}", number)),
        }
    }

    fn split(&mut self) -> Result<bool, Error> {
        match self {
            Element::I64(n) => {
                if *n >= 10 {
                    let new_n = *n / 2;
                    *self = Element::Number(Box::new(Number {
                        x: Element::I64(new_n),
                        y: Element::I64(new_n + *n % 2),
                    }));
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Element::Number(number) => number.split(),
        }
    }

    fn explode_at_level(&mut self, level: usize) -> Result<Option<OptionalI64Pair>, Error> {
        match self {
            Element::Number(number) => {
                if level == 3 {
                    let x = number.x.as_i64()?;
                    let y = number.y.as_i64()?;
                    *self = Element::I64(0);
                    Ok(Some((Some(x), Some(y))))
                } else {
                    number.explode_at_level(level + 1)
                }
            }
            Element::I64(_) => Ok(None),
        }
    }

    fn add_right(&mut self, n: i64) -> Option<i64> {
        match self {
            Element::I64(element) => {
                *element += n;
                None
            }
            Element::Number(number) => {
                let result = number.x.add_right(n);
                if let Some(n) = result {
                    number.y.add_right(n)
                } else {
                    None
                }
            }
        }
    }

    fn add_left(&mut self, n: i64) -> Option<i64> {
        match self {
            Element::I64(element) => {
                *element += n;
                None
            }
            Element::Number(number) => {
                let result = number.y.add_left(n);
                if let Some(n) = result {
                    number.x.add_left(n)
                } else {
                    None
                }
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Element::I64(n) => *n,
            Element::Number(number) => number.magnitude(),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::I64(n) => write!(f, "{}", n),
            Element::Number(number) => write!(f, "{}", number),
        }
    }
}

impl<I: Iterator<Item = char>> Reader<Peekable<I>> {
    fn read_number(&mut self) -> Result<Number, Error> {
        self.expect('[')?;
        let x = self.read_element()?;
        self.expect(',')?;
        let y = self.read_element()?;
        self.expect(']')?;
        Ok(Number { x, y })
    }

    fn read_element(&mut self) -> Result<Element, Error> {
        if let Some(&c) = self.0.peek() {
            if c == '[' {
                Ok(Element::Number(Box::new(self.read_number()?)))
            } else {
                Ok(Element::I64(self.read_i64()?))
            }
        } else {
            Err(anyhow!("Unexpected end of stream"))
        }
    }

    fn read_i64(&mut self) -> Result<i64, Error> {
        let mut chars = Vec::new();
        loop {
            if let Some(c) = self.0.peek() {
                if ('0'..='9').contains(c) {
                    chars.push(self.read_one()?);
                } else if *c == ',' || *c == ']' {
                    break;
                } else {
                    return Err(anyhow!(
                        "Unexpected character: expected digit, comma, or close bracket, got '{}'",
                        self.read_one()?
                    ));
                }
            }
        }
        let s: String = chars.into_iter().collect();
        s.parse().map_err(Error::from)
    }

    fn expect(&mut self, expected: char) -> Result<(), Error> {
        let c = self.read_one()?;
        if c != expected {
            Err(anyhow!(
                "Unexpected character: expected='{}', got='{}'",
                expected,
                c
            ))
        } else {
            Ok(())
        }
    }
    fn read_one(&mut self) -> Result<char, Error> {
        self.0
            .next()
            .ok_or_else(|| anyhow!("Unexpected end of stream"))
    }
}

#[test]
fn example() {
    assert_eq!(
        "[1,2]".parse::<Number>().unwrap() + "[[3,4],5]".parse::<Number>().unwrap(),
        "[[1,2],[[3,4],5]]".parse::<Number>().unwrap()
    );

    let mut number = Number::from_str("[[[[[9,8],1],2],3],4]").unwrap();
    assert!(number.explode().unwrap());
    assert_eq!(number, "[[[[0,9],2],3],4]".parse().unwrap());
    let mut number = Number::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap();
    assert!(number.explode().unwrap());
    assert_eq!(number, "[7,[6,[5,[7,0]]]]".parse().unwrap());
    let mut number = Number::from_str("[[6,[5,[4,[3,2]]]],1]").unwrap();
    assert!(number.explode().unwrap());
    assert_eq!(number, "[[6,[5,[7,0]]],3]".parse().unwrap());
    let mut number = Number::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
    assert!(number.explode().unwrap());
    assert_eq!(number, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap());
    let mut number = Number::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
    assert!(number.explode().unwrap());
    assert_eq!(number, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap());

    let mut element = Element::I64(10);
    element.split().unwrap();
    assert_eq!(
        element,
        Element::Number(Box::new(Number {
            x: Element::I64(5),
            y: Element::I64(5)
        }))
    );
    let mut element = Element::I64(11);
    element.split().unwrap();
    assert_eq!(
        element,
        Element::Number(Box::new(Number {
            x: Element::I64(5),
            y: Element::I64(6)
        }))
    );

    let mut number = Number::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap()
        + Number::from_str("[1,1]").unwrap();
    number.reduce().unwrap();
    assert_eq!(
        number,
        Number::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap()
    );

    let input = "[1,1]
[2,2]
[3,3]
[4,4]";
    assert_eq!(
        sum(input).unwrap(),
        Number::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap()
    );

    assert_eq!(
        Number::from_str("[[1,2],[[3,4],5]]").unwrap().magnitude(),
        143
    );
}

#[test]
fn long_sum() {
    let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
    assert_eq!(
        sum(input).unwrap(),
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap()
    );
}

#[test]
fn final_test() {
    let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    assert_eq!(
        sum(input).unwrap(),
        Number::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]").unwrap()
    );
    assert_eq!(magnitude(input).unwrap(), 4140);
}

#[test]
fn part_2() {
    let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    assert_eq!(largest_magnitude(input).unwrap(), 3993);
}
