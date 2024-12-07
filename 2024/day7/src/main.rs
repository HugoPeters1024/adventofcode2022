use std::{collections::VecDeque, io::BufRead};

use nom::IResult;

fn parse_line(input: &str) -> IResult<&str, (i64, Vec<i64>)> {
    let (input, checksum) = nom::character::complete::i64(input)?;
    let (input, _) = nom::bytes::complete::tag(": ")(input)?;
    let (input, numbers) = nom::multi::separated_list1(
        nom::character::complete::space1,
        nom::character::complete::i64,
    )(input)?;
    Ok((input, (checksum, numbers)))
}

fn valid(checksum: i64, numbers: &Vec<i64>, part2: bool) -> bool {
    let mut work = VecDeque::new();
    work.push_front((numbers[0], 1));

    while let Some((acc, idx)) = work.pop_front() {
        if idx == numbers.len() {
            if checksum == acc {
                return true;
            }
            continue;
        }

        work.push_front((acc+numbers[idx], idx+1));
        work.push_front((acc*numbers[idx], idx+1));

        if part2 {
            let num_digits = if numbers[idx] == 0 { 1 } else { (numbers[idx] as f64).log10().floor() as u32 + 1 };
            work.push_front((acc * 10_i64.pow(num_digits) + numbers[idx], idx+1));
        }
    }

    return false;
}

fn main() {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let (checksum, numbers) = parse_line(&line).unwrap().1;
        if valid(checksum, &numbers, false) {
            sum1 += checksum;
        }

        if valid(checksum, &numbers, true) {
            sum2 += checksum;
        }
    }

    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");
}
