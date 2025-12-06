use nom::{
    IResult, Parser,
    character::complete::{i64 as parse_i64, space1},
    multi::separated_list1,
};
use std::io::BufRead;

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    nom::sequence::preceded(
        nom::combinator::opt(space1),
        separated_list1(space1, parse_i64),
    )
    .parse(input)
}

fn main() {
    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<String> = Vec::new();
    for line in lines.iter() {
        if let Ok((_, res)) = parse_numbers(line) {
            numbers.push(res);
        } else {
            operators = line.split_whitespace().map(|x| x.to_string()).collect();
        }
    }

    let mut sum = 0;
    for col in 0..numbers[0].len() {
        let mut acc = Vec::new();
        for row in 0..numbers.len() {
            acc.push(numbers[row][col]);
        }

        match operators[col].as_str() {
            "*" => sum += acc.iter().fold(1, |x, y| x * y),
            "+" => sum += acc.iter().fold(0, |x, y| x + y),
            x => panic!("{}", x),
        };
    }

    println!("Part 1: {sum}");

    let mut sum = 0;
    let lines: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
    let input_width = lines.iter().map(|x| x.len()).max().unwrap();
    let mut op = ' ';
    let mut acc: Vec<usize> = Vec::new();
    for col in 0..input_width {
        let mut digits: Vec<usize> = Vec::new();
        for row in 0..lines.len() {
            if col >= lines[row].len() {
                continue;
            }
            let c = lines[row][col];
            if let Some(d) = c.to_digit(10) {
                digits.push(d as usize);
            } else if c == '*' || c == '+' {
                op = c
            }
        }

        if digits.len() > 0 {
            acc.push(digits.iter().fold(0, |acc, x| acc * 10 + x));
        }

        if digits.len() == 0 || col == input_width - 1 {
            let sub = match op {
                '*' => acc.iter().fold(1, |x, y| x * y),
                '+' => acc.iter().fold(0, |x, y| x + y),
                x => panic!("{}", x),
            };
            sum += sub;
            acc.clear();
        }
    }

    println!("Part 2: {sum}");
}
