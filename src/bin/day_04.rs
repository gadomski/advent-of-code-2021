use anyhow::{anyhow, Error};
use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/day_04.txt");
    println!("Part 1: {}", score_of_winning_board(input).unwrap());
    println!("Part 2: {}", score_of_last_to_win_board(input).unwrap());
}

fn score_of_winning_board(input: &str) -> Result<u64, Error> {
    let mut game = Game::new(input)?;
    let (i, number) = game.play_until_a_board_wins()?;
    let board = &game.boards[i];
    Ok(board.sum_of_all_unmarked_numbers() * u64::from(number))
}

fn score_of_last_to_win_board(input: &str) -> Result<u64, Error> {
    let mut game = Game::new(input)?;
    let (i, number) = game.play_until_all_boards_win()?;
    let board = &game.boards[i];
    Ok(board.sum_of_all_unmarked_numbers() * u64::from(number))
}

#[derive(Debug)]
struct Game {
    numbers: Vec<u16>,
    boards: Vec<Board>,
}

#[derive(Debug)]
struct Board {
    values: Vec<Vec<u16>>,
    marks: Vec<Vec<bool>>,
}

impl Game {
    fn new(input: &str) -> Result<Game, Error> {
        let groups: Vec<_> = input.split("\n\n").collect();
        if groups.is_empty() {
            return Err(anyhow!("Invalid input: {}", input));
        }
        Ok(Game {
            numbers: groups[0]
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?,
            boards: groups[1..]
                .iter()
                .map(|group| group.parse())
                .collect::<Result<_, _>>()?,
        })
    }

    fn play_until_a_board_wins(&mut self) -> Result<(usize, u16), Error> {
        loop {
            let number = self.play_one()?;
            for (i, board) in self.boards.iter().enumerate() {
                if board.wins() {
                    return Ok((i, number));
                }
            }
        }
    }

    fn play_until_all_boards_win(&mut self) -> Result<(usize, u16), Error> {
        let mut winning_boards = vec![false; self.boards.len()];
        let mut number_of_winning_boards = 0;
        loop {
            let could_be_last_turn = number_of_winning_boards == winning_boards.len() - 1;
            let number = self.play_one()?;
            for (i, board) in self.boards.iter().enumerate() {
                if board.wins() && !winning_boards[i] {
                    if could_be_last_turn {
                        return Ok((i, number));
                    } else {
                        winning_boards[i] = true;
                        number_of_winning_boards += 1;
                    }
                }
            }
        }
    }

    fn play_one(&mut self) -> Result<u16, Error> {
        if self.numbers.is_empty() {
            return Err(anyhow!("No more numbers!"));
        }
        let number = self.numbers.remove(0);
        for board in &mut self.boards {
            board.mark(number);
        }
        Ok(number)
    }
}

impl Board {
    fn mark(&mut self, number: u16) {
        for (i, row) in self.values.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == number {
                    self.marks[i][j] = true;
                    return;
                }
            }
        }
    }

    fn wins(&self) -> bool {
        let mut column_is_marked = vec![true; self.marks[0].len()];
        for row in self.marks.iter() {
            let mut row_is_marked = true;
            for (j, mark) in row.iter().enumerate() {
                row_is_marked &= mark;
                column_is_marked[j] &= mark;
            }
            if row_is_marked {
                return true;
            }
        }
        column_is_marked.iter().any(|v| *v)
    }

    fn sum_of_all_unmarked_numbers(&self) -> u64 {
        let mut sum: u64 = 0;
        for (i, row) in self.marks.iter().enumerate() {
            for (j, &mark) in row.iter().enumerate() {
                if !mark {
                    sum += u64::from(self.values[i][j]);
                }
            }
        }
        sum
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Board, Error> {
        let mut values = Vec::new();
        let mut marks = Vec::new();
        for line in s.lines() {
            let line: Vec<u16> = line
                .split_whitespace()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;
            marks.push(vec![false; line.len()]);
            values.push(line);
        }
        Ok(Board { values, marks })
    }
}

#[test]
fn example() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
    let mut game = Game::new(input).unwrap();
    let (i, number) = game.play_until_a_board_wins().unwrap();
    assert_eq!(i, 2);
    assert_eq!(number, 24);
    let board = &game.boards[i];
    assert_eq!(board.sum_of_all_unmarked_numbers(), 188);
    assert_eq!(score_of_winning_board(input).unwrap(), 4512);

    let mut game = Game::new(input).unwrap();
    let (i, number) = game.play_until_all_boards_win().unwrap();
    assert_eq!(i, 1);
    assert_eq!(number, 13);
    assert_eq!(score_of_last_to_win_board(input).unwrap(), 1924);
}
