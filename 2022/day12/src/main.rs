use std::{io::BufRead, collections::{HashSet, VecDeque}};

fn main() {
    let mut grid: Vec<Vec<u32>> = std::io::stdin().lock().lines().map(|x| x.unwrap().chars().map(|x| x as u32).collect()).collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut visisted : HashSet<(usize, usize)> = HashSet::new();

    let mut s_loc : (usize, usize) = (0, 0);
    let mut e_loc : (usize, usize) = (0, 0);

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'S' as u32 {
                s_loc = (x, y);
                grid[y][x] = 'a' as u32;
            }

            if grid[y][x] == 'E' as u32 {
                e_loc = (x, y);
                grid[y][x] = 'z' as u32;
            }
        }
    }

    let mut work : VecDeque<(usize, usize, u32)> = VecDeque::new();
    work.push_back((s_loc.0, s_loc.1, 0));

    while let Some((cx, cy, cost)) = work.pop_front() {
        if visisted.contains(&(cx,cy)) {
            continue;
        }

        visisted.insert((cx, cy));

        if (cx, cy) == e_loc {
            println!("Found it! {}", cost);
            break;
        }

        if cx < width -1 && grid[cy][cx+1] <= grid[cy][cx] + 1 {
            work.push_back((cx+1, cy, cost+1));
        }

        if cx > 0 && grid[cy][cx-1] <= grid[cy][cx] + 1 {
            work.push_back((cx-1, cy, cost+1));
        }

        if cy < height - 1 && grid[cy+1][cx] <= grid[cy][cx] + 1 {
            work.push_back((cx, cy+1, cost+1));
        }

        if cy > 0 && grid[cy-1][cx] <= grid[cy][cx] + 1 {
            work.push_back((cx, cy-1, cost+1));
        }
    }

    // Part 2
    let mut all_starts : Vec<(usize, usize)> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'a' as u32 {
                all_starts.push((x, y));
            }
        }
    }

    let mut all_costs : Vec<u32> = Vec::new();
    for (sx, sy) in all_starts {
        let mut work : VecDeque<(usize, usize, u32)> = VecDeque::new();
        work.push_back((sx, sy, 0));
        let mut visisted : HashSet<(usize, usize)> = HashSet::new();
        while let Some((cx, cy, cost)) = work.pop_front() {
            if visisted.contains(&(cx,cy)) {
                continue;
            }

            visisted.insert((cx, cy));

            if (cx, cy) == e_loc {
                all_costs.push(cost);
                break;
            }

            if cx < width -1 && grid[cy][cx+1] <= grid[cy][cx] + 1 {
                work.push_back((cx+1, cy, cost+1));
            }

            if cx > 0 && grid[cy][cx-1] <= grid[cy][cx] + 1 {
                work.push_back((cx-1, cy, cost+1));
            }

            if cy < height - 1 && grid[cy+1][cx] <= grid[cy][cx] + 1 {
                work.push_back((cx, cy+1, cost+1));
            }

            if cy > 0 && grid[cy-1][cx] <= grid[cy][cx] + 1 {
                work.push_back((cx, cy-1, cost+1));
            }
        }
    }

    all_costs.sort();
    dbg!(all_costs[0]);

//    for y in 0..height {
//        for x in 0..width {
//            if visisted.contains(&(x, y)) {
//                print!("#");
//            } else {
//                print!(".");
//            }
//        }
//        println!();
//    }
}
