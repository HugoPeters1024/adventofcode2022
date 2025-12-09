use bitvec::{access::BitSafe, prelude::*};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::{
    io::BufRead,
    sync::{
        Mutex,
        atomic::{AtomicIsize, AtomicUsize, Ordering},
    },
};

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

fn main() {
    let markers: Vec<Vec2> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| {
            let v = x
                .unwrap()
                .split(",")
                .map(|el| el.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            Vec2 { x: v[0], y: v[1] }
        })
        .collect();

    let min_x = markers.iter().map(|v| v.x).min().unwrap();
    let min_y = markers.iter().map(|v| v.y).min().unwrap();
    let markers: Vec<Vec2> = markers
        .iter()
        .map(|v| Vec2 {
            x: v.x - min_x,
            y: v.y - min_y,
        })
        .collect();

    let mut max = 0;
    for pair in markers.iter().combinations(2) {
        let surface = ((pair[0].x - pair[1].x).abs() + 1) * ((pair[0].y - pair[1].y).abs() + 1);
        if surface > max {
            max = surface;
        }
    }

    println!("Part 1: {max}");

    let width = 1 + markers.iter().map(|v| v.x).max().unwrap() as usize;
    let height = 1 + markers.iter().map(|v| v.y).max().unwrap() as usize;

    let mut grid = bitvec![0b0; width*height];
    for (i, lhs) in markers.iter().enumerate() {
        let rhs = &markers[(i + 1) % markers.len()];

        let dx = (rhs.x - lhs.x).signum();
        let dy = (rhs.y - lhs.y).signum();

        let mut x = lhs.x;
        let mut y = lhs.y;

        while !(x == rhs.x && y == rhs.y) {
            grid.set(y as usize * width + x as usize, true);
            x += dx;
            y += dy;
        }

        grid.set(y as usize * width + x as usize, true);
    }
    println!("done with the outer edges, filled {}", grid.count_ones());

    print_grid(&grid, width, height, 200, 40);

    let grid = Mutex::new(grid);
    let y_filled = AtomicUsize::new(0);
    (0..height).into_par_iter().for_each(|y| {
        let mut bitline = bitvec![0b0; width];
        let progress = y_filled.fetch_add(1, Ordering::Relaxed);
        if progress % 1000 == 0 {
            println!("filling {}%", progress as f32 / height as f32 * 100.0);
        }
        for x in 0..width {
            let mut crossings = 0;
            let px = x as isize;
            let py = y as isize;

            for (i, lhs) in markers.iter().enumerate() {
                let rhs = &markers[(i + 1) % markers.len()];

                let x1 = lhs.x;
                let y1 = lhs.y;
                let x2 = rhs.x;
                let y2 = rhs.y;

                let y_min = y1.min(y2);
                let y_max = y1.max(y2);

                if y1 == y2 {
                    continue;
                }

                if py >= y_min && py < y_max {
                    let x_intersect = if y2 != y1 {
                        x1 + ((py - y1) * (x2 - x1)) / (y2 - y1)
                    } else {
                        continue;
                    };

                    if x_intersect >= px {
                        crossings += 1;
                    }
                }
            }

            // Point is inside if odd number of crossings
            if crossings % 2 == 1 {
                bitline.set(x, true);
            }
        }

        let mut grid = grid.lock().unwrap();
        for x in 0..width {
            if bitline.get(x).unwrap() == true {
                grid.set(y * width + x, true);
            }
        }
    });

    let grid = grid.lock().unwrap();
    println!("done filling, filled {}", grid.count_ones());
    print_grid(&grid, width, height, 200, 40);

    if grid.get(0).unwrap() == true {
        println!("doubt!!!");
        //return;
    } else {
        println!("no doubt");
    }

    let max = AtomicIsize::new(0);
    let count = AtomicUsize::new(0);
    markers
        .clone()
        .into_iter()
        .combinations(2)
        .par_bridge()
        .for_each(|pair| {
            let idx = count.fetch_add(1, Ordering::Relaxed);
            if idx % 1000 == 0 {
                println!(
                    "Progress: {}% (best: {})",
                    (idx as f32 / ((markers.len() as f32 * (markers.len() as f32 - 1.0)) / 2.0))
                        * 100.0,
                    max.load(Ordering::Relaxed)
                )
            }

            let surface = ((pair[0].x - pair[1].x).abs() + 1) * ((pair[0].y - pair[1].y).abs() + 1);

            if surface < max.load(Ordering::Relaxed) {
                return;
            }

            let sx = pair[0].x.min(pair[1].x);
            let mx = pair[0].x.max(pair[1].x);

            let sy = pair[0].y.min(pair[1].y);
            let my = pair[0].y.max(pair[1].y);

            for y in sy..=my {
                for x in sx..=mx {
                    if !grid.get(y as usize * width + x as usize).unwrap() {
                        return;
                    }
                }
            }

            let new_best = max.fetch_max(surface, Ordering::Relaxed);
            println!("new best: {new_best}");
        });

    // too low: 128303539
    // too low: 128306297
    // wrong:   1469755782
    // wrong:   364004166
    println!("Part 2: {}", max.load(Ordering::Relaxed));
}

fn print_grid(grid: &BitVec, width: usize, height: usize, mx: usize, my: usize) {
    for y in 0..my.min(height) {
        for x in 0..mx.min(width) {
            print!(
                "{}",
                if grid.get(y * width + x).unwrap() == true {
                    'X'
                } else {
                    '.'
                }
            );
        }
        println!("");
    }
}
