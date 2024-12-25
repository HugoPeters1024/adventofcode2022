use std::io::BufRead;

fn main() {
    let mut keys: Vec<[u8; 5]> = Vec::new();
    let mut locks: Vec<[u8; 5]> = Vec::new();

    let mut input = std::io::stdin().lock().lines();

    while let Some(first_line) = input.next().map(|x| x.unwrap()) {
        let is_lock = first_line == "#####";

        let mut next_five: Vec<Vec<char>> = Vec::new();
        for _ in 0..5 {
            next_five.push(input.next().unwrap().unwrap().chars().collect());
        }

        let mut heights = [0; 5];
        if is_lock {
            for y in 0..5 {
                for x in 0..5 {
                    if next_five[y][x] == '#' {
                        heights[x] += 1;
                    }
                }
            }
            locks.push(heights);
        } else {
            for y in (0..5).rev() {
                for x in 0..5 {
                    if next_five[y][x] == '#' {
                        heights[x] += 1;
                    }
                }
            }
            keys.push(heights);
        }

        // bottom row
        input.next();
        // empty row
        input.next();
    }

    let mut sum = 0;
    for key in &keys {
        for lock in &locks {
            if (0..5).all(|idx| key[idx] + lock[idx] <= 5) {
                sum += 1;
            }
        }
    }

    println!("Part 1: {sum}");
}
