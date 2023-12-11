use std::io::BufRead;

use itertools::Itertools;

fn main() {
    let initial_map = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (y, row) in initial_map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if char == &'#' {
                galaxies.push((x, y));
            }
        }
    }

    let initial_width = initial_map[0].len();
    let initial_height = initial_map.len();

    let mut row_expansion: Vec<isize> = vec![0; initial_width];

    for (y, row) in initial_map.iter().enumerate() {
        let prev = if y == 0 { 0 } else { row_expansion[y - 1] };
        row_expansion[y] = prev;
        if row.iter().all(|c| c == &'.') {
            row_expansion[y] += 1;
        }
    }

    let mut col_expansion: Vec<isize> = vec![0; initial_height];

    for x in 0..initial_width {
        let prev = if x == 0 { 0 } else { col_expansion[x - 1] };
        col_expansion[x] = prev;
        if (0..initial_height)
            .map(|y| initial_map[y][x])
            .all(|c| c == '.')
        {
            col_expansion[x] += 1;
        }
    }

    let mut sum = 0;
    for combi in galaxies.iter().combinations(2) {
        let (x1, y1) = *combi[0];
        let (x2, y2) = *combi[1];

        let dist_x = ((x1 as isize + col_expansion[x1]) - (x2 as isize + col_expansion[x2])).abs();
        let dist_y = ((y1 as isize + row_expansion[y1]) - (y2 as isize + row_expansion[y2])).abs();
        sum += dist_x + dist_y;
    }

    println!("Part 1: {}", sum);

    let mut row_expansion: Vec<isize> = vec![0; initial_width];

    for (y, row) in initial_map.iter().enumerate() {
        let prev = if y == 0 { 0 } else { row_expansion[y - 1] };
        row_expansion[y] = prev;
        if row.iter().all(|c| c == &'.') {
            row_expansion[y] += 999999;
        }
    }

    let mut col_expansion: Vec<isize> = vec![0; initial_height];

    for x in 0..initial_width {
        let prev = if x == 0 { 0 } else { col_expansion[x - 1] };
        col_expansion[x] = prev;
        if (0..initial_height)
            .map(|y| initial_map[y][x])
            .all(|c| c == '.')
        {
            col_expansion[x] += 999999;
        }
    }

    let mut sum = 0;
    for combi in galaxies.iter().combinations(2) {
        let (x1, y1) = *combi[0];
        let (x2, y2) = *combi[1];

        let dist_x = ((x1 as isize + col_expansion[x1]) - (x2 as isize + col_expansion[x2])).abs();
        let dist_y = ((y1 as isize + row_expansion[y1]) - (y2 as isize + row_expansion[y2])).abs();
        sum += dist_x + dist_y;
    }

    println!("Part 2: {}", sum);
}
