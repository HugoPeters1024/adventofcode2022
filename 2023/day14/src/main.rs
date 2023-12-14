use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn do_steps(&self, x: isize, y: isize, steps: isize) -> (isize, isize) {
        match self {
            Dir::North => (x, y - steps),
            Dir::East => (x + steps, y),
            Dir::South => (x, y + steps),
            Dir::West => (x - steps, y),
        }
    }

    fn succ(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
        }
    }
}

fn mk_iterator(dir: &Dir, width: isize, height: isize) -> Box<dyn Iterator<Item = (isize, isize)>> {
    match dir {
        Dir::North => Box::new((0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))),
        Dir::South => Box::new(
            (0..height)
                .rev()
                .flat_map(move |y| (0..width).map(move |x| (x, y))),
        ),
        Dir::East => Box::new(
            (0..width)
                .rev()
                .flat_map(move |x| (0..height).map(move |y| (x, y))),
        ),
        Dir::West => Box::new((0..width).flat_map(move |x| (0..height).map(move |y| (x, y)))),
    }
}

fn rotate(grid: &mut Vec<Vec<char>>, dir: &Dir) {
    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    for (x, y) in mk_iterator(&dir, width, height) {
        if grid[y as usize][x as usize] == 'O' {
            let mut delta = 1;
            loop {
                let (nx, ny) = dir.do_steps(x, y, delta);
                if nx < 0
                    || nx >= width
                    || ny < 0
                    || ny >= height
                    || grid[ny as usize][nx as usize] != '.'
                {
                    break;
                }
                delta += 1;
            }
            delta -= 1;

            if delta != 0 {
                let (nx, ny) = dir.do_steps(x, y, delta);
                grid[ny as usize][nx as usize] = 'O';
                grid[y as usize][x as usize] = '.';
            }
        }
    }
}

fn score(grid: &Vec<Vec<char>>) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'O' {
                sum += height - y;
            }
        }
    }

    sum
}

fn main() {
    let input_grid: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut grid = input_grid.clone();

    rotate(&mut grid, &Dir::North);
    println!("Part 1: {}", score(&grid));

    let mut grid = input_grid.clone();

    let mut seen: HashMap<(Vec<Vec<char>>, Dir), usize> = HashMap::new();
    let mut idx_to_score: Vec<usize> = Vec::new();
    let mut dir = Dir::North;
    const MAX_ITER: usize = 4000000000;

    for i in 0..=MAX_ITER {
        let cache_key = (grid.clone(), dir.clone());
        if let Some(prev_idx) = seen.get(&cache_key) {
            println!(
                "Loop detected from iteration {} with size {}",
                i,
                i - prev_idx
            );

            let base = *prev_idx;
            let loop_size = i - base;
            let end = base + (MAX_ITER - base) % loop_size;
            println!("Score at end: {}", idx_to_score[end]);
            break;
        }

        seen.insert(cache_key, i);
        idx_to_score.push(score(&grid));

        rotate(&mut grid, &dir);
        dir = dir.succ();
    }
}
