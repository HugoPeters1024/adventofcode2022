use std::{collections::VecDeque, io::BufRead};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn as_num(&self) -> usize {
        match self {
            Dir::Up => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Right => 3,
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    fn next_pos(&self, x: usize, y: usize, width: usize, height: usize) -> Option<(usize, usize)> {
        match self {
            Dir::Up => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Dir::Down => {
                if y < height - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Dir::Left => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Dir::Right => {
                if x < width - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    cost: i32,
    straight_streak: i32,
    x: usize,
    y: usize,
    dir: Dir,
}

fn solve(grid: &Vec<Vec<i32>>, min_streak: i32, max_streak: i32) {
    let width = grid[0].len();
    let height = grid.len();

    let mut work = VecDeque::new();
    work.push_front(State {
        cost: -grid[0][0],
        straight_streak: 0,
        x: 0,
        y: 0,
        dir: Dir::Right,
    });
    work.push_back(State {
        cost: -grid[0][0],
        straight_streak: 0,
        x: 0,
        y: 0,
        dir: Dir::Down,
    });

    let mut visited: Vec<Vec<[i32; 120]>> = vec![vec![[i32::MAX; 120]; width]; height];

    let mut min = i32::MAX;
    while let Some(w) = work.pop_front() {
        let mut w = w.clone();
        w.cost += grid[w.y][w.x];

        if visited[w.y][w.x][w.dir.as_num() + 10 * (w.straight_streak as usize)] <= w.cost {
            continue;
        }
        visited[w.y][w.x][w.dir.as_num() + 10 * (w.straight_streak as usize)] = w.cost;

        if w.x == grid.len() - 1 && w.y == grid[0].len() - 1 {
            min = min.min(w.cost);
            continue;
        }

        if let Some((nx, ny)) = w.dir.next_pos(w.x, w.y, width, height) {
            if w.straight_streak < max_streak - 1 {
                let mut w = w.clone();
                w.straight_streak += 1;
                w.x = nx;
                w.y = ny;
                work.push_back(w);
            }
        }

        if w.straight_streak >= min_streak - 1 {
            let left = w.dir.turn_left();

            if let Some((nx, ny)) = left.next_pos(w.x, w.y, width, height) {
                let mut w = w.clone();
                w.straight_streak = 0;
                w.x = nx;
                w.y = ny;
                w.dir = left;
                work.push_back(w);
            }

            let right = w.dir.turn_right();

            if let Some((nx, ny)) = right.next_pos(w.x, w.y, width, height) {
                let mut w = w.clone();
                w.straight_streak = 0;
                w.x = nx;
                w.y = ny;
                w.dir = right;
                work.push_back(w);
            }
        }
    }
    println!("Part 1: {}", min);
}

fn main() {
    let grid: Vec<Vec<i32>> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    solve(&grid, 0, 3);
    solve(&grid, 4, 10);
}
