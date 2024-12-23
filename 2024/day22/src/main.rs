use std::{collections::VecDeque, io::BufRead};

use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;

fn single_iter(mut x: isize) -> isize {
    const MODULO: isize = 16777216;
    x = ((x * 64) ^ x) % MODULO;
    x = ((x / 32) ^ x) % MODULO;
    x = ((x * 2048) ^ x) % MODULO;
    x
}

fn calc_iteration(mut x: isize, mut iteration: isize) -> isize {
    while iteration > 0 {
        x = single_iter(x);
        iteration -= 1;
    }
    x
}

fn best_price(mut secret: isize, sequence: &[isize; 4]) -> Option<isize> {
    let mut diffs = VecDeque::new();
    for _ in 0..2000 {
        let prev_price = secret % 10;
        secret = single_iter(secret);
        let price = secret % 10;

        diffs.push_back(price - prev_price);
        if diffs.len() > 4 {
            diffs.pop_front().unwrap();
        }

        if diffs == sequence {
            return Some(price);
        }
    }

    None
}

fn main() {
    let initials: Vec<isize> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().parse::<isize>().unwrap())
        .collect();

    let mut sum: isize = 0;
    for secret in &initials {
        sum += calc_iteration(*secret, 2000);
    }

    println!("Part 1: {sum}");

    let best_price_found = (-9..=9)
        .into_par_iter()
        .map(|s0| {
            let mut best_price_found = 0;
            for s1 in -9..=9 {
                if s0 + s1 > 9 || s0 + s1 < -9 {
                    continue;
                }
                //println!("s1 = {}, best_price_found = {}", s1, best_price_found);
                for s2 in -9..=9 {
                    if s1 + s2 > 9 || s1 + s2 < -9 {
                        continue;
                    }
                    for s3 in -9..=9 {
                        if s2 + s3 > 9 || s2 + s3 < -9 {
                            continue;
                        }
                        let sequence = [s0, s1, s2, s3];
                        let mut gains = 0;

                        for secret in &initials {
                            if let Some(a_price) = best_price(*secret, &sequence) {
                                gains += a_price;
                            }
                        }

                        if gains > best_price_found {
                            best_price_found = gains;
                        }
                    }
                }
            }
            best_price_found
        })
        .max();

    println!("Part 2: {:?}", best_price_found);
}
