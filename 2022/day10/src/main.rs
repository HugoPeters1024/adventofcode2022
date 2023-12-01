use std::{io::BufRead, collections::HashSet};

use scanf::sscanf;


fn main() {
    let mut x = 1;
    let mut history : Vec<i32> = Vec::new();
    let mut pixels : Vec<bool> = vec![false; 40*6];
    let mut pixel_pos = 0;

    history.push(x);
    for line in std::io::stdin().lock().lines().map(|x| x.unwrap()) {
        if line == "noop" {
            if ((pixel_pos % 40) - x).abs() <= 1 {
                pixels[pixel_pos as usize] = true;
            }
            pixel_pos += 1;
            history.push(x);
        } else {
            let mut val : i32 = 0;
            sscanf!(&line, "addx {}", val).unwrap();
            if ((pixel_pos % 40) - x).abs() <= 1 {
                pixels[pixel_pos as usize] = true;
            }
            pixel_pos += 1;
            history.push(x);

            if ((pixel_pos % 40) - x).abs() <= 1 {
                pixels[pixel_pos as usize] = true;
            }
            pixel_pos += 1;
            x += val;
            history.push(x);
        }

    }
    
    let mut total = 0;
    for i in 0..6 {
        let idx = 20 + 40 * i - 1;
        println!("[{}] = {}", idx, history[idx]);
        total += history[idx] * (idx as i32 + 1);
    }


    for y in 0..6 {
        for x in 0..40 {
            let idx = x + 40 * y;
            if pixels[idx] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    dbg!(total);
}
