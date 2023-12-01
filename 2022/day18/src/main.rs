use std::{collections::{HashSet, HashMap, VecDeque}, io::BufRead};
use scanf::sscanf;

fn main() {
    let mut cubes: Vec<(i32,i32,i32)> = Vec::new();

    let mut vmin = (i32::MAX,i32::MAX,i32::MAX);
    let mut vmax = (i32::MIN,i32::MIN,i32::MIN);

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut x : i32 = 0;
        let mut y : i32 = 0;
        let mut z : i32 = 0;
        sscanf!(&line, "{},{},{}", x, y, z).unwrap();
        cubes.push((x,y,z));

        vmin.0 = vmin.0.min(x);
        vmin.1 = vmin.1.min(y);
        vmin.2 = vmin.2.min(z);

        vmax.0 = vmax.0.max(x);
        vmax.1 = vmax.1.max(y);
        vmax.2 = vmax.2.max(z);
    }

    let cube_set : HashSet<(i32, i32, i32)> = HashSet::from_iter(cubes.iter().cloned());

    // part 1
    // optimistic estimate
    let mut total = cubes.len() * 6;

    // correct
    for (x,y,z) in cubes.iter().cloned() {
        if cube_set.contains(&(x+1,y,z)) { total -= 1; }
        if cube_set.contains(&(x-1,y,z)) { total -= 1; }
        if cube_set.contains(&(x,y+1,z)) { total -= 1; }
        if cube_set.contains(&(x,y-1,z)) { total -= 1; }
        if cube_set.contains(&(x,y,z+1)) { total -= 1; }
        if cube_set.contains(&(x,y,z-1)) { total -= 1; }
    }
    dbg!(total);

    // part 2
    // eliminate airbubbles
    let mut around : HashSet<(i32,i32,i32)> = HashSet::new();

    for (x,y,z) in &cubes {
        let offsets = vec![
            (1,0,0), (-1,0,0),
            (0,1,0), (0,-1,0),
            (0,0,1), (0,0,-1),
        ];
        for (dx, dy, dz) in offsets {
            let (x,y,z) = (*x+dx, *y+dy, *z+dz);
            if inside(x,y,z,vmin,vmax) && !cube_set.contains(&(x,y,z)) {
                around.insert((x,y,z));
            }
        }
    }

    let mut in_bubble: HashSet<(i32,i32,i32)> = HashSet::new();
    let mut outside: HashSet<(i32,i32,i32)> = HashSet::new();

    dbg!(around.len());

    for (x,y,z) in around.iter().cloned() {
        let mut work : VecDeque<(i32,i32,i32)> = VecDeque::new();
        work.push_front((x,y,z));
        let mut visisted : HashSet<(i32,i32,i32)> = HashSet::new();

        loop {
            if work.len() > 100000 { break; }
            if let Some((x,y,z)) = work.pop_front() {
                if cube_set.contains(&(x,y,z)) { continue; }
                if visisted.contains(&(x,y,z)) { continue; }
                if outside.contains(&(x,y,z)) { continue; }
                if in_bubble.contains(&(x,y,z)) { continue; }
                visisted.insert((x,y,z));

                if x == 0 && y == 0 && z == 0 {
                    outside = outside.union(&visisted).cloned().collect();
                    break;
                }

                if x < 0 || y < 0 || z < 0 {
                    panic!();
                }

                if x > y && x > z {
                    work.push_front((x-1,y,z));
                    work.push_front((x,y-1,z));
                    work.push_front((x,y,z-1));
                    work.push_front((x+1,y,z));
                    work.push_front((x,y+1,z));
                    work.push_front((x,y,z+1));
                } else if y > x && y > z {
                    work.push_front((x,y-1,z));
                    work.push_front((x-1,y,z));
                    work.push_front((x,y,z-1));
                    work.push_front((x+1,y,z));
                    work.push_front((x,y+1,z));
                    work.push_front((x,y,z+1));
                } else {
                    work.push_front((x,y,z-1));
                    work.push_front((x,y-1,z));
                    work.push_front((x-1,y,z));
                    work.push_front((x+1,y,z));
                    work.push_front((x,y+1,z));
                    work.push_front((x,y,z+1));
                }
            } else {
                in_bubble = in_bubble.union(&visisted).cloned().collect();
                break;
            }
        }
    }

    dbg!(in_bubble.len());
    let mut air_total = in_bubble.len() * 6;

    for (x,y,z) in in_bubble.iter().cloned() {
        if in_bubble.contains(&(x+1,y,z)) { air_total -= 1; }
        if in_bubble.contains(&(x-1,y,z)) { air_total -= 1; }
        if in_bubble.contains(&(x,y+1,z)) { air_total -= 1; }
        if in_bubble.contains(&(x,y-1,z)) { air_total -= 1; }
        if in_bubble.contains(&(x,y,z+1)) { air_total -= 1; }
        if in_bubble.contains(&(x,y,z-1)) { air_total -= 1; }
    }


    total -= air_total;

    dbg!(total);
}

fn inside(x: i32, y: i32, z: i32, vmin: (i32,i32,i32), vmax: (i32,i32,i32)) -> bool {
    x >= vmin.0 && x <= vmax.0 &&
    y >= vmin.1 && y <= vmax.1 &&
    z >= vmin.2 && z <= vmax.2
}
