use std::{io::BufRead, collections::HashSet};
use scanf::sscanf;

fn main() {
    // part1
    //let mut head_pos = (0,0);
    //let mut tail_pos = (0,0);

    //let mut visisted : HashSet<(i32,i32)> = HashSet::new();
    //visisted.insert(tail_pos);

    //for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
    //    let mut dir = 'U';
    //    let mut steps = 0;
    //    sscanf!(&line, "{} {}", dir, steps).unwrap();

    //    for _ in 0..steps {
    //        match dir {
    //            'U' => head_pos.1 += 1,
    //            'D' => head_pos.1 -= 1,
    //            'L' => head_pos.0 -= 1,
    //            'R' => head_pos.0 += 1,
    //            _ => panic!("Invalid direction"),
    //        }
    //        
    //        let dx = head_pos.0 - tail_pos.0;
    //        let dy = head_pos.1 - tail_pos.1;

    //        if dx.abs() > 1 || dy.abs() > 1 {
    //            tail_pos.0 += dx.signum();
    //            tail_pos.1 += dy.signum();
    //        }


    //        println!("{}, {} (head moved {} and now at {},{})", tail_pos.0, tail_pos.1, dir, head_pos.0, head_pos.1);
    //        visisted.insert(tail_pos);
    //    }
    //}

    //dbg!(visisted.len());
    
    // part2
    let rope_length = 10;
    let tail_idx = rope_length - 1;
    let mut knots : Vec<(i32,i32)> = vec![(0,0); rope_length];

    let mut visisted : HashSet<(i32,i32)> = HashSet::new();
    visisted.insert(knots[tail_idx]);

    for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
        let mut dir = 'U';
        let mut steps = 0;
        sscanf!(&line, "{} {}", dir, steps).unwrap();

        for _ in 0..steps {
            match dir {
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                'L' => knots[0].0 -= 1,
                'R' => knots[0].0 += 1,
                _ => panic!("Invalid direction"),
            }
            
            for i in 1..rope_length {
                let dx = knots[i-1].0 - knots[i].0;
                let dy = knots[i-1].1 - knots[i].1;

                if dx.abs() > 1 || dy.abs() > 1 {
                    knots[i].0 += dx.signum();
                    knots[i].1 += dy.signum();
                }
            }

            println!("tail at {},{}", knots[tail_idx].0, knots[tail_idx].1);

            visisted.insert(knots[tail_idx]);
        }
    }

    //visualise
    let minx = *visisted.iter().map(|(x,_)| x).min().unwrap();
    let maxx = *visisted.iter().map(|(x,_)| x).max().unwrap();

    let miny = *visisted.iter().map(|(_,y)| y).min().unwrap();
    let maxy = *visisted.iter().map(|(_,y)| y).max().unwrap();

    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            if visisted.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    dbg!(visisted.len());
}
