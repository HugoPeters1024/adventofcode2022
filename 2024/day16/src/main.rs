use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    io::BufRead,
};

use priority_queue::PriorityQueue;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum Dir {
    Right,
    Up,
    Left,
    Down,
}

impl Dir {
    fn turn_clockwise(&self) -> Self {
        match self {
            Dir::Right => Dir::Down,
            Dir::Up => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Left,
        }
    }

    fn turn_counter_clockwise(&self) -> Self {
        match self {
            Dir::Right => Dir::Up,
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
        }
    }
}

fn main() {
    let maze: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut sx = 0;
    let mut sy = 0;
    for (y, line) in maze.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                sx = x;
                sy = y;
            }
        }
    }

    let mut visisted: HashMap<(usize, usize, Dir), usize> = HashMap::new();

    let mut work: PriorityQueue<(usize, usize, Dir, Vec<(usize, usize)>), Reverse<usize>> =
        PriorityQueue::new();
    work.push((sx, sy, Dir::Right, Vec::new()), Reverse(0));

    let mut on_best_path: HashSet<(usize, usize)> = HashSet::new();
    let mut best_score = usize::MAX;

    while let Some(((x, y, dir, path), Reverse(score))) = work.pop() {
        if score > best_score {
            continue;
        }

        let mut path = path.clone();
        path.push((x, y));
        let path = path;

        if maze[y][x] == 'E' {
            if score <= best_score {
                on_best_path.extend(&path);
                best_score = score;
            }
        }

        if let Some(min_score_for_state) = visisted.get(&(x, y, dir)) {
            if score > *min_score_for_state {
                continue;
            }
        }

        visisted.insert((x, y, dir), score);

        match dir {
            Dir::Right if maze[y][x + 1] != '#' => {
                work.push((x + 1, y, dir, path.clone()), Reverse(score + 1));
            }
            Dir::Up if maze[y - 1][x] != '#' => {
                work.push((x, y - 1, dir, path.clone()), Reverse(score + 1));
            }
            Dir::Left if maze[y][x - 1] != '#' => {
                work.push((x - 1, y, dir, path.clone()), Reverse(score + 1));
            }
            Dir::Down if maze[y + 1][x] != '#' => {
                work.push((x, y + 1, dir, path.clone()), Reverse(score + 1));
            }
            _ => {}
        }

        work.push(
            (x, y, dir.turn_clockwise(), path.clone()),
            Reverse(score + 1000),
        );
        work.push(
            (x, y, dir.turn_counter_clockwise(), path.clone()),
            Reverse(score + 1000),
        );
    }

    println!("Part 1: {best_score}");
    println!("Part 2: {}", on_best_path.len());
}
