use std::{collections::{HashSet, HashMap}, io::BufRead};
use scanf::sscanf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Blocked {
    Rock,
    Sand,
}

fn main() {
    let mut blocked: HashMap<(i32, i32), Blocked> = HashMap::new();

    let mut maxy = 0;

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut parts: Vec<(i32,i32)> = Vec::new();

        for elem in line.split(" -> ") {
            let mut x : i32 = 0;
            let mut y : i32 = 0;
            sscanf!(elem, "{},{}", x, y).unwrap();
            maxy = maxy.max(y);
            parts.push((x, y));
        }

        let parts = parts;
        let (mut cx, mut cy) = parts[0];
        for (nx, ny) in parts.iter().skip(1) {
            let dx = (nx - cx).signum();
            let dy = (ny - cy).signum();

            blocked.insert((cx, cy), Blocked::Rock);
            while cx != *nx || cy != *ny {
                cx += dx;
                cy += dy;
                blocked.insert((cx, cy), Blocked::Rock);
            }

        }
    }


    // part 1
    //let mut count = 0;
    //'outer: loop {
    //    let (mut sx, mut sy) = (500, 0);
    //    loop {
    //        if blocked.get(&(sx, sy+1)).is_none() {
    //            sy += 1;
    //        } else if blocked.get(&(sx-1, sy+1)).is_none() {
    //            sx -= 1;
    //            sy += 1;
    //        } else if blocked.get(&(sx+1, sy+1)).is_none() {
    //            sx += 1;
    //            sy += 1;
    //        } else {
    //            count += 1;
    //            blocked.insert((sx, sy), Blocked::Sand);
    //            break;
    //        }


    //        if sy > maxy {
    //            dbg!(count);
    //            break 'outer;
    //        }
    //    }

    //    //for y in 0..=9 {
    //    //    for x in 494..=503 {
    //    //        print!("{}", match blocked.get(&(x, y)) {
    //    //            Some(Blocked::Rock) => '#',
    //    //            Some(Blocked::Sand) => 'O',
    //    //            None => '.',
    //    //        });
    //    //    }
    //    //    println!();
    //    //}
    //}

    // part 2
    let mut count = 0;
    while blocked.get(&(500, 0)).is_none() {
        let (mut sx, mut sy) = (500, 0);
        loop {
            if sy == maxy + 1 {
                count += 1;
                blocked.insert((sx, sy), Blocked::Sand);
                break;
            } else if blocked.get(&(sx, sy+1)).is_none() {
                sy += 1;
            } else if blocked.get(&(sx-1, sy+1)).is_none() {
                sx -= 1;
                sy += 1;
            } else if blocked.get(&(sx+1, sy+1)).is_none() {
                sx += 1;
                sy += 1;
            } else {
                count += 1;
                blocked.insert((sx, sy), Blocked::Sand);
                break;
            } 
        }
    }

    dbg!(count);
}
