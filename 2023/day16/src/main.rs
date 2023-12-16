use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn delta(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}

fn energy_number(grid: &Vec<Vec<char>>, initial: (isize, isize, Dir)) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut seen: HashSet<(usize, usize, Dir)> = HashSet::new();

    let mut work: VecDeque<(isize, isize, Dir)> = VecDeque::new();
    work.push_back(initial);

    while let Some((x, y, dir)) = work.pop_front() {
        if x < 0 || y < 0 || x >= width as isize || y >= height as isize {
            continue;
        }

        if seen.contains(&(x as usize, y as usize, dir)) {
            continue;
        }
        seen.insert((x as usize, y as usize, dir));

        let (dx, dy) = dir.delta();
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        match (dir, grid[y as usize][x as usize]) {
            (_, '.') => {
                work.push_front((nx, ny, dir));
            }

            (Dir::Left | Dir::Right, '|') => {
                work.push_front((x, y - 1, Dir::Up));
                work.push_front((x, y + 1, Dir::Down));
            }
            (Dir::Up | Dir::Down, '|') => {
                work.push_front((nx, ny, dir));
            }
            (Dir::Up | Dir::Down, '-') => {
                work.push_front((x + 1, y, Dir::Right));
                work.push_front((x - 1, y, Dir::Left));
            }
            (Dir::Left | Dir::Right, '-') => {
                work.push_front((nx, ny, dir));
            }
            (Dir::Left, '\\') => work.push_front((x, y - 1, Dir::Up)),
            (Dir::Left, '/') => work.push_front((x, y + 1, Dir::Down)),
            (Dir::Right, '\\') => work.push_front((x, y + 1, Dir::Down)),
            (Dir::Right, '/') => work.push_front((x, y - 1, Dir::Up)),
            (Dir::Up, '\\') => work.push_front((x - 1, y, Dir::Left)),
            (Dir::Up, '/') => work.push_front((x + 1, y, Dir::Right)),
            (Dir::Down, '\\') => work.push_front((x + 1, y, Dir::Right)),
            (Dir::Down, '/') => work.push_front((x - 1, y, Dir::Left)),
            _ => panic!("{}, {}, {:?}, {}", x, y, dir, grid[y as usize][x as usize]),
        }
    }

    let seen_no_dir: HashSet<(usize, usize)> = seen.iter().map(|(x, y, _)| (*x, *y)).collect();

    seen_no_dir.len()
}

fn main() {
    let grid: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    println!("Part 1 {}", energy_number(&grid, (0, 0, Dir::Right)));

    let mut highest = 0;

    // Left side
    for y in 0..height {
        highest = highest.max(energy_number(&grid, (0, y as isize, Dir::Right)));
    }

    // Right side
    for y in 0..height {
        highest = highest.max(energy_number(
            &grid,
            ((width - 1) as isize, y as isize, Dir::Left),
        ));
    }

    // Top side
    for x in 0..width {
        highest = highest.max(energy_number(&grid, (x as isize, 0, Dir::Down)));
    }

    // Bottom side
    for x in 0..width {
        highest = highest.max(energy_number(
            &grid,
            (x as isize, (height - 1) as isize, Dir::Up),
        ));
    }

    println!("Part 2 {}", highest);
}
