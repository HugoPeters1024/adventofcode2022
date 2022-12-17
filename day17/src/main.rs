use std::{io::BufRead, collections::{HashSet, HashMap}, borrow::{BorrowMut, Borrow}};

#[derive(Debug)]
enum Dir { Left, Right }

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct State {
    rock_idx: usize,
    jet_idx: usize,
    rel_x: i64,
    y_to_go: i64,
    // as offsets from heighest_y
    heights: [u8; 7],
}

#[derive(Debug)]
struct CycleInfo {
    cycle_start: usize,
    cycle_len: usize,
    cycle_start_height: i64,
    cycle_added_height: i64,
    relative_heights: Vec<i64>,
}

fn main() {
    let jets: Vec<Dir> = std::io::stdin().lock().lines().next().unwrap().unwrap().chars().map(|c| match c {
        '<' => Dir::Left,
        '>' => Dir::Right,
        _ => panic!("invalid input"),
    }).collect();

    // uppperbound of the world is 2022 * 4 height

    let mut jet_idx = 0;
    let mut world : Vec<u8> = vec![0; 2022 * 4 * 7];
    let mut heighest_y = 0;
    
    let mut history : HashMap<State, (usize, i64)> = HashMap::new();

    let mut cycle : Option<CycleInfo> = None;

    for rock_idx in 0.. {
        if let Some(cycle_info) = cycle.as_mut() {
            cycle_info.relative_heights.push(heighest_y-cycle_info.cycle_start_height-cycle_info.cycle_added_height);
            if cycle_info.relative_heights.len() == cycle_info.cycle_len {
                break;
            }
        }

        let mut x = 2;
        let mut y = heighest_y + 3;

        let points = get_rock_points(rock_idx);

        loop {
            if cycle.is_none() {
                let state = State {
                    rock_idx: rock_idx % 5,
                    jet_idx : jet_idx % jets.len(),
                    rel_x: x,
                    y_to_go: y - heighest_y,
                    heights: get_height_footprint(&world, &heighest_y)
                };

                if let Some((rid, height)) = history.get(&state) {
                    println!("cycle at rock {} with increased height {} originally found at rock {} with height {}", rock_idx, heighest_y-height, rid, height);
                    cycle = Some(CycleInfo {
                        cycle_start: *rid,
                        cycle_len: rock_idx - rid,
                        cycle_start_height: *height,
                        cycle_added_height: (heighest_y - height),
                        relative_heights: vec![0],
                    });
                } else {
                    history.insert(state, (rock_idx, heighest_y));
                }
            }

            let mut xoff = match jets[jet_idx % jets.len()] {
                Dir::Left => -1,
                Dir::Right => 1,
            };
            jet_idx += 1;

            if !rock_ok(&points, x + xoff, y, &world) {
                xoff = 0;
            }

            x += xoff;

            if rock_ok(&points, x, y-1, &world) {
                y -= 1;
            } else {
                for (px, py) in &points {
                    let wx = px + x;
                    let wy = py + y;
                    heighest_y = heighest_y.max(wy+1);
                    world[(wy * 7 + wx) as usize] = 1;
                }
                break;
            }

        }
    }

    let info = cycle.unwrap();

    let want_to_know_at = 1000000000000;

    let nr_cycles = (want_to_know_at - info.cycle_start) / info.cycle_len;
    let mod_cycles = (want_to_know_at - info.cycle_start) % info.cycle_len;

    dbg!(nr_cycles);
    dbg!(mod_cycles);

    let total_height = info.cycle_start_height + nr_cycles as i64 * info.cycle_added_height + info.relative_heights[mod_cycles as usize];
    dbg!(total_height);
}

fn rock_ok(points: &Vec<(i64,i64)>, xoff: i64, yoff: i64, world: &Vec<u8>) -> bool {
    for (x,y) in points {
        let x = x + xoff;
        let y = y + yoff;
        if x < 0 || x >= 7 {
            return false;
        }

        if y < 0 {
            return false;
        }


        if world[(y * 7 + x) as usize] != 0 {
            return false;
        }
    }

    true
}

fn get_rock_points(rock_idx: usize) -> Vec<(i64, i64)> {
    let mut ret = Vec::new();
    match rock_idx % 5 {
        0 => {
            // horz line
            ret.push((0, 0));
            ret.push((1, 0));
            ret.push((2, 0));
            ret.push((3, 0));
        }
        1 => {
            // plus
            ret.push((1, 0));
            ret.push((1, 1));
            ret.push((1, 2));
            ret.push((0, 1));
            ret.push((2, 1));
        }
        2 => {
            // reverse L
            ret.push((0, 0));
            ret.push((1, 0));
            ret.push((2, 0));
            ret.push((2, 1));
            ret.push((2, 2));
        }
        3 => {
            // vert line
            ret.push((0, 0));
            ret.push((0, 1));
            ret.push((0, 2));
            ret.push((0, 3));
        }
        4 => {
            // square
            ret.push((0, 0));
            ret.push((1, 0));
            ret.push((1, 1));
            ret.push((0, 1));
        }
        _ => panic!("invalid rock_idx"),
    }

    ret
}

fn get_height_footprint(world: &Vec<u8>, heighest_y: &i64) -> [u8; 7] {
    let mut ret = [0; 7];
    for x in 0..7 {
        ret[x] = get_relative_height(world, x as i64, heighest_y);
    }

    ret
}

fn get_relative_height(world: &Vec<u8>, x: i64, heighest_y: &i64) -> u8 {
    let mut y = *heighest_y;
    let mut ret= 0;
    while y >= 0 {
        if world[(y * 7 + x) as usize] != 0 {
            return ret;
        }
        y -= 1;
        ret+=1;
    }

    0
}
