use std::{collections::HashMap, io::BufRead};

use scanf::sscanf;

fn main() {
    let mut steps: Vec<char> = Vec::new();
    let mut lookup: HashMap<String, (String, String)> = HashMap::new();

    for (i, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            steps = line.chars().collect();
            continue;
        }

        if i == 1 {
            continue;
        }

        let mut k = "".to_string();
        let mut lhs = "".to_string();
        let mut rhs = "".to_string();
        sscanf!(&line, "{} = ({}, {})", k, lhs, rhs).unwrap();
        lookup.insert(k, (lhs, rhs));
    }

    let mut pos = "AAA";
    if lookup.contains_key(pos) {
        for (i, step) in steps.iter().cycle().enumerate() {
            if pos == "ZZZ" {
                println!("Part 1: {}", i);
                break;
            }

            let (lhs, rhs) = &lookup[pos];
            if *step == 'L' {
                pos = lhs;
            } else {
                pos = rhs;
            }
        }
    }

    let mut multi_pos: Vec<&String> = lookup.keys().filter(|x| x.ends_with("A")).collect();
    let mut lcm: usize = 1;
    for (i, step) in steps.iter().cycle().enumerate() {
        if multi_pos.len() == 0 {
            break;
        }

        multi_pos = multi_pos
            .iter()
            .filter_map(|pos| {
                if pos.ends_with("Z") {
                    lcm = num::integer::lcm(i, lcm);
                    None
                } else {
                    let (lhs, rhs) = &lookup[*pos];
                    if *step == 'L' {
                        Some(lhs)
                    } else {
                        Some(rhs)
                    }
                }
            })
            .collect();
    }

    println!("Part 2: {}", lcm);
}
