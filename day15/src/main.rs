use std::{io::BufRead, default, collections::HashSet};

use scanf::sscanf;


fn main() {
    let mut beacons : Vec<(i64, i64)> = Vec::new();
    let mut sensors : Vec<(i64, i64, i64)> = Vec::new();

    let mut minx = i64::MAX;
    let mut maxx = 0;

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut sx : i64 = 0;
        let mut sy : i64 = 0;
        let mut bx : i64 = 0;
        let mut by : i64 = 0;
        sscanf!(&line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by).unwrap();

        let dx = (sx - bx).abs();
        let dy = (sy - by).abs();
        let d = dx + dy;

        minx = minx.min(sx-d);
        maxx = maxx.max(sx+d);

        beacons.push((bx, by));
        sensors.push((sx, sy, d));
    }

    // part1
    //dbg!(minx);
    //dbg!(maxx);

    //let mut visisted : HashSet<i64> = HashSet::new();
    //const Y: i64 = 2000000;
    //for x in minx..=maxx {
    //    for (sx, sy, d) in &sensors {
    //        let dx = (sx - x).abs();
    //        let dy = (sy - Y).abs();
    //        if dx + dy <= *d {
    //            visisted.insert(x);
    //        }
    //    }
    //}

    //for (bx, by) in beacons {
    //    if by == Y {
    //        visisted.remove(&bx);
    //    }
    //}

    //dbg!(visisted.len());
    //

    const M : i64 = 4000000;
    
    // part2 loop over the outline of each sensor as
    // that should contain the desired point
    for (sx, sy, d) in &sensors {
        let mut cx = sx + d + 1;
        let mut cy = *sy;

        // right-up
        loop {
            if cx == *sx { break; }
            if !overlaps(&cx, &cy, &sensors) && cx >= 0 && cx <= M && cy >= 0 && cy <= M {
                println!("{}", cx * 4000000 + cy);
                return;
            }
            cx -= 1;
            cy -= 1;
        }

        // up-left
        loop {
            if cx == *sx - d  - 1{ break; }
            if !overlaps(&cx, &cy, &sensors) && cx >= 0 && cx <= M && cy >= 0 && cy <= M {
                println!("{}", cx * 4000000 + cy);
                return;
            }
            cx -= 1;
            cy += 1;
        }

        // left-down
        loop {
            if cx == *sx { break; }
            if !overlaps(&cx, &cy, &sensors) && cx >= 0 && cx <= M && cy >= 0 && cy <= M {
                println!("{}", cx * 4000000 + cy);
                return;
            }
            cx += 1;
            cy += 1;
        }

        // down-right
        loop {
            if cx == *sx + d + 1 { break; }
            if !overlaps(&cx, &cy, &sensors) && cx >= 0 && cx <= M && cy >= 0 && cy <= M {
                println!("{}", cx * 4000000 + cy);
                return;
            }
            cx += 1;
            cy -= 1;
        }
    }
}

fn overlaps(cx: &i64, cy: &i64, sensors: &Vec<(i64, i64, i64)>) -> bool {
    for (sx, sy, d) in sensors {
        let dx = (sx - cx).abs();
        let dy = (sy - cy).abs();
        if dx + dy <= *d {
            return true;
        }
    }
    false
}
