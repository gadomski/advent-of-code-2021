use thiserror::Error;

fn main() {
    let input = include_str!("../../inputs/day_10.txt");
    println!("Part 1: {}", syntax_error_score(input).unwrap());
    println!("Part 2: {}", middle_completion_score(input).unwrap());
}

fn middle_completion_score(input: &str) -> Result<u64, ParseError> {
    let mut scores = Vec::new();
    for line in input.lines() {
        match parse(line) {
            Ok(stack) => scores.push(completion_score(&stack)),
            Err(err) => match err {
                ParseError::UnexpectedCharacter(_) => {}
                _ => return Err(err),
            },
        }
    }
    if scores.len() % 2 == 0 {
        panic!("Didn't expect an even number of lines");
    }
    scores.sort_unstable();
    Ok(scores[scores.len() / 2])
}

fn completion_score(stack: &[char]) -> u64 {
    let mut score = 0;
    for c in stack.iter().rev() {
        score *= 5;
        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Did not expect {} to make it this far", c),
        }
    }
    score
}

fn syntax_error_score(input: &str) -> Result<u64, ParseError> {
    let mut total = 0;
    for line in input.lines() {
        match parse(line) {
            Ok(_) => {}
            Err(err) => match err {
                ParseError::UnexpectedCharacter(c) => total += score(c)?,
                _ => return Err(err),
            },
        }
    }
    Ok(total)
}

fn parse(line: &str) -> Result<Vec<char>, ParseError> {
    let mut stack = Vec::new();
    for c in line.chars() {
        let ok = match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
                true
            }
            ')' => maybe_pop(&mut stack, '('),
            ']' => maybe_pop(&mut stack, '['),
            '}' => maybe_pop(&mut stack, '{'),
            '>' => maybe_pop(&mut stack, '<'),
            _ => return Err(ParseError::InvalidCharacter(c)),
        };
        if !ok {
            return Err(ParseError::UnexpectedCharacter(c));
        }
    }
    Ok(stack)
}

fn maybe_pop(stack: &mut Vec<char>, c: char) -> bool {
    if stack.last().map(|&last| last == c).unwrap_or(false) {
        stack.pop();
        true
    } else {
        false
    }
}

fn score(c: char) -> Result<u64, ParseError> {
    match c {
        ')' => Ok(3),
        ']' => Ok(57),
        '}' => Ok(1197),
        '>' => Ok(25137),
        _ => Err(ParseError::NoScoreDefined(c)),
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("no score is defined for char: {0}")]
    NoScoreDefined(char),
    #[error("unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("invalid character: {0}")]
    InvalidCharacter(char),
}

#[test]
fn example() {
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    assert_eq!(syntax_error_score(input).unwrap(), 26397);
    assert_eq!(middle_completion_score(input).unwrap(), 288957);
}
