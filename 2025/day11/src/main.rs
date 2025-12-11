use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

fn main() {
    let input: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.iter() {
        let (from, tos) = line.split_once(":").unwrap();
        let tos = tos.split_whitespace().collect();
        graph.insert(from, tos);
    }

    println!("Part 1: {}", count_paths_part1(&graph));

    let mut cache = HashMap::new();
    println!(
        "Part 2: {}",
        count_paths_part2_recursive(&graph, &mut cache, "svr", 0)
    );
}

fn count_paths_part1(graph: &HashMap<&str, Vec<&str>>) -> usize {
    let mut work = VecDeque::new();
    work.push_front("you");

    let mut sum = 0;
    while let Some(node) = work.pop_front() {
        if node == "out" {
            sum += 1;
            continue;
        }

        if let Some(children) = graph.get(node) {
            for child in children.iter() {
                work.push_front(child);
            }
        }
    }

    sum
}

fn count_paths_part2_recursive<'a>(
    graph: &'a HashMap<&'a str, Vec<&str>>,
    cache: &mut HashMap<(&'a str, u8), usize>,
    from: &'a str,
    visit_log: u8,
) -> usize {
    if from == "out" && visit_log == 0b11 {
        return 1;
    }
    if let Some(n) = cache.get(&(from, visit_log)) {
        return *n;
    }

    let mut visit_log = visit_log.clone();
    if from == "dac" {
        visit_log |= 0b01;
    };
    if from == "fft" {
        visit_log |= 0b10;
    };
    let mut sum = 0;
    if let Some(children) = graph.get(from) {
        for child in children.iter() {
            sum += count_paths_part2_recursive(graph, cache, child, visit_log);
        }
    }

    cache.insert((from, visit_log), sum);
    println!("from {} it is {} paths", from, sum);
    sum
}
