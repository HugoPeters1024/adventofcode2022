use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

fn trail_score(data: &Vec<Vec<u32>>, start_x: usize, start_y: usize) -> usize {
    let width = data[0].len();
    let height = data.len();
    let mut visisted: HashSet<(usize, usize)> = HashSet::new();
    let mut work = VecDeque::new();
    let mut peaks_found: HashSet<(usize, usize)> = HashSet::new();
    work.push_front((start_x, start_y));

    while let Some((x, y)) = work.pop_front() {
        if visisted.contains(&(x, y)) {
            continue;
        }

        visisted.insert((x, y));

        if data[y][x] == 9 {
            peaks_found.insert((x, y));
        }

        if x < width-1 && data[y][x + 1] == data[y][x] + 1 {
            work.push_front((x + 1, y));
        }

        if x > 0 && data[y][x - 1] == data[y][x] + 1 {
            work.push_front((x - 1, y));
        }

        if y < height-1 && data[y + 1][x] == data[y][x] + 1 {
            work.push_front((x, y + 1));
        }

        if y > 0 && data[y - 1][x] == data[y][x] + 1 {
            work.push_front((x, y - 1));
        }
    }

    peaks_found.len()
}

fn trail_rating(data: &Vec<Vec<u32>>, start_x: usize, start_y: usize) -> usize {
    let width = data[0].len();
    let height = data.len();
    let mut work: VecDeque<(usize, usize, HashSet<(usize, usize)>)> = VecDeque::new();
    let mut paths_found = 0;
    work.push_front((start_x, start_y, HashSet::new()));

    while let Some((x, y, path)) = work.pop_front() {
        if path.contains(&(x, y)) {
            continue;
        }

        let mut path = path;
        path.insert((x, y));

        if data[y][x] == 9 {
            paths_found += 1;
            continue;
        }

        if x < width-1 && data[y][x + 1] == data[y][x] + 1 {
            work.push_front((x + 1, y, path.clone()));
        }

        if x > 0 && data[y][x - 1] == data[y][x] + 1 {
            work.push_front((x - 1, y, path.clone()));
        }

        if y < height-1 && data[y + 1][x] == data[y][x] + 1 {
            work.push_front((x, y + 1, path.clone()));
        }

        if y > 0 && data[y - 1][x] == data[y][x] + 1 {
            work.push_front((x, y - 1, path.clone()));
        }
    }

    paths_found
}

fn main() {
    let data: Vec<Vec<u32>> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let width = data[0].len();
    let height = data.len();

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if data[y][x] == 0 {
                sum += trail_score(&data, x, y);
            }
        }
    }

    println!("Part 1: {sum}");

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if data[y][x] == 0 {
                sum += trail_rating(&data, x, y);
            }
        }
    }

    println!("Part 2: {sum}");
}
