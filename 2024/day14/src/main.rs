use std::io::BufRead;

use scanf::sscanf;

fn part1(robots: &Vec<(isize, isize, isize, isize)>, steps: isize) -> [isize;4] {
    let width = 101;
    let height = 103;

    let mut quadrants = [0; 4];

    for (px, py, vx, vy) in robots.iter().cloned() {
        let nx = (px + steps * vx).rem_euclid(width);
        let ny = (py + steps * vy).rem_euclid(height);

        if nx < width / 2 {
            if ny < height / 2 {
                quadrants[0] += 1;
            } else if ny > height / 2 {
                quadrants[1] += 1;
            }
        } else if nx > width / 2 {
            if ny < height / 2 {
                quadrants[2] += 1;
            } else if ny > height / 2 {
                quadrants[3] += 1;
            }
        }
    }

    quadrants
}

fn part2(robots: &Vec<(isize, isize, isize, isize)>, steps: isize) -> isize {
    let width = 101;
    let height = 103;

    let mut heuristic = 0;

    for (px, py, vx, vy) in robots.iter().cloned() {
        let nx = (px + steps * vx).rem_euclid(width);
        let ny = (py + steps * vy).rem_euclid(height);
        heuristic += (width/2 - nx).abs();
        heuristic += (height/2 - ny).abs();
    }

    heuristic
}

fn main() {
    let robots: Vec<(isize, isize, isize, isize)> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut px = 0;
            let mut py = 0;
            let mut vx = 0;
            let mut vy = 0;
            sscanf!(&l, "p={},{} v={},{}", px, py, vx, vy).unwrap();
            (px, py, vx, vy)
        })
        .collect();

    let answer = part1(&robots, 100);
    let answer = answer[0] * answer[1] * answer[2] * answer[3];

    println!("Part 1: {answer}");

    let mut min_heuristic = 10000000000isize;
    let mut min_idx = 0;
    for steps in 0..10403 {
        let heuristic = part2(&robots, steps);
        if heuristic < min_heuristic {
            min_heuristic = heuristic;
            min_idx = steps;
        }
    }

    dbg!(min_idx);
}
