use std::io::BufRead;

use itertools::Itertools;

fn main() {
    let banks: Vec<Vec<usize>> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut sum = 0;
    for bank in banks.iter() {
        let mut bank_max = 0;
        for i in 0..bank.len() {
            for j in i + 1..bank.len() {
                let v = 10 * bank[i] + bank[j];
                bank_max = bank_max.max(v);
            }
        }
        sum += bank_max;
    }

    println!("Part 1: {sum}");

    let mut sum = 0;
    for bank in banks.iter() {
        sum += largest_n(12, bank, 0);
    }

    println!("Part 2: {sum}");
}

fn largest_n(num_digits: usize, bank: &[usize], offset: usize) -> usize {
    if num_digits == 0 {
        return 0;
    }

    if bank.len() - offset < num_digits {
        panic!(
            "offset was {}, but need to take {} digits (len() = {})",
            offset,
            num_digits,
            bank.len()
        );
    }

    let best_digit = bank
        .iter()
        .skip(offset)
        .take(bank.len() - offset - num_digits + 1)
        .max()
        .unwrap();

    return bank
        .iter()
        .enumerate()
        .skip(offset)
        .take(bank.len() - offset - num_digits + 1)
        .filter(|(_, x)| *x == best_digit)
        .map(|(i, x)| {
            x * 10_usize.pow((num_digits - 1) as u32) + largest_n(num_digits - 1, bank, i + 1)
        })
        .max()
        .unwrap();
}
