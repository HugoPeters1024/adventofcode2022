use itertools::Itertools;
use num::integer::*;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn main() {
    let mut width: isize = 0;
    let mut height: isize = 0;
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        width = line.len() as isize;
        height += 1;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for locs in antennas.values() {
        for locpair in locs.iter().combinations(2) {
            let (x1, y1) = locpair[0];
            let (x2, y2) = locpair[1];
            let line = ((x2 - x1), (y2 - y1));
            antinodes.insert((x1 - line.0, y1 - line.1));
            antinodes.insert((x2 + line.0, y2 + line.1));
        }
    }

    let total = antinodes
        .iter()
        .filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
        .count();

    println!("Part 1: {total}");

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for locs in antennas.values() {
        for locpair in locs.iter().combinations(2) {
            let (x1, y1) = locpair[0];
            let (x2, y2) = locpair[1];
            let line = ((x2 - x1), (y2 - y1));
            let gcd = isize::extended_gcd(&line.0, &line.1).gcd;
            let line = (line.0 / gcd, line.1 / gcd);

            let mut x = *x1;
            let mut y = *y1;
            while x >= 0 && x < width && y >= 0 && y < height {
                antinodes.insert((x, y));
                x -= line.0;
                y -= line.1;
            }

            let mut x = *x1;
            let mut y = *y1;
            while x >= 0 && x < width && y >= 0 && y < height {
                antinodes.insert((x, y));
                x += line.0;
                y += line.1;
            }

            antinodes.insert((x1 - line.0, y1 - line.1));
            antinodes.insert((x2 + line.0, y2 + line.1));
        }
    }

    let total = antinodes
        .iter()
        .filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
        .count();

    println!("Part 2: {total}");
}
