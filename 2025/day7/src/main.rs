use std::{collections::HashMap, io::BufRead};

fn main() {
    let grid: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut sx = 0;
    let mut sy = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'S' {
                sx = x;
                sy = y;
            }
        }
    }

    let mut tmp_grid = grid.clone();
    let splits = count_splits(&mut tmp_grid, sx, sy);
    println!("Part 1: {splits}");

    let tmp_grid = grid.clone();
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();
    let timelines = count_timelines(&tmp_grid, &mut seen, sx, sy);
    println!("Part 2: {timelines}");
}

fn count_splits(grid: &mut Vec<Vec<char>>, x: usize, y: usize) -> usize {
    if grid[y][x] == '|' {
        return 0;
    }
    grid[y][x] = '|';

    let height = grid.len();
    if y == height - 1 {
        return 0;
    }

    if grid[y + 1][x] == '^' {
        return 1 + count_splits(grid, x - 1, y + 1) + count_splits(grid, x + 1, y + 1);
    } else {
        return count_splits(grid, x, y + 1);
    }
}

fn count_timelines(
    grid: &Vec<Vec<char>>,
    seen: &mut HashMap<(usize, usize), usize>,
    x: usize,
    y: usize,
) -> usize {
    if let Some(cached) = seen.get(&(x, y)) {
        return *cached;
    }
    let height = grid.len();
    if y == height - 1 {
        return 1;
    }

    let ret = if grid[y + 1][x] == '^' {
        count_timelines(grid, seen, x - 1, y + 1) + count_timelines(grid, seen, x + 1, y + 1)
    } else {
        count_timelines(grid, seen, x, y + 1)
    };

    seen.insert((x, y), ret);
    return ret;
}
