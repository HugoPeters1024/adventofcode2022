use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

use itertools::Itertools;
use scanf::sscanf;

fn main() {
    let mut nodes: HashSet<String> = HashSet::new();
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut lhs = String::new();
        let mut rhs = String::new();
        sscanf!(&line, "{}-{}", lhs, rhs).unwrap();

        nodes.insert(lhs.clone());
        nodes.insert(rhs.clone());
        connections
            .entry(lhs.clone())
            .or_default()
            .insert(rhs.clone());
        connections.entry(rhs).or_default().insert(lhs);
    }


    let mut clusters: HashSet<Vec<String>> = HashSet::new();

    let mut nodes: Vec<String> = nodes.iter().cloned().collect();
    nodes.sort();

    for node in &nodes {
        let mut visisted: HashSet<String> = HashSet::new();
        let mut cluster: HashSet<String> = HashSet::new();
        let mut work = VecDeque::new();
        work.push_front(node.clone());

        while let Some(node) = work.pop_front() {
            if visisted.contains(&node) {
                continue;
            }
            visisted.insert(node.clone());

            if cluster
                .iter()
                .all(|v| connections.get(&node).unwrap().contains(v))
            {
                cluster.insert(node.clone());
            }

            for neighbour in connections.get(&node).unwrap_or(&HashSet::new()) {
                work.push_back(neighbour.clone());
            }
        }

        let mut cluster: Vec<String> = cluster.iter().cloned().collect();
        cluster.sort();
        clusters.insert(cluster.clone());
    }

    let mut sum = 0;
    let mut seen: HashSet<Vec<&String>> = HashSet::new();
    for cluster in &clusters {
        for threecluster in cluster.iter().combinations(3) {
            if seen.contains(&threecluster) {
                continue;
            }
            seen.insert(threecluster.clone());
            if threecluster.iter().any(|name| name.starts_with("t")) {
                sum += 1;
            }
        }
    }


    println!("Part 1: {sum}");
    let z = clusters.iter().max_by_key(|x| x.len()).unwrap().join(",");
    println!("Part 2: {}", z);
}
