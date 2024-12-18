use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    io::BufRead,
};

use priority_queue::PriorityQueue;
use scanf::sscanf;

const WIDTH: usize = 71usize;
const HEIGHT: usize = 71usize;
const FALLEN: usize = 1024;

fn heuristic(x: usize, y: usize) -> usize {
    (WIDTH - x) + (HEIGHT - y)
}

fn path_length(occupied: &HashSet<(usize, usize)>) -> Option<usize> {
    let mut visisted_in: HashMap<(usize, usize), usize> = HashMap::new();

    let mut work: PriorityQueue<(usize, usize), Reverse<(usize, usize)>> = PriorityQueue::new();
    work.push((0, 0), Reverse((0, heuristic(0, 0))));

    while let Some(((x, y), Reverse((steps, _)))) = work.pop() {
        if occupied.contains(&(x, y)) {
            continue;
        }

        if let Some(best_steps) = visisted_in.get(&(x, y)) {
            if steps >= *best_steps {
                continue;
            }
        }
        visisted_in.insert((x, y), steps);

        if x == WIDTH - 1 && y == HEIGHT - 1 {
            return Some(steps);
        }

        if x > 0 {
            work.push((x - 1, y), Reverse((steps + 1, heuristic(x - 1, y))));
        }

        if y > 0 {
            work.push((x, y - 1), Reverse((steps + 1, heuristic(x, y - 1))));
        }

        if x < WIDTH - 1 {
            work.push((x + 1, y), Reverse((steps + 1, heuristic(x + 1, y))));
        }

        if y < HEIGHT - 1 {
            work.push((x, y + 1), Reverse((steps + 1, heuristic(x, y + 1))));
        }
    }

    None
}

fn main() {
    let bytes: Vec<(usize, usize)> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let mut x = 0;
            let mut y = 0;
            sscanf!(&line.unwrap(), "{},{}", x, y).unwrap();
            (x, y)
        })
        .collect();

    let mut occupied: HashSet<(usize, usize)> = bytes.iter().take(FALLEN).cloned().collect();
    println!("Part 1: {}", path_length(&occupied).unwrap());

    let mut next_byte = FALLEN;
    loop {
        let byte = bytes[next_byte];
        occupied.insert(byte);
        if path_length(&occupied).is_none() {
            println!("Part 2: {},{}", byte.0, byte.1);
            break;
        }
        next_byte += 1;
    }
}
