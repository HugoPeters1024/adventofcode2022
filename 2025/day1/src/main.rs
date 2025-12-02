use std::io::BufRead;

use scanf::sscanf;

enum Rot {
    L(isize),
    R(isize),
}

fn main() {
    let rotations: Vec<Rot> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .filter_map(|line| {
            let mut n: isize = 0;
            if sscanf!(line.as_str(), "L{}", &mut n).is_ok() {
                Some(Rot::L(n))
            } else if sscanf!(line.as_str(), "R{}", &mut n).is_ok() {
                Some(Rot::R(n))
            } else {
                None
            }
        })
        .collect();

    let mut current_value: isize = 50;
    let mut zero_count = 0;
    for rot in &rotations {
        match rot {
            Rot::L(l) => {
                current_value = (current_value - l).rem_euclid(100);
            }
            Rot::R(r) => {
                current_value = (current_value + r).rem_euclid(100);
            }
        };

        if current_value == 0 {
            zero_count += 1;
        }
    }

    println!("Part 1: {zero_count}");

    let mut current_value: isize = 50;
    let mut zero_count = 0;
    for rot in &rotations {
        let n = match rot {
            Rot::L(l) => l,
            Rot::R(r) => r,
        };

        for _ in 0..*n {
            current_value = (current_value
                + match rot {
                    Rot::L(_) => -1,
                    Rot::R(_) => 1,
                })
            .rem_euclid(100);
            if current_value == 0 {
                zero_count += 1;
            }
        }
    }

    println!("Part 2: {zero_count}");
}
