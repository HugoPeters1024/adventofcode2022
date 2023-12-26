use std::{io::BufRead, collections::{HashMap, VecDeque, HashSet}};

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use scanf::sscanf;

fn main() {
    let mut name_to_idx: HashMap<String, usize> = HashMap::new();
    let mut idx_to_name: IndexMap<usize, String> = IndexMap::new();
    let mut max_idx = 0;

    let mut register = |name: &String| {
        if let Some(idx) = name_to_idx.get(name) {
            return *idx;
        }

        let idx = max_idx;
        name_to_idx.insert(name.clone(), idx);
        idx_to_name.insert(idx, name.clone());
        max_idx += 1;
        return idx;
    };

    let mut edges: Vec<Vec<usize>> = vec![vec![]; 1000000];


    for line in std::io::stdin().lock().lines() {
        let mut lhs = "".to_string();
        let mut rhs = "".to_string();
        if let Ok(_) = sscanf!(&line.unwrap(), "{}: {}", lhs, rhs) {
            let lhs_idx = register(&lhs);
            for rhs in rhs.split_whitespace() {
                let rhs_idx = register(&rhs.to_string());
                edges[lhs_idx].push(rhs_idx);
                edges[rhs_idx].push(lhs_idx);
            }
        }
    }

    drop(register);

    //let clb = name_to_idx.get(&"clb".to_string()).unwrap();
    //let brd = name_to_idx.get(&"brd".to_string()).unwrap();

    //edges[*clb].retain(|x| *x != *brd);
    //edges[*brd].retain(|x| *x != *clb);

    //let bbz = name_to_idx.get(&"bbz".to_string()).unwrap();
    //let jxd = name_to_idx.get(&"jxd".to_string()).unwrap();

    //edges[*bbz].retain(|x| *x != *jxd);
    //edges[*jxd].retain(|x| *x != *bbz);

    //let glz = name_to_idx.get(&"glz".to_string()).unwrap();
    //let mxd = name_to_idx.get(&"mxd".to_string()).unwrap();

    //edges[*glz].retain(|x| *x != *mxd);
    //edges[*mxd].retain(|x| *x != *glz);


    for _ in 0..3 {
        let mut visit_count: HashMap<(usize, usize), usize> = HashMap::new();

        for i in 0..10000 {
            // pick two random nodes
            let source = rand::random::<usize>() % max_idx;
            let sink = rand::random::<usize>() % max_idx;

            if source == sink {
                continue;
            }

            let mut work: VecDeque<(usize, Vec<usize>)> = VecDeque::new();
            let mut visited: HashSet<usize> = HashSet::new();
            work.push_back((source.clone(), Vec::new()));

            let mut found = false;
            while let Some((current, path)) = work.pop_front() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current.clone());

                if current == sink {
                    found = true;
                    println!("#{}: {} -> {} in {} steps", i, source, sink, visited.len());
                    for (lhs, rhs) in path.iter().tuple_windows() {
                        *visit_count.entry((lhs.clone(), rhs.clone())).or_insert(0) += 1;
                    }
                    break;
                }

                for next in &edges[current] {
                    let mut npath = path.clone();
                    npath.push(current);
                    work.push_back((next.clone(), npath.clone()));
                }
            }

            if !found {
                panic!("no path found from {} to {} (visited: {})", source, sink, visit_count.len());
            }
        }

        let mut visits: Vec<((usize, usize), usize)> = visit_count.iter().map(|(k, v)| (k.clone(), *v)).collect();
        visits.sort_by(|a, b| b.1.cmp(&a.1));

        let ((lhs,rhs), count) = visits[0];
        println!("{} -> {} visited {} times, removing", idx_to_name.get(&lhs).unwrap(), idx_to_name.get(&rhs).unwrap(), count);

        edges[lhs].retain(|x| *x != rhs);
        edges[rhs].retain(|x| *x != lhs);
    }


    let mut clusters: HashSet<usize> = HashSet::new();
    while clusters.len() < 2 {
        // pick a random node
        // and find all the nodes connected to it
        let start = rand::random::<usize>() % max_idx;

        let mut visisted: HashSet<usize> = HashSet::new();
        let mut work: VecDeque<usize> = VecDeque::new();
        work.push_back(start);

        while let Some(node) = work.pop_front() {
            if visisted.contains(&node) {
                continue;
            }
            visisted.insert(node);

            for next in &edges[node] {
                work.push_front(*next);
            }
        }

        clusters.insert(visisted.len());
    }

    println!("Part 1: {}", clusters.iter().product::<usize>());
}
