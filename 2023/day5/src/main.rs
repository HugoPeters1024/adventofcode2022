use std::{collections::HashMap, io::BufRead, hash::{DefaultHasher, Hash, Hasher}};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use scanf::sscanf;

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("seeds: ")(input)?;
    separated_list1(space1, parse_i64)(input)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let mut seeds = Vec::new();
    let mut dest_lookup: HashMap<u64, u64> = HashMap::new();
    let mut map_lookup: HashMap<(u64, u64), Vec<(i64, i64, i64)>> = HashMap::new();

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

        dest_lookup.insert(calculate_hash(&source), calculate_hash(&dest));

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

        map_lookup.insert((calculate_hash(&source), calculate_hash(&dest)), entries);
        i += 1;
    }

    let mut locations = Vec::new();

    for seed in &seeds {
        let mut source = calculate_hash(&"seed");
        let mut dest = calculate_hash(&"soil");
        let mut source_id = *seed;

        loop {
            let entries = &map_lookup[&(source, dest)];
            if let Some(((dest_start, _, _), delta)) = entries
                .iter()
                .map(|x| (x, source_id - x.1))
                .filter(|x| x.1 >= 0 && x.1 <= x.0 .2)
                .min_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
            {
                source_id = dest_start + delta;
            }

            source = dest.clone();
            dest = if let Some(ndest) = dest_lookup.get(&dest) {
                ndest.clone()
            } else {
                locations.push(source_id);
                break;
            };
        }
    }

    println!("Part 1: {}", locations.iter().min().unwrap());

    let mut min_location = 999999999999999999;
    for seed_and_range in seeds.chunks(2) {
        let start_seed = seed_and_range[0];
        for seed in start_seed..=start_seed + seed_and_range[1] {
            let mut source = calculate_hash(&"seed");
            let mut dest = calculate_hash(&"soil");
            let mut source_id = seed;

            loop {
                let entries = &map_lookup[&(source, dest)];

                if let Some(((dest_start, _, _), delta)) = entries
                    .iter()
                    .map(|x| (x, source_id - x.1))
                    .filter(|x| x.1 >= 0 && x.1 <= x.0 .2)
                    .min_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
                {
                    source_id = dest_start + delta;
                }

                source = dest;
                dest = if let Some(ndest) = dest_lookup.get(&dest) {
                    *ndest
                } else {
                    min_location = std::cmp::min(source_id, min_location);
                    break;
                };
            }
        }
    }

    println!("Part 2: {}", min_location);
}
