use std::{collections::VecDeque, io::BufRead};

fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let (width, height) = (map[0].len(), map.len());

    let (mut sx, mut sy) = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'S' {
                sx = x;
                sy = y;
            }
        }
    }

    let MAX_STEPS: usize = 65;

    dbg!(width, height);
    dbg!(sx, sy);

    let mut visisted = vec![vec![vec![false; MAX_STEPS + 1]; width]; height];

    let mut work = VecDeque::new();
    work.push_back((sx, sy, 0));

    while let Some((x, y, steps)) = work.pop_front() {
        if map[y][x] == '#' {
            continue;
        }

        if visisted[y][x][steps] {
            continue;
        }

        visisted[y][x][steps] = true;

        if steps == MAX_STEPS {
            continue;
        }

        if x > 0 {
            work.push_back((x - 1, y, steps + 1));
        }

        if x < width - 1 {
            work.push_back((x + 1, y, steps + 1));
        }

        if y > 0 {
            work.push_back((x, y - 1, steps + 1));
        }

        if y < height - 1 {
            work.push_back((x, y + 1, steps + 1));
        }
    }

    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if visisted[y][x][MAX_STEPS] {
                count += 1;
            }
        }
    }

    println!("Part 1 {}", count);

    for y in 0..height {
        for x in 0..width {
            if visisted[y][x][MAX_STEPS] {
                print!("O");
            } else {
                print!("{}", map[y][x]);
            }
        }
        println!();
    }

    let MAX_STEPS = 26501365;

    println!("{} fits in {}: {} times, with a remainder of {}", width, MAX_STEPS, MAX_STEPS / width, MAX_STEPS % width);
    println!("{} fits in {}: {} times, with a remainder of {}", height, MAX_STEPS, MAX_STEPS / height, MAX_STEPS % height);

    let FULL_REACH = 7320;
    let HALF_REACH = 3720;

    let n = MAX_STEPS / FULL_REACH;

    let full_count = n/2 + ((n-1) * (n-1))/2;

    println!("Part 2: {}", full_count * FULL_REACH + HALF_REACH);
}
