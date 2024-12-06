use std::{collections::HashSet, io::BufRead};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn get_delta(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        }
    }

    fn rotate_r(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

fn count_till_escape(board: &Vec<Vec<char>>, mut ppos: (isize, isize), mut pdir: Dir) -> Option<usize> {
    let width = board[0].len() as isize;
    let height = board.len() as isize;
    let mut visisted: HashSet<((isize, isize), Dir)> = HashSet::new();
    loop {
        if visisted.contains(&(ppos, pdir)) {
            return None
        }
        visisted.insert((ppos, pdir));

        let d = pdir.get_delta();
        let nx = ppos.0 + d.0;
        let ny = ppos.1 + d.1;

        if nx < 0 || nx >= width || ny < 0 || ny >= height {
            return Some(visisted.iter().map(|x| x.0).collect::<HashSet<_>>().len());
        }

        if board[ny as usize][nx as usize] == '#' {
            pdir = pdir.rotate_r();
        } else {
            ppos.0 = nx;
            ppos.1 = ny;
        }
    }

}

fn main() {
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut ppos = (0, 0);
    let pdir = Dir::Up;

    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        let mut tmp = Vec::new();
        for (x, char) in line.unwrap().chars().enumerate() {
            if char != '^' {
                tmp.push(char);
            } else {
                tmp.push('.');
                ppos.0 = x as isize;
                ppos.1 = y as isize;
            }
        }

        board.push(tmp);
    }

    let ppos = ppos;


    println!("Part 1: {}", count_till_escape(&board, ppos, pdir).unwrap());

    let width = board[0].len() as isize;
    let height = board.len() as isize;

    let mut paradox_count = 0;
    for y in 0..height {
        for x in 0..width {
            if board[y as usize][x as usize] == '.' && (x,y) != ppos {
                board[y as usize][x as usize] = '#';
                if count_till_escape(&board, ppos, pdir).is_none() {
                    paradox_count += 1;
                }
                board[y as usize][x as usize] = '.';
            }
        }
    }

    println!("Part 2: {}", paradox_count);
}
