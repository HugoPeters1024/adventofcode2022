use std::{
    collections::HashMap,
    io::BufRead,
};

type Level = HashMap<(i32, i32), char>;

fn is_live(level: &Level, x: i32, y: i32) -> bool {
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some(value) = level.get(&(x + dx, y + dy)) {
                if *value != '.' && !value.is_digit(10) {
                    return true;
                }
            }
        }
    }

    false
}

fn main() {
    let mut level: HashMap<(i32, i32), char> = HashMap::new();
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    // mapping from a coord to the start_x of the number and the value of the number
    let mut loc_to_number: HashMap<(i32, i32), (i32, u64)> = HashMap::new();

    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        max_y = y as i32;
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            max_x = x as i32;
            level.insert((x as i32, y as i32), c);
        }
    }

    let mut sum = 0;
    for y in 0..=max_y {
        let mut acc = 0;
        let mut live = false;
        let mut number_start_x = 0;
        let mut number_len = 0;
        for x in 0..=max_x {
            let value = level.get(&(x, y)).unwrap();
            if let Some(digit) = value.to_digit(10) {
                if acc == 0 {
                    number_start_x = x;
                }
                number_len += 1;
                acc = acc * 10 + digit;
                live |= is_live(&level, x, y);
            }

            if !value.is_digit(10) || x == max_x {
                if live {
                    sum += acc;
                    for i in 0..number_len {
                        loc_to_number.insert((number_start_x + i, y), (number_start_x, acc as u64));
                    }
                }
                live = false;
                number_len = 0;
                acc = 0;
            }
        }
    }

    println!("Part 1: {}", sum);

    let mut sum = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            let value = level.get(&(x, y)).unwrap();
            if *value == '*' {
                let mut neighbour_numbers: HashMap<(i32, i32), u64> = HashMap::new();
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if let Some((start_x, v)) = loc_to_number.get(&(x + dx, y + dy)) {
                            neighbour_numbers.insert((*start_x, y+dy), *v);
                        }
                    }
                }

                if neighbour_numbers.len() == 2 {
                    sum += neighbour_numbers.values().product::<u64>();
                }
            }
        }
    }

    println!("Part 2: {}", sum);
}
