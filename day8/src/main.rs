use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

use itertools::Itertools;

#[derive(Debug)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

fn dist2(lhs: &Vec3, rhs: &Vec3) -> isize {
    (lhs.x - rhs.x).pow(2) + (lhs.y - rhs.y).pow(2) + (lhs.z - rhs.z).pow(2)
}

fn main() {
    let nodes: Vec<Vec3> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| {
            let xyz: Vec<isize> = x.unwrap().split(",").map(|x| x.parse().unwrap()).collect();
            Vec3 {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
            }
        })
        .collect();

    let mut dists: Vec<(usize, usize, isize)> = nodes
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pair| (pair[0].0, pair[1].0, dist2(pair[0].1, pair[1].1)))
        .collect();

    dists.sort_by_key(|d| d.2);
    let dists = dists;

    let mut conns: HashMap<usize, Vec<usize>> = HashMap::new();

    for (lhs, rhs, _) in dists.iter().take(1000) {
        if path_exists(&conns, *lhs, *rhs) {
            continue;
        }

        conns.entry(*lhs).or_default().push(*rhs);
        conns.entry(*rhs).or_default().push(*lhs);
    }

    let mut clusters = Vec::new();
    while let Some(n) = pop_cluster(&mut conns) {
        clusters.push(n);
    }

    clusters.sort();
    clusters.reverse();

    let product = clusters.iter().take(3).fold(1, |x, y| x * y);

    println!("Part 1: {product}");

    let mut conns: HashMap<usize, Vec<usize>> = HashMap::new();

    for (lhs, rhs, _) in dists.iter() {
        conns.entry(*lhs).or_default().push(*rhs);
        conns.entry(*rhs).or_default().push(*lhs);

        if count_cluster(&conns) == nodes.len() {
            let magic = nodes[*lhs].x * nodes[*rhs].x;
            println!("Part 2: {magic}");
            break;
        }
    }
}

fn path_exists(graph: &HashMap<usize, Vec<usize>>, from: usize, to: usize) -> bool {
    let mut work = VecDeque::new();
    let mut seen: HashSet<usize> = HashSet::new();
    work.push_front(from);

    while let Some(res) = work.pop_front() {
        if res == to {
            return true;
        }
        if seen.contains(&res) {
            continue;
        }

        seen.insert(res);

        if let Some(children) = graph.get(&res) {
            for child in children.iter() {
                work.push_back(*child);
            }
        }
    }

    false
}

fn count_cluster(graph: &HashMap<usize, Vec<usize>>) -> usize {
    let Some((start, _)) = graph.iter().next() else {
        return 0;
    };

    let mut seen: HashSet<usize> = HashSet::new();
    let mut work = VecDeque::new();
    work.push_back(start);

    while let Some(res) = work.pop_front() {
        if seen.contains(res) {
            continue;
        }

        seen.insert(*res);

        if let Some(children) = graph.get(&res) {
            for child in children.iter() {
                work.push_back(child);
            }
        }
    }

    seen.len()
}

fn pop_cluster(graph: &mut HashMap<usize, Vec<usize>>) -> Option<usize> {
    let Some((start, _)) = graph.iter().next() else {
        return None;
    };

    let mut seen: HashSet<usize> = HashSet::new();
    let mut work = VecDeque::new();
    work.push_back(start);

    while let Some(res) = work.pop_front() {
        if seen.contains(res) {
            continue;
        }

        seen.insert(*res);

        if let Some(children) = graph.get(&res) {
            for child in children.iter() {
                work.push_back(child);
            }
        }
    }

    for seen in seen.iter() {
        graph.remove(seen);
    }

    Some(seen.len())
}
