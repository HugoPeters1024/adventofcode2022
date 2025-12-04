use std::io::BufRead;

fn main() {
    let grid: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut free = 0;
    let width = grid[0].len() as isize;
    let height = grid[0].len() as isize;

    for y in 0..grid.len() as isize {
        for x in 0..grid[0].len() as isize {
            if grid[y as usize][x as usize] != '@' {
                continue;
            }
            let mut adj_count = 0;
            for i in -1isize..=1 {
                for j in -1isize..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let x = x + i;
                    let y = y + j;

                    if x < 0 || x >= width || y < 0 || y >= height {
                        continue;
                    }

                    if grid[y as usize][x as usize] == '@' {
                        adj_count += 1;
                    }
                }
            }

            if adj_count < 4 {
                free += 1;
            }
        }
    }

    println!("Part 1: {free}");

    let mut grid = grid.clone();
    let mut removed = 0;
    while remove_roll(&mut grid) {
        removed += 1;
    }


    println!("Part 2: {removed}");
}

fn remove_roll(grid: &mut Vec<Vec<char>>) -> bool {
    let width = grid[0].len() as isize;
    let height = grid[0].len() as isize;

    for y in 0..grid.len() as isize {
        for x in 0..grid[0].len() as isize {
            if grid[y as usize][x as usize] != '@' {
                continue;
            }
            let mut adj_count = 0;
            for i in -1isize..=1 {
                for j in -1isize..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let x = x + i;
                    let y = y + j;

                    if x < 0 || x >= width || y < 0 || y >= height {
                        continue;
                    }

                    if grid[y as usize][x as usize] == '@' {
                        adj_count += 1;
                    }
                }
            }

            if adj_count < 4 {
                grid[y as usize][x as usize] = '.';
                return true;
            }
        }
    }

    return false;
}
