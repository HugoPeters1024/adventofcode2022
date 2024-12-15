use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

fn solve(mut level: Vec<Vec<char>>, instructions: &Vec<char>) -> usize {
    let width = level[0].len();
    let height = level.len();

    let mut player_pos = (0isize, 0isize);
    for y in 0..height {
        for x in 0..width {
            if level[y][x] == '@' {
                player_pos = (x as isize, y as isize);
            }
        }
    }

    for instruction in instructions {
        //for line in &level {
        //    for c in line {
        //        print!("{}", c);
        //    }
        //    println!();
        //}
        //println!();
        //dbg!(instruction);

        let (dx, dy): (isize, isize) = match instruction {
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => panic!(),
        };

        let mut push_length = 0;
        while level[(player_pos.1 + dy + push_length * dy) as usize]
            [(player_pos.0 + dx + push_length * dx) as usize]
            == 'O'
        {
            push_length += 1;
        }

        if level[(player_pos.1 + dy + push_length * dy) as usize]
            [(player_pos.0 + dx + push_length * dx) as usize]
            != '.'
        {
            continue;
        }

        // move the player
        level[player_pos.1 as usize][player_pos.0 as usize] = '.';
        level[(player_pos.1 + dy) as usize][(player_pos.0 + dx) as usize] = '@';
        player_pos.0 += dx;
        player_pos.1 += dy;

        // move push length number of boxes
        if push_length > 0 {
            level[(player_pos.1 + push_length * dy) as usize]
                [(player_pos.0 + push_length * dx) as usize] = 'O';
        }
    }

    for line in &level {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!();

    let mut sum = 0;
    for (y, line) in level.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'O' {
                sum += 100 * y + x;
            }
        }
    }

    sum
}

fn solve2(mut level: Vec<Vec<char>>, instructions: &Vec<char>) -> usize {
    let width = level[0].len();
    let height = level.len();

    let mut player_pos = (0isize, 0isize);
    for y in 0..height {
        for x in 0..width {
            if level[y][x] == '@' {
                player_pos = (x as isize, y as isize);
            }
        }
    }

    for instruction in instructions {
        //for line in &level {
        //    for c in line {
        //        print!("{}", c);
        //    }
        //    println!();
        //}
        //println!();
        //dbg!(instruction);

        let (dx, dy): (isize, isize) = match instruction {
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => panic!(),
        };

        let mut to_be_moved = Vec::new();
        let mut work: VecDeque<(isize, isize)> = VecDeque::new();

        work.push_front((player_pos.0, player_pos.1));
        let mut possible = true;

        while let Some((x, y)) = work.pop_front() {
            let nx = (x + dx) as usize;
            let ny = (y + dy) as usize;

            if level[ny][nx] == '#' {
                possible = false;
                break;
            }

            to_be_moved.push((x, y, level[y as usize][x as usize]));

            if level[ny][nx] == '.' {
                continue;
            }

            if level[ny][nx] == '[' {
                work.push_front((nx as isize, ny as isize));

                if dy != 0 {
                    work.push_front(((nx + 1) as isize, ny as isize));
                }
            }

            if level[ny][nx] == ']' {
                work.push_front((nx as isize, ny as isize));

                if dy != 0 {
                    work.push_front(((nx - 1) as isize, ny as isize));
                }
            }
        }

        if !possible {
            continue;
        }

        player_pos.0 += dx;
        player_pos.1 += dy;
        for (x, y, _) in &to_be_moved {
            level[*y as usize][*x as usize] = '.';
        }

        for (x, y, c) in &to_be_moved {
            level[(y + dy) as usize][(x + dx) as usize] = *c;
        }
    }

    for line in &level {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!();

    let mut sum = 0;
    for (y, line) in level.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '[' {
                sum += 100 * y + x;
            }
        }
    }

    sum
}

fn main() {
    let mut level: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();

    let mut level_done = false;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            level_done = true;
            continue;
        }

        if !level_done {
            level.push(line.chars().collect());
        } else {
            let mut to_add = line.chars().collect();
            instructions.append(&mut to_add);
        }
    }

    let answer = solve(level.clone(), &instructions);
    println!("Part 1: {answer}");

    let mut level2: Vec<Vec<char>> = Vec::new();
    for line in &level {
        let mut to_add = Vec::new();
        for c in line {
            if *c == '@' {
                to_add.push('@');
                to_add.push('.');
            } else if *c == 'O' {
                to_add.push('[');
                to_add.push(']');
            } else {
                to_add.push(*c);
                to_add.push(*c);
            }
        }
        level2.push(to_add);
    }

    let answer = solve2(level2.clone(), &instructions);
    println!("Part 2: {answer}");
}
