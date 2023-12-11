use std::{
    collections::{HashSet, VecDeque, HashMap},
    io::BufRead,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Winding {
    Clockwise,
    CounterClockwise,
}


struct Puzzle {
    fields: Vec<Vec<char>>,
    width: usize,
    height: usize,
    startx: usize,
    starty: usize,
}

fn iter_dirs() -> impl Iterator<Item = (isize, isize)> {
    vec![(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter()
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

    let puzzle = Puzzle {
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

    let mut coord_to_horz = HashMap::new();
    for i in 0..path.len() {
        let (x, y) = path[i];
        let (xn, yn) = path[(i + 1) % path.len()];
        if x == xn {
            continue;
        }

        if x < xn {
            coord_to_horz.insert((x,y), Winding::Clockwise);
            coord_to_horz.insert((xn,yn), Winding::Clockwise);
        } else {
            coord_to_horz.insert((x,y), Winding::CounterClockwise);
            coord_to_horz.insert((xn,yn), Winding::CounterClockwise);
        }
    }

    let path_lookup: HashSet<(usize, usize)> = path.iter().cloned().collect();

    let mut inside: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..grid_height {
        for x in 0..grid_width {
            if puzzle.fields[y][x] != '.' {
                if !path_lookup.contains(&(x, y)) {
                    inside.insert((x, y));
                }
                continue;
            }


            let debug = x == 16 && y == 0;

            if debug {
                println!("Starting at {}, {}", x, y);
            }

            let mut winding_number = 0;
            let mut ys = y as isize;
            loop {
                ys -= 1;
                if ys < 0 {
                    break;
                }

                if let Some(winding) = coord_to_horz.get(&(x, ys as usize)) {
                    if puzzle.fields[ys as usize][x] == 'L' {
                        continue;
                    }
                    if puzzle.fields[ys as usize][x] == 'F' {
                        continue;
                    }
                    if debug {
                        println!("Winding at {}, {} is {:?}", x, ys, winding);
                    }
                    match winding {
                        Winding::Clockwise => winding_number += 1,
                        Winding::CounterClockwise => winding_number += 1,
                    }
                }
            }

            if winding_number % 2 != 0 {
                inside.insert((x, y as usize));
            }
        }
    }

    println!("Part 2: {}", inside.len());

    for y in 0..grid_height {
        for x in 0..grid_width {
            if inside.contains(&(x, y)) {
                print!("I");
            } else {
                if let Some(winding) = coord_to_horz.get(&(x, y)) {
                    match winding {
                        Winding::Clockwise => print!(">"),
                        Winding::CounterClockwise => print!("<"),
                    }
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}
