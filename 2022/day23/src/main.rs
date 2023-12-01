use std::{io::BufRead, collections::{HashMap, HashSet}};

fn main() {
    let mut WIDTH = 0;
    let mut elves : Vec<(i32,i32)> = Vec::new();

    let mut y = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        WIDTH = line.len() as i32;
        let mut x = 0;
        for c in line.chars() {
            match c {
                '#' => { elves.push((x,y)); },
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }

    let HEIGHT = y;

    let mut dir : i32 = 0;

    for round in 0.. {
        let elf_set : HashSet<(i32,i32)> = HashSet::from_iter(elves.iter().cloned());
        let mut new_elves: Vec<(i32,i32)> = elves.clone();

        let mut stale_count = 0;
        for (i,(ex,ey)) in elves.iter().cloned().enumerate() {
            let mut all_empty = true;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    if elf_set.contains(&(ex+dx, ey+dy)) {
                        all_empty = false;
                        break;
                    }
                }
            }

            if all_empty {
                stale_count += 1;
//                println!("Elf {} proposed not to move at all", i);
                continue;
            }

            let mut nx = ex;
            let mut ny = ey;


            for ldir in dir..dir+4 {
                let ldir = ldir % 4;
                // North
                if ldir == 0 {
                    if !elf_set.contains(&(ex-1, ey-1))
                    && !elf_set.contains(&(ex, ey-1))
                    && !elf_set.contains(&(ex+1, ey-1)) {
//                        println!("Elf {} proposed to move north", i);
                        ny -= 1;
                        break;
                    }
                }

                // South
                if ldir == 1 {
                    if !elf_set.contains(&(ex-1, ey+1))
                    && !elf_set.contains(&(ex, ey+1))
                    && !elf_set.contains(&(ex+1, ey+1)) {
//                        println!("Elf {} proposed to move south", i);
                        ny += 1;
                        break;
                    }
                }

                // West
                if ldir == 2 {
                    if !elf_set.contains(&(ex-1, ey-1))
                    && !elf_set.contains(&(ex-1, ey))
                    && !elf_set.contains(&(ex-1, ey+1)) {
//                        println!("Elf {} proposed to move west", i);
                        nx -= 1;
                        break;
                    }
                }

                // East
                if ldir ==3 {
                    if !elf_set.contains(&(ex+1, ey-1))
                    && !elf_set.contains(&(ex+1, ey))
                    && !elf_set.contains(&(ex+1, ey+1)) {
//                        println!("Elf {} proposed to move east", i);
                        nx += 1;
                        break;
                    }
                }
            }
            new_elves[i] = (nx,ny);
        }

//        println!("-------");

        let overlap: HashMap<(i32,i32),i32> = new_elves.iter().fold(HashMap::new(), |mut acc, (x,y)| {
            *acc.entry((*x,*y)).or_insert(0) += 1;
            acc
        });

        for (i, (nx, ny)) in new_elves.iter().cloned().enumerate() {
            if overlap.get(&(nx,ny)).unwrap() == &1 {
                elves[i] = (nx,ny);
            }
        }

        if stale_count == elf_set.len() {
            println!("Nobody moved in round {}", round+1);
            break;
        }

        dir += 1;
    }


    let min_x: i32 = *elves.iter().map(|(x,_)| x).min().unwrap();
    let max_x: i32 = elves.iter().map(|(x,_)| x).max().unwrap() + 1;
    let min_y: i32 = *elves.iter().map(|(_,y)| y).min().unwrap();
    let max_y: i32 = elves.iter().map(|(_,y)| y).max().unwrap() + 1;

    for y in min_y..max_y {
        for x in min_x..max_x {
            if elves.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    dbg!((min_x - max_x).abs() * (min_y - max_y).abs() - elves.len() as i32);
}
