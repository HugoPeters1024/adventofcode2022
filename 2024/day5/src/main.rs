use scanf::sscanf;
use std::{collections::HashSet, io::BufRead};
use topo_sort::TopoSort;

fn insert_or_update(ts: &mut TopoSort<usize>, k: usize, v: usize) {
    if let Some(deps) = ts.get(&k) {
        let mut new_deps = deps.clone();
        new_deps.insert(v);
        ts.insert(k, new_deps);
    } else {
        ts.insert(k, vec![v]);
    }
}

fn main() {
    let mut rules: Vec<(usize, usize)> = Vec::new();

    let mut updates: Vec<Vec<usize>> = Vec::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();

        let mut rule_lhs = 0;
        let mut rule_rhs = 0;

        if sscanf!(&line, "{}|{}", rule_lhs, rule_rhs).is_ok() {
            rules.push((rule_lhs, rule_rhs));
            continue;
        }

        if line == "" {
            continue;
        }

        updates.push(
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        );
    }

    let mut sum = 0;
    let mut wrongs: Vec<Vec<usize>> = Vec::new();

    for update in updates.iter() {
        let mut ts = TopoSort::<usize>::new();
        let numbers_in_update: HashSet<usize> = update.iter().cloned().collect();

        for rule in &rules {
            if numbers_in_update.contains(&rule.0) && numbers_in_update.contains(&rule.1) {
                insert_or_update(&mut ts, rule.0, rule.1);
            }
        }

        for i in 0..update.len() - 1 {
            insert_or_update(&mut ts, update[i], update[i + 1]);
        }

        if !ts.cycle_detected() {
            sum += update[update.len() / 2];
        } else {
            wrongs.push(update.clone());
        }
    }

    println!("part 1: {sum}");

    let mut wrong_sum = 0;
    for mut wrong in wrongs {
        'outer: loop {
            for i in 0..wrong.len() - 1 {
                for (k, v) in &rules {
                    if *k == wrong[i + 1] && *v == wrong[i] {
                        let tmp = wrong[i];
                        wrong[i] = wrong[i + 1];
                        wrong[i + 1] = tmp;
                        continue 'outer;
                    }
                }
            }

            break;
        }

        wrong_sum += wrong[wrong.len() / 2];
    }

    println!("part 2: {wrong_sum}");
}
