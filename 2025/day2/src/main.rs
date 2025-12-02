use std::io::BufRead;

use itertools::Itertools;

fn main() {
    let ranges = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| {
            let parts: Vec<usize> = x.split('-').map(|x| x.parse::<usize>().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect::<Vec<_>>();

    let mut sum_of_invalid = 0;
    for &(lhs, rhs) in ranges.iter() {
        for i in lhs..=rhs {
            let str = format!("{}", i);
            if str.len() % 2 != 0 {
                continue;
            }
            let lhs = str.chars().take(str.len() / 2).collect::<String>();
            let rhs = str
                .chars()
                .skip(str.len() / 2)
                .take(str.len() / 2)
                .collect::<String>();
            if lhs == rhs {
                sum_of_invalid += i;
            }
        }
    }

    println!("Part 1: {sum_of_invalid}");

    let mut sum_of_invalid = 0;
    for &(lhs, rhs) in ranges.iter() {
        for i in lhs..=rhs {
            let str = format!("{}", i);
            for wz in 1..=str.len()/2 {
                if str
                    .chars()
                    .chunks(wz)
                    .into_iter()
                    .map(|x| x.collect::<String>())
                    .all_equal()
                {
                    sum_of_invalid += i;
                    break;
                }
            }
        }
    }

    println!("Part 1: {sum_of_invalid}");
}
