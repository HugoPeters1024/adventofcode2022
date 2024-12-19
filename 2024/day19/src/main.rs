use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

fn is_possible(
    pattern: &Vec<char>,
    towels: &Vec<Vec<char>>,
    def_no: &mut HashSet<Vec<char>>,
    prefix: Vec<char>,
) -> bool {
    //println!("intermediate: {}", prefix.iter().collect::<String>());
    //println!("{} are def impossible", def_no.len());

    if &prefix == pattern {
        return true;
    }

    if def_no.contains(&prefix) {
        return false;
    }

    let mut possible = false;
    for towel in towels {
        if prefix.len() + towel.len() > pattern.len() {
            continue;
        }
        let interested_in = &pattern[prefix.len()..prefix.len() + towel.len()];
        if towel == interested_in {
            let mut new_prefix = prefix.clone();
            new_prefix.extend(towel);

            if is_possible(pattern, towels, def_no, new_prefix) {
                possible = true;
                break;
            }
        }
    }

    if !possible {
        //println!("imposible: {}", prefix.iter().collect::<String>());
        def_no.insert(prefix);
    }

    possible
}

fn count_possibilities(
    pattern: &Vec<char>,
    towels: &Vec<Vec<char>>,
    suffix_cache: &mut HashMap<Vec<char>, usize>,
    prefix: Vec<char>,
) -> usize {
    if &prefix == pattern {
        return 1;
    }

    let suffix = &pattern[prefix.len()..].to_vec();
    if let Some(result) = suffix_cache.get(suffix) {
        return *result;
    }

    let mut ways = 0;
    for towel in towels {
        if prefix.len() + towel.len() > pattern.len() {
            continue;
        }
        let interested_in = &pattern[prefix.len()..prefix.len() + towel.len()];
        if towel == interested_in {
            let mut new_prefix = prefix.clone();
            new_prefix.extend(towel);

            ways += count_possibilities(pattern, towels, suffix_cache, new_prefix);
        }
    }

    suffix_cache.insert(suffix.clone(), ways);
    ways
}

fn main() {
    let mut lines = std::io::stdin().lock().lines();

    let towels: Vec<Vec<char>> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| {
            x.chars()
                .take_while(|c| ['w', 'u', 'b', 'r', 'g'].contains(c))
                .collect()
        })
        .collect();

    lines.next();

    let patterns = lines.map(|x| x.unwrap()).collect::<Vec<_>>();

    let mut possible = 0;
    for pattern in &patterns {
        let pattern: Vec<char> = pattern.chars().collect();
        let mut known_impossible_states: HashSet<Vec<char>> = HashSet::new();
        if is_possible(&pattern, &towels, &mut known_impossible_states, Vec::new()) {
            possible += 1;
        }
    }

    println!("Part 1: {possible}");

    let mut ways = 0;
    for pattern in &patterns {
        let pattern: Vec<char> = pattern.chars().collect();
        let mut suffix_cache: HashMap<Vec<char>, usize> = HashMap::new();
        ways += count_possibilities(&pattern, &towels, &mut suffix_cache, Vec::new());
    }

    println!("Part 2: {ways}");
}
