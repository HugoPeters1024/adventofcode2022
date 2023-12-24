use std::{io::BufRead, collections::HashSet};

use scanf::sscanf;

#[derive(Debug, Clone)]
struct Brick {
    minx: i32,
    miny: i32,
    minz: i32,
    maxx: i32,
    maxy: i32,
    maxz: i32,
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        self.minx <= other.maxx
            && self.maxx >= other.minx
            && self.miny <= other.maxy
            && self.maxy >= other.miny
            && self.minz <= other.maxz
            && self.maxz >= other.minz
    }
}

fn render_bricks_xz(bricks: &[Brick]) {
    let minx = bricks.iter().map(|b| b.minx).min().unwrap();
    let maxx = bricks.iter().map(|b| b.maxx).max().unwrap();

    let minz = bricks.iter().map(|b| b.minz).min().unwrap();
    let maxz = bricks.iter().map(|b| b.maxz).max().unwrap();

    for z in (minz..=maxz+1).rev() {
        for x in minx..=maxx+1 {
            let mut found = false;
            for brick in bricks {
                if brick.minx <= x && brick.maxx >= x && brick.minz <= z && brick.maxz >= z {
                    found = true;
                    break;
                }
            }

            if found {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}




fn settle(bricks: &mut Vec<Brick>, ignore: Option<usize>) -> HashSet<usize> {
    let mut moved = HashSet::new();
    let mut moves = 0;
    let mut prev_moves = 0;
    loop {
        'outer: for i in 0..bricks.len() {
            if ignore == Some(i) {
                continue;
            }

            if bricks[i].minz > 1 {
                let mut new_brick = bricks[i].clone();
                new_brick.minz -= 1;
                new_brick.maxz -= 1;

                let mut valid = true;
                for j in 0..bricks.len() {
                    if i == j || ignore == Some(j) {
                        continue;
                    }

                    if new_brick.overlaps(&bricks[j]) {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    bricks[i] = new_brick;
                    moves += 1;
                    moved.insert(i);
                    if moves % 1000 == 0 {
                        println!("moves: {}", moves);
                    }
                    break 'outer;
                }
            }
        }

        if prev_moves == moves {
            break;
        }
        prev_moves = moves;
    }

    moved
}

fn main() {
    let mut bricks: Vec<Brick> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (mut x1, mut y1, mut z1, mut x2, mut y2, mut z2) = (0, 0, 0, 0, 0, 0);

            sscanf!(&l, "{},{},{}~{},{},{}", x1, y1, z1, x2, y2, z2).unwrap();

            Brick {
                minx: std::cmp::min(x1, x2),
                miny: std::cmp::min(y1, y2),
                minz: std::cmp::min(z1, z2),
                maxx: std::cmp::max(x1, x2),
                maxy: std::cmp::max(y1, y2),
                maxz: std::cmp::max(z1, z2),
            }
        })
        .collect();

    settle(&mut bricks, None);

    let mut count = 0;
    let mut fall_count = 0;
    for i in 0..bricks.len() {
        let mut copy = bricks.clone();
        let moved = settle(&mut copy, Some(i));

        if moved.len() == 0 {
            println!("brick {} is removable", i+1);
            count += 1;
        } else {
            println!("brick {} would cause {} bricks to fall", i+1, moved.len());
            fall_count += moved.len();
        }
    }

    println!("Part 1: {}", count);
    println!("Part 2: {}", fall_count);
}
