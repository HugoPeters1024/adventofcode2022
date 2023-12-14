use std::{io::BufRead, option::Iter};

enum Dir { North, East, South, West }

fn rotate(grid: &mut Vec<Vec<char>>, dir: Dir) {
    let width = grid[0].len();
    let height = grid.len();
    let mut it = match dir {
        North => (0..height).flat_map(|y| (0..width).map(|x| (x,y))).collect(),
        South => (0..height).rev().flat_map(|y| (0..width).map(|x| (x,y))).collect(),
        _ => panic!()
    };

    for (x,y) in it {
    }
}

fn score(grid: &Vec<Vec<char>>) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'O' {
                sum += height-y;
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

    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'O' {
                let mut delta_north = 1;
                while y >= delta_north && grid[y-delta_north][x] == '.' {
                    delta_north += 1;
                }
                delta_north -= 1;

                if delta_north != 0 {
                    grid[y-delta_north][x] = 'O';
                    grid[y][x] = '.';
                }
            }
        }
    }


    println!("Part 1: {}", score(&grid));
}
