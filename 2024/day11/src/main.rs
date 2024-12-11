use std::{collections::HashMap, io::BufRead};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn num_digits(x: usize) -> usize {
    if x == 0 {
        1
    } else {
        (x as f64).log10().floor() as usize + 1
    }
}

fn cached(seen: &mut HashMap<(usize, usize), usize>, stone: usize, steps: usize) -> usize {
    if let Some(cached) = seen.get(&(stone, steps)) {
        return *cached;
    }
    let answer = expansion(stone, steps, seen);
    seen.insert((stone, steps), answer);
    return answer;
}

fn expansion(stone: usize, steps: usize, seen: &mut HashMap<(usize, usize), usize>) -> usize {
    if steps == 0 {
        return 1;
    }

    if stone == 0 {
        return cached(seen, 1, steps - 1);
    }

    if num_digits(stone) % 2 == 0 {
        let stone_str = format!("{}", stone);
        let lhs = stone_str[0..stone_str.len() / 2].parse::<usize>().unwrap();
        let rhs = stone_str[stone_str.len() / 2..stone_str.len()]
            .parse::<usize>()
            .unwrap();
        return cached(seen, lhs, steps - 1) + cached(seen, rhs, steps - 1);
    }

    return cached(seen, stone * 2024, steps - 1);
}

fn main() {
    let stones: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut buffer = stones.clone();

    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for stone in &buffer {
            if *stone == 0 {
                new_stones.push(1);
            } else if num_digits(*stone) % 2 == 0 {
                let stone_str = format!("{}", stone);
                new_stones.push(stone_str[0..stone_str.len() / 2].parse::<usize>().unwrap());
                new_stones.push(
                    stone_str[stone_str.len() / 2..stone_str.len()]
                        .parse::<usize>()
                        .unwrap(),
                );
            } else {
                new_stones.push(stone * 2024);
            }
        }

        buffer = new_stones;
    }

    println!("Part 1: {}", buffer.len());
    let sum = stones
        .par_iter()
        .map(|stone| {
            let mut seen = HashMap::new();
            let sum = expansion(*stone, 75, &mut seen);
            sum
        })
        .sum::<usize>();

    println!("Part 2: {}", sum);
}
