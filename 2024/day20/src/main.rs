use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

fn main() {
    let grid: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut start_x = 0isize;
    let mut start_y = 0isize;

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_x = x as isize;
                start_y = y as isize;
            }
        }
    }

    let mut dist_from_start: HashMap<(isize, isize), isize> = HashMap::new();

    let mut work: VecDeque<(isize, isize, isize)> = VecDeque::new();
    work.push_front((start_x, start_y, 0));
    while let Some((x, y, dist)) = work.pop_front() {
        if grid[y as usize][x as usize] == '#' {
            continue;
        }

        if dist_from_start.contains_key(&(x, y)) {
            continue;
        }

        dist_from_start.insert((x, y), dist);

        work.push_back((x + 1, y, dist + 1));
        work.push_back((x - 1, y, dist + 1));
        work.push_back((x, y + 1, dist + 1));
        work.push_back((x, y - 1, dist + 1));
    }

    let cheat_threshold = 100;
    let max_cheat_length = 20isize;

    let mut cheats_found = 0;
    let mut work: VecDeque<(isize, isize, isize)> = VecDeque::new();
    let mut visisted: HashSet<(isize, isize)> = HashSet::new();
    work.push_front((start_x, start_y, 0));
    while let Some((x, y, dist)) = work.pop_front() {
        if grid[y as usize][x as usize] == '#' {
            continue;
        }

        if visisted.contains(&(x, y)) {
            continue;
        }
        visisted.insert((x, y));

        for dy in -max_cheat_length..=max_cheat_length {
            for dx in -max_cheat_length..=max_cheat_length {
                let distance_travelled = dx.abs() + dy.abs();
                // crude filter
                if distance_travelled > max_cheat_length {
                    continue;
                }

                let nx = x + dx;
                let ny = y + dy;
                if let Some(potential_further) = dist_from_start.get(&(nx, ny)) {
                    let gain = potential_further - dist - distance_travelled;
                    if gain >= cheat_threshold {
                        cheats_found += 1
                    }
                }
            }
        }

        dist_from_start.insert((x, y), dist);

        work.push_back((x + 1, y, dist + 1));
        work.push_back((x - 1, y, dist + 1));
        work.push_back((x, y + 1, dist + 1));
        work.push_back((x, y - 1, dist + 1));
    }

    println!(
        "cheats found with threshold {}: {}",
        cheat_threshold, cheats_found
    );
}
