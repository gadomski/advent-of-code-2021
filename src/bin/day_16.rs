use anyhow::{anyhow, Error};
use std::{collections::VecDeque, convert::TryFrom, str::FromStr};

fn main() {
    let input = include_str!("../../inputs/day_16.txt").trim();
    let packet: Packet = input.parse().unwrap();
    println!("Part 1: {}", packet.sum_of_version_numbers());
    println!("Part 2: {}", packet.evaluate().unwrap());
}

#[derive(Debug)]
struct Stream(VecDeque<bool>);

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    type_id: u8,
    body: Body,
}

#[derive(Debug, PartialEq)]
enum Body {
    Literal(u64),
    Operator {
        operation: Operation,
        packets: Vec<Packet>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Stream {
    fn read_packet(&mut self) -> Result<Packet, Error> {
        let version = self.read_u8()?;
        let type_id = self.read_u8()?;
        let body = match type_id {
            4 => Body::Literal(self.read_literal()?),
            _ => {
                let length_type_id = self.read_one()?;
                let mut subpackets = Vec::new();
                if length_type_id {
                    let number_of_subpackets = self.read_usize(11)?;
                    for _ in 0..number_of_subpackets {
                        subpackets.push(self.read_packet()?);
                    }
                } else {
                    let length = self.read_usize(15)?;
                    let target = self.0.len() - length;
                    while self.0.len() > target {
                        subpackets.push(self.read_packet()?);
                    }
                    if self.0.len() != target {
                        return Err(anyhow!(
                            "Read too many bits, expected {} left, have {}",
                            target,
                            self.0.len()
                        ));
                    }
                }
                Body::Operator {
                    operation: type_id.try_into()?,
                    packets: subpackets,
                }
            }
        };
        Ok(Packet {
            version,
            type_id,
            body,
        })
    }

    fn read_literal(&mut self) -> Result<u64, Error> {
        let mut value = 0;
        loop {
            let continue_ = self.read_one()?;
            for _ in 0..4 {
                value <<= 1;
                if self.read_one()? {
                    value += 1;
                }
            }
            if !continue_ {
                break;
            }
        }
        Ok(value)
    }

    fn read_usize(&mut self, length: usize) -> Result<usize, Error> {
        let mut value = 0;
        for _ in 0..length {
            value <<= 1;
            if self.read_one()? {
                value += 1;
            }
        }
        Ok(value)
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut value = 0;
        for _ in 0..3 {
            value <<= 1;
            if self.read_one()? {
                value += 1;
            }
        }
        Ok(value)
    }

    fn read_one(&mut self) -> Result<bool, Error> {
        self.0
            .pop_front()
            .ok_or_else(|| anyhow!("Unexpected end of stream"))
    }
}

impl Packet {
    fn sum_of_version_numbers(&self) -> u32 {
        let mut sum = u32::from(self.version);
        if let Body::Operator { ref packets, .. } = self.body {
            sum += packets
                .iter()
                .map(|packet| packet.sum_of_version_numbers())
                .sum::<u32>();
        }
        sum
    }

    fn evaluate(&self) -> Result<u64, Error> {
        use Operation::*;
        match self.body {
            Body::Literal(n) => Ok(n),
            Body::Operator {
                operation,
                ref packets,
            } => {
                let values = packets
                    .iter()
                    .map(|packet| packet.evaluate())
                    .collect::<Result<Vec<u64>, _>>()?;
                match operation {
                    Sum => Ok(values.into_iter().sum()),
                    Product => Ok(values.into_iter().product()),
                    Minimum => values
                        .into_iter()
                        .min()
                        .ok_or_else(|| anyhow!("Cannot find the min of nothing")),
                    Maximum => values
                        .into_iter()
                        .max()
                        .ok_or_else(|| anyhow!("Cannot find the max of nothing")),
                    GreaterThan => {
                        if values.len() == 2 {
                            if values[0] > values[1] {
                                Ok(1)
                            } else {
                                Ok(0)
                            }
                        } else {
                            Err(anyhow!(
                                "GreaterThan packet does not have two values: {:?}",
                                values
                            ))
                        }
                    }
                    LessThan => {
                        if values.len() == 2 {
                            if values[0] < values[1] {
                                Ok(1)
                            } else {
                                Ok(0)
                            }
                        } else {
                            Err(anyhow!(
                                "LessThan packet does not have two values: {:?}",
                                values
                            ))
                        }
                    }
                    EqualTo => {
                        if values.len() == 2 {
                            if values[0] == values[1] {
                                Ok(1)
                            } else {
                                Ok(0)
                            }
                        } else {
                            Err(anyhow!(
                                "LessThan packet does not have two values: {:?}",
                                values
                            ))
                        }
                    }
                }
            }
        }
    }
}

impl FromStr for Packet {
    type Err = Error;
    fn from_str(s: &str) -> Result<Packet, Error> {
        let mut bits = VecDeque::new();
        for c in s.chars() {
            let expanded = match c {
                '0' => [false, false, false, false],
                '1' => [false, false, false, true],
                '2' => [false, false, true, false],
                '3' => [false, false, true, true],
                '4' => [false, true, false, false],
                '5' => [false, true, false, true],
                '6' => [false, true, true, false],
                '7' => [false, true, true, true],
                '8' => [true, false, false, false],
                '9' => [true, false, false, true],
                'A' => [true, false, true, false],
                'B' => [true, false, true, true],
                'C' => [true, true, false, false],
                'D' => [true, true, false, true],
                'E' => [true, true, true, false],
                'F' => [true, true, true, true],
                _ => return Err(anyhow!("Unexpected character: {}", c)),
            };
            bits.extend(expanded);
        }
        let mut stream = Stream(bits);
        stream.read_packet()
    }
}

impl TryFrom<u8> for Operation {
    type Error = Error;

    fn try_from(n: u8) -> Result<Operation, Error> {
        use Operation::*;
        match n {
            0 => Ok(Sum),
            1 => Ok(Product),
            2 => Ok(Minimum),
            3 => Ok(Maximum),
            5 => Ok(GreaterThan),
            6 => Ok(LessThan),
            7 => Ok(EqualTo),
            _ => Err(anyhow!("Invalid operation code: {}", n)),
        }
    }
}

#[test]
fn examples() {
    assert_eq!(
        "D2FE28".parse::<Packet>().unwrap(),
        Packet {
            version: 6,
            type_id: 4,
            body: Body::Literal(2021),
        }
    );
    assert_eq!(
        "8A004A801A8002F478"
            .parse::<Packet>()
            .unwrap()
            .sum_of_version_numbers(),
        16
    );
    assert_eq!(
        "620080001611562C8802118E34"
            .parse::<Packet>()
            .unwrap()
            .sum_of_version_numbers(),
        12
    );
    assert_eq!(
        "C0015000016115A2E0802F182340"
            .parse::<Packet>()
            .unwrap()
            .sum_of_version_numbers(),
        23
    );
    assert_eq!(
        "A0016C880162017C3686B18A3D4780"
            .parse::<Packet>()
            .unwrap()
            .sum_of_version_numbers(),
        31
    );
    assert_eq!(
        "C200B40A82".parse::<Packet>().unwrap().evaluate().unwrap(),
        3
    );
    assert_eq!(
        "04005AC33890"
            .parse::<Packet>()
            .unwrap()
            .evaluate()
            .unwrap(),
        54
    );
    assert_eq!(
        "880086C3E88112"
            .parse::<Packet>()
            .unwrap()
            .evaluate()
            .unwrap(),
        7
    );
    assert_eq!(
        "CE00C43D881120"
            .parse::<Packet>()
            .unwrap()
            .evaluate()
            .unwrap(),
        9
    );
    assert_eq!(
        "D8005AC2A8F0"
            .parse::<Packet>()
            .unwrap()
            .evaluate()
            .unwrap(),
        1
    );
    assert_eq!(
        "F600BC2D8F".parse::<Packet>().unwrap().evaluate().unwrap(),
        0
    );
    assert_eq!(
        "9C005AC2F8F0"
            .parse::<Packet>()
            .unwrap()
            .evaluate()
            .unwrap(),
        0
    );
    assert_eq!(
        "9C0141080250320F1802104A08"
            .parse::<Packet>()
            .unwrap()
            .evaluate()
            .unwrap(),
        1
    );
}
