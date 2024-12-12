use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let width = map[0].len();
    let height = map.len();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let mut regions: Vec<(char, Vec<(usize, usize)>)> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if seen.contains(&(x, y)) {
                continue;
            }

            let marker = map[y][x];
            let mut region: Vec<(usize, usize)> = Vec::new();
            let mut work: VecDeque<(isize, isize)> = VecDeque::new();
            work.push_front((x as isize, y as isize));

            while let Some((x, y)) = work.pop_front() {
                if x < 0 || y < 0 || x >= width as isize || y >= height as isize {
                    continue;
                }

                if map[y as usize][x as usize] != marker {
                    continue;
                }

                if seen.contains(&(x as usize, y as usize)) {
                    continue;
                }

                region.push((x as usize, y as usize));
                seen.insert((x as usize, y as usize));

                work.push_front((x + 1, y));
                work.push_front((x - 1, y));
                work.push_front((x, y + 1));
                work.push_front((x, y - 1));
            }

            regions.push((marker, region));
        }
    }

    let mut sum1 = 0;
    let mut sum2 = 0;
    for (marker, region) in &regions {
        let mut perimeter = 0;
        for (x, y) in region {
            let mut edge_count = 0;
            if *x <= 0 || map[*y][x - 1] != *marker {
                edge_count += 1
            }

            if *x >= width - 1 || map[*y][x + 1] != *marker {
                edge_count += 1
            }

            if *y <= 0 || map[y - 1][*x] != *marker {
                edge_count += 1
            }

            if *y >= height - 1 || map[y + 1][*x] != *marker {
                edge_count += 1
            }

            if edge_count == 0 {
                // part of the shap itself
                continue;
            }

            perimeter += edge_count
        }

        sum1 += region.len() * perimeter;

        let mut lines = vec![vec![0; width]; height];
        for (x, y) in region.iter().cloned() {
            if x <= 0 || map[y][x - 1] != *marker {
                lines[y][x] |= 1;
            }

            if x >= width - 1 || map[y][x + 1] != *marker {
                lines[y][x] |= 2;
            }

            if y <= 0 || map[y - 1][x] != *marker {
                lines[y][x] |= 4;
            }

            if y >= height - 1 || map[y + 1][x] != *marker {
                lines[y][x] |= 8;
            }
        }

        let mut sides = 0;
        // top to bottom
        for x in 0..width {
            let mut toggle = false;
            for y in 0..height {
                if (lines[y][x] & 1) != 0 {
                    if !toggle {
                        sides += 1;
                    }
                    toggle = true;
                } else {
                    toggle = false;
                }
            }
        }

        // bottom to top
        for x in 0..width {
            let mut toggle = false;
            for y in 0..height {
                if (lines[y][x] & 2) != 0 {
                    if !toggle {
                        sides += 1;
                    }
                    toggle = true;
                } else {
                    toggle = false;
                }
            }
        }

        // left to right
        for y in 0..height {
            let mut toggle = false;
            for x in 0..width {
                if (lines[y][x] & 4) != 0 {
                    if !toggle {
                        sides += 1;
                    }
                    toggle = true;
                } else {
                    toggle = false;
                }
            }
        }

        // right to left
        for y in 0..height {
            let mut toggle = false;
            for x in 0..width {
                if (lines[y][x] & 8) != 0 {
                    if !toggle {
                        sides += 1;
                    }
                    toggle = true;
                } else {
                    toggle = false;
                }
            }
        }

        sum2 += region.len() * sides;
    }

    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");
}
