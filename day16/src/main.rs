use std::{io::BufRead, collections::{HashMap, VecDeque, HashSet, hash_map::DefaultHasher}};
use std::hash::{Hash, Hasher};
use std::collections::BinaryHeap;
use scanf::sscanf;

#[derive(Debug, Clone)]
struct Valve {
    name: u64,
    flow: u32,
    neighbours: Vec<u64>,
}

#[derive(Debug, Clone)]
struct Item {
    time: u32,
    current_flow: u32,
    total: u32,
    current_valve: u64,
    unopened: HashSet<u64>,
}

#[derive(Debug, Clone)]
struct CombinedItem {
    time: u32,
    current_flow: u32,
    total: u32,
    current_valve_me: u64,
    current_valve_elephant: u64,
    unopened: HashSet<u64>,
}

fn main() {
    let mut valves : HashMap<u64, Valve> = HashMap::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut valve: String = "".to_string();
        let mut flow = 0;
        let mut neighbour_str : String = "".to_string();
        if sscanf!(&line, "Valve {} has flow rate={}; tunnels lead to valves {}", valve, flow, neighbour_str).is_err() {
            sscanf!(&line, "Valve {} has flow rate={}; tunnel leads to valve {}", valve, flow, neighbour_str).unwrap();
        }

        let valve_id = calculate_hash(&valve);

        let neighbours : Vec<u64> = neighbour_str.split(", ").map(|s| calculate_hash(&s.to_string())).collect();
        let valve = Valve { name: valve_id, flow, neighbours };
        valves.insert(valve_id, valve);
    }

    let mut distances : HashMap<(u64, u64), u32> = HashMap::new();

    // a loop over the cartesian producct of vales.values()
    for (lhs, rhs) in valves.values().flat_map(|v| valves.values().map(move |v2| (v, v2))) {
        let mut visisted: HashSet<u64> = HashSet::new();
        let mut work : VecDeque<(u64, u32)> = VecDeque::new();
        work.push_back((lhs.name, 0));

        while let Some((current,cost)) = work.pop_front() {
            if visisted.contains(&current) {
                continue;
            }
            visisted.insert(current);

            if current == rhs.name {
                distances.insert((lhs.name, rhs.name), cost);
                break;
            }

            for neighbour in &valves[&current].neighbours {
                work.push_back((*neighbour, cost + 1));
            }
        }
    }

    println!("distances initialized");

    // part 1
    let mut work : VecDeque<Item> = VecDeque::new();

    let mut start = Item {
        time: 0,
        current_flow: 0,
        total: 0,
        unopened: valves.keys().cloned().collect(),
        current_valve: calculate_hash(&"AA".to_string()),
    };

    // never open 0 flow
    for valve in valves.values() {
        if valve.flow == 0 {
            start.unopened.remove(&valve.name);
        }
    }
    work.push_front(start);

    let mut best = 0;
    while let Some(current) = work.pop_front() {
        // always an option to do nothing
        best = best.max(current.total + (30 - current.time) * current.current_flow);

        // open the current valve if it is not open yet
        if current.time < 30 && current.unopened.contains(&current.current_valve) {
            let mut new = current.clone();
            new.unopened.remove(&current.current_valve);
            new.total += new.current_flow;
            new.current_flow += valves[&current.current_valve].flow;
            new.time += 1;
            work.push_back(new);
        } else {
            // go to an unopened valve
            for target in current.unopened.iter() {
                let dis = distances.get(&(current.current_valve, *target)).unwrap();
                if current.time + dis < 30 {
                    let mut new = current.clone();
                    new.time += dis;
                    new.total += dis * new.current_flow;
                    new.current_valve = *target;
                    work.push_back(new);
                }
            }
        }
    }

    dbg!(best);
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
