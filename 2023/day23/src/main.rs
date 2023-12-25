use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
    io::BufRead,
};

use bitvector::BitVector;
use indexmap::IndexSet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct HIndexSet {
    inner: IndexSet<usize>,
}

impl HIndexSet {
    fn new() -> Self {
        Self {
            inner: IndexSet::new(),
        }
    }

    fn inserted(&self, x: usize, y: usize) -> Self {
        let mut ret = self.clone();
        let v = 1000 * y + x;
        ret.inner.insert(v);
        return ret;
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        let v = 1000 * y + x;
        self.inner.contains(&v)
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl std::hash::Hash for HIndexSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for v in self.inner.iter() {
            v.hash(state);
        }
    }
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let startx = 1;
    let starty = 0;

    let mut visisted: HashSet<HIndexSet> = HashSet::new();
    let mut work: VecDeque<(usize, usize, HIndexSet)> = VecDeque::new();
    work.push_front((startx, starty, HIndexSet::new()));

    let mut max = 0;

    while let Some((x, y, xs)) = work.pop_front() {
        if x == width - 2 && y == height - 1 {
            println!("Found path with {} steps", xs.inner.len());
            max = max.max(xs.inner.len());
        }

        if grid[y][x] == '#' {
            continue;
        }

        if xs.contains(x, y) {
            continue;
        }

        let xs = xs.inserted(x, y);

        if visisted.contains(&xs) {
            continue;
        }
        visisted.insert(xs.clone());

        if grid[y][x] == '>' && x < width - 1 {
            work.push_back((x + 1, y, xs.clone()));
        } else if grid[y][x] == '<' && x > 0 {
            work.push_back((x - 1, y, xs.clone()));
        } else if grid[y][x] == '^' && y > 0 {
            work.push_back((x, y - 1, xs.clone()));
        } else if grid[y][x] == 'v' && y < height - 1 {
            work.push_back((x, y + 1, xs.clone()));
        } else {
            if x > 0 {
                work.push_back((x - 1, y, xs.clone()));
            }

            if x < width - 1 {
                work.push_back((x + 1, y, xs.clone()));
            }

            if y > 0 {
                work.push_back((x, y - 1, xs.clone()));
            }

            if y < height - 1 {
                work.push_back((x, y + 1, xs.clone()));
            }
        }
    }

    max
}

trait GetHash {
    fn get_hash(&self) -> u64;
}

impl GetHash for HashSet<(usize, usize)> {
    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for (x, y) in self.iter() {
            x.hash(&mut hasher);
            y.hash(&mut hasher);
        }
        hasher.finish()
    }
}

fn part2_dfs(grid: &Vec<Vec<char>>) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let startx = 1;
    let starty = 0;

    let mut work: VecDeque<(usize, usize, BitVector)> = VecDeque::new();
    work.push_front((startx, starty, BitVector::new(width * height)));

    let mut max = 0;

    while let Some((x, y, xs)) = work.pop_front() {
        if x == width - 2 && y == height - 1 {
            max = max.max(xs.len());
            println!("Found path with {} steps, best {}", xs.len(), max);
        }

        if grid[y][x] == '#' {
            continue;
        }

        let key = width * y + x;
        if xs.contains(key) {
            continue;
        }
        let mut xs = xs.clone();
        xs.insert(key);

        if y < height - 1 {
            work.push_front((x, y + 1, xs.clone()));
        }

        if x > 0 {
            work.push_front((x - 1, y, xs.clone()));
        }

        if x < width - 1 {
            work.push_front((x + 1, y, xs.clone()));
        }

        if y > 0 {
            work.push_front((x, y - 1, xs.clone()));
        }
    }

    max
}

fn part2_backtrack(grid: &Vec<Vec<char>>, width: &usize, height: &usize, x: usize, y: usize, visisted: &mut BitVector, best: &mut usize) -> usize {
    if grid[y][x] == '#' {
        return 0;
    }

    if x == width - 2 && y == height - 1 {
        if visisted.len() > *best {
            *best = visisted.len();
            println!("Found new best path with {} steps", visisted.len());
            return *best;
        }
    }

    let key = width * y + x;
    if visisted.contains(key) {
        return 0;
    }

    let mut max = 0;
    visisted.insert(key);
    if y < height - 1 {
        max = max.max(part2_backtrack(grid, width, height, x, y + 1, visisted, best));
    }

    if x > 0 {
        max = max.max(part2_backtrack(grid, width, height, x - 1, y, visisted, best));
    }

    if x < width - 1 {
        max = max.max(part2_backtrack(grid, width, height, x + 1, y, visisted, best));
    }

    if y > 0 {
        max = max.max(part2_backtrack(grid, width, height, x, y - 1, visisted, best));
    }
    visisted.remove(key);
    max
}

fn main() {
    let grid: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    println!("Part 1: {}", part1(&grid));
    let width = grid[0].len();
    let height = grid.len();
    let mut visisted = BitVector::new(width * height);
    let mut best = 0;
    println!("Part 2: {}", part2_backtrack(&grid, &width, &height, 1, 0, &mut visisted, &mut best));
}
