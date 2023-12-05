use std::{collections::VecDeque, io::BufRead, sync::Mutex};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use scanf::sscanf;

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("seeds: ")(input)?;
    separated_list1(space1, parse_i64)(input)
}

fn find_best_match(map: &Vec<(i64, i64, i64)>, source_id: i64) -> i64 {
    let mut lhs: i64 = 0;
    let mut rhs: i64 = map.len() as i64 - 1;
    while lhs <= rhs {
        let mid = (lhs + rhs) / 2;
        let (dest_start, source_start, range) = &map[mid as usize];
        if source_start + range <= source_id {
            lhs = mid + 1;
        } else if source_start > &source_id {
            rhs = mid - 1;
        } else {
            let delta = source_id - source_start;
            return dest_start + delta;
        }
    }

    source_id
}

fn main() {
    let mut seeds = Vec::new();
    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];

        if i == 0 {
            seeds = parse_seeds(line).unwrap().1;
            i += 1;
            continue;
        }

        if line.is_empty() {
            i += 1;
            continue;
        }

        let mut source = String::new();
        let mut dest = String::new();
        sscanf!(&line, "{}-to-{} map:", source, dest).unwrap();

        let mut entries = Vec::new();
        let mut dest_start = 0;
        let mut source_start = 0;
        let mut range = 0;

        i += 1;
        while let Ok(_) = sscanf!(&lines[i], "{} {} {}", dest_start, source_start, range) {
            entries.push((dest_start, source_start, range));
            i += 1;
            if i >= lines.len() {
                break;
            }
        }

        entries.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));

        maps.push(entries);
        i += 1;
    }

    let mut locations = Vec::new();

    for seed in &seeds {
        let mut source = 0;
        let mut dest = 1;
        let mut source_id = *seed;
        dbg!(source_id);

        loop {
            source_id = find_best_match(&maps[source], source_id);

            source += 1;
            dest += 1;

            if dest == 8 {
                locations.push(source_id);
                break;
            }
        }
    }

    println!("Part 1: {}", locations.iter().min().unwrap());

    let mut divided_seeds: VecDeque<(i64, i64, bool)> =
        VecDeque::from_iter(seeds.chunks(2).map(|v| (v[0], v[1], false)));

    let mil = 1000000;
    loop {
        let (start, range, seen) = divided_seeds.pop_front().unwrap();
        if seen {
            divided_seeds.push_back((start, range, true));
            break;
        }
        if range > mil {
            divided_seeds.push_front((start + mil, range - mil, false));
            divided_seeds.push_back((start, mil, true));
        } else {
            divided_seeds.push_back((start, range, true));
        }
    }

    println!("Work items: {}", divided_seeds.len());

    // shared mutex to collect the minimum result
    let min_location: Mutex<i64> = Mutex::new(999999999999999999);
    divided_seeds.par_iter().for_each(|(start_seed, range, _)| {
        // We use a local minimum as an upper bounded on the shared minimum
        // to prevent locking the mutex when not needed
        let mut local_min: i64 = 999999999999999999;
        for seed in *start_seed..=*start_seed + range {
            let mut source = 0;
            let mut source_id = seed;

            loop {
                source_id = find_best_match(&maps[source], source_id);

                source += 1;
                if source == 7 {
                    if source_id < local_min {
                        let mut r = min_location.lock().unwrap();
                        *r = std::cmp::min(source_id, *r);
                        // update the local min while we have to mutex to
                        // prevent locking in the future.
                        local_min = std::cmp::min(source_id, *r);
                    }
                    break;
                };
            }
        }
    });

    println!("Part 2: {}", min_location.lock().unwrap());
}
