use std::io::BufRead;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

fn parse_u64(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(i)
}

fn parse_u32(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}

fn parse_u32_list(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, parse_u32)(i)
}

fn parse_times(i: &str) -> IResult<&str, Vec<u32>> {
    let (i, _) = tag("Time:")(i)?;
    let (i, _) = space1(i)?;
    parse_u32_list(i)
}

fn parse_distances(i: &str) -> IResult<&str, Vec<u32>> {
    let (i, _) = tag("Distance:")(i)?;
    let (i, _) = space1(i)?;
    parse_u32_list(i)
}

fn main() {
    let input: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let (_, times) = parse_times(&input[0]).unwrap();
    let (_, distances) = parse_distances(&input[1]).unwrap();

    let mut product = 1;
    for (time, record) in times.iter().zip(&distances) {
        let mut win_options = 0;
        for hold_time in 0..=*time {
            let speed = hold_time;
            let distance = speed * (time - hold_time);
            if distance > *record {
                win_options += 1;
            }
        }

        product *= win_options;
    }

    println!("Part 1 {}", product);

    // Part 2
    let (_, input_time) = parse_u64(
        &input[0]
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(""),
    )
    .unwrap();
    let (_, input_record) = parse_u64(
        &input[1]
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(""),
    )
    .unwrap();

    let mut win_options = 0;
    for hold_time in 0..=input_time {
        let speed = hold_time;
        let distance = speed * (input_time - hold_time);
        if distance > input_record {
            win_options += 1;
        }
    }

    println!("Part 2 {}", win_options);
}
