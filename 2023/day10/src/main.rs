use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

struct Puzzle {
    fields: Vec<Vec<char>>,
    width: usize,
    height: usize,
    startx: usize,
    starty: usize,
}

fn iter_dirs() -> impl Iterator<Item = (isize, isize)> {
    vec![(1, 0), (0, -1), (-1, 0), (0, 1)].into_iter()
}

fn turn_left(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (0, 1) => (-1, 0),
        (1, 0) => (0, 1),
        (0, -1) => (1, 0),
        (-1, 0) => (0, -1),
        _ => panic!("Invalid direction"),
    }
}

fn turn_right(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        _ => panic!("Invalid direction"),
    }
}

fn valid_move(puzzle: &Puzzle, x1: usize, y1: usize, dir: &(isize, isize)) -> bool {
    let (x2, y2) = (x1 as isize + dir.0, y1 as isize + dir.1);
    if x2 < 0 || y2 < 0 || x2 >= puzzle.width as isize || y2 >= puzzle.height as isize {
        //println!("Out of bounds");
        return false;
    }

    let can_move_out = match (puzzle.fields[y1][x1], dir.0, dir.1) {
        ('|', 0, _) => true,
        ('-', _, 0) => true,
        ('L', 0, -1) => true,
        ('L', 1, 0) => true,
        ('J', 0, -1) => true,
        ('J', -1, 0) => true,
        ('7', 0, 1) => true,
        ('7', -1, 0) => true,
        ('F', 0, 1) => true,
        ('F', 1, 0) => true,
        ('S', _, _) => true,
        (_, _, _) => false,
    };

    if !can_move_out {
        //println!("Can't move out {}", puzzle.fields[y1][x1]);
        return false;
    }

    let can_move_in = match (puzzle.fields[y2 as usize][x2 as usize], dir.0, dir.1) {
        ('|', 0, _) => true,
        ('-', _, 0) => true,
        ('L', 0, 1) => true,
        ('L', -1, 0) => true,
        ('J', 0, 1) => true,
        ('J', 1, 0) => true,
        ('7', 0, -1) => true,
        ('7', 1, 0) => true,
        ('F', 0, -1) => true,
        ('F', -1, 0) => true,
        ('S', _, _) => true,
        (_, _, _) => false,
    };

    if !can_move_in {
        //println!("Can't move in {}", puzzle.fields[y2 as usize][x2 as usize]);
        return false;
    }

    can_move_out && can_move_in
}

fn find_path(
    current_dir: (isize, isize),
    puzzle: &Puzzle,
    path: &mut VecDeque<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
) -> bool {
    let (x, y) = *path.back().unwrap();

    if x == puzzle.startx && y == puzzle.starty && path.len() > 1 {
        return true;
    }

    if visited.contains(&(x, y)) {
        return false;
    }
    visited.insert((x, y));

    for ndir in &[current_dir, turn_left(current_dir), turn_right(current_dir)] {
        if !valid_move(puzzle, x, y, ndir) {
            //println!("Invalid move {:?}", ndir);
            continue;
        }

        let (nx, ny) = (x as isize + ndir.0, y as isize + ndir.1);
        let (nx, ny) = (nx as usize, ny as usize);

        path.push_back((nx, ny));
        if find_path(*ndir, puzzle, path, visited) {
            return true;
        }
        path.pop_back();
        visited.remove(&(nx, ny));
    }

    false
}

fn main() {
    let grid: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let (mut startx, mut starty) = (0, 0);

    for y in 0..grid_height {
        for x in 0..grid_width {
            if grid[y][x] == 'S' {
                startx = x;
                starty = y;
            }
        }
    }

    let mut puzzle = Puzzle {
        fields: grid,
        width: grid_width,
        height: grid_height,
        startx,
        starty,
    };

    let mut path = VecDeque::new();
    for dir in iter_dirs() {
        let mut visited = HashSet::new();
        path.clear();
        path.push_back((startx, starty));
        let found_path = find_path(dir, &puzzle, &mut path, &mut visited);

        if found_path {
            println!("Part 1: {}", path.len() / 2);
            break;
        }
    }

    let path_lookup: HashSet<(usize, usize)> = HashSet::from_iter(path.iter().cloned());

    let exits: HashSet<(usize, usize)> = HashSet::from_iter([path[1], path[path.len() - 2]]);
    dbg!(&exits);

    // Replace S with the right piece of pipe
    let exits_bits = iter_dirs()
        .map(|(dx, dy)| (startx as isize + dx, starty as isize + dy))
        .map(|(x, y)| (x as usize, y as usize))
        .map(|(x, y)| if exits.contains(&(x, y)) { 1 } else { 0 })
        .collect::<Vec<_>>();

    dbg!(&exits_bits);

    match exits_bits.as_slice() {
        [1, 1, 0, 0] => puzzle.fields[starty][startx] = 'L',
        [1, 0, 1, 0] => puzzle.fields[starty][startx] = '-',
        [1, 0, 0, 1] => puzzle.fields[starty][startx] = 'F',
        [0, 1, 1, 0] => puzzle.fields[starty][startx] = 'J',
        [0, 1, 0, 1] => puzzle.fields[starty][startx] = '|',
        [0, 0, 1, 1] => puzzle.fields[starty][startx] = '7',
        _ => panic!("Invalid exit configuration"),
    }

    // Find the number of tiles enclosed by path using the Even-Odd rule
    let mut enclosed_tiles = HashSet::new();
    for y in 0..grid_height {
        for x in 0..grid_width {
            if path_lookup.contains(&(x, y)) {
                continue;
            }

            let mut xs = x;
            let mut crossing_count = 0;
            loop {
                xs += 1;
                if xs >= grid_width {
                    break;
                }

                if ['-', 'F', '7'].contains(&puzzle.fields[y][xs]) {
                    continue;
                }

                if path_lookup.contains(&(xs, y)) {
                    crossing_count += 1;
                }
            }

            if crossing_count % 2 == 1 {
                enclosed_tiles.insert((x, y));
            }
        }
    }

    for y in 0..grid_height {
        for x in 0..grid_width {
            if path_lookup.contains(&(x, y)) {
                print!("\x1b[31m{}\x1b[0m", puzzle.fields[y][x]);
            } else if enclosed_tiles.contains(&(x, y)) {
                print!("\x1b[32m{}\x1b[0m", puzzle.fields[y][x]);
            } else {
                print!("{}", puzzle.fields[y][x]);
            }
        }
        println!();
    }

    println!("Part 2: {}", enclosed_tiles.len());
}
