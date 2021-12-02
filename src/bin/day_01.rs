use anyhow::{anyhow, Error};

fn main() {
    let input = include_str!("../../inputs/day_01.txt");
    println!("Part 1: {}", number_of_increases(input, 1).unwrap());
    println!("Part 2: {}", number_of_increases(input, 3).unwrap());
}

fn number_of_increases(input: &str, window_size: usize) -> Result<usize, Error> {
    let depths = input
        .lines()
        .map(|line| line.parse().map_err(Error::from))
        .collect::<Result<Vec<u32>, Error>>()?;
    let mut sums = depths.iter().enumerate().filter_map(|(i, &depth)| {
        let mut sum = depth;
        for delta in 1..window_size {
            if let Some(other) = depths.get(i + delta) {
                sum += other;
            } else {
                return None;
            }
        }
        Some(sum)
    });
    let mut previous = sums
        .next()
        .ok_or(anyhow!("Window size is larger than the number of elements"))?;
    let mut count = 0;
    for sum in sums {
        if sum > previous {
            count += 1;
        }
        previous = sum;
    }
    Ok(count)
}

#[test]
fn example() {
    let input = "199
200
208
210
200
207
240
269
260
263";
    assert_eq!(number_of_increases(input, 1).unwrap(), 7);
    assert_eq!(number_of_increases(input, 3).unwrap(), 5);
}
