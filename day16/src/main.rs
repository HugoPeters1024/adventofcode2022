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
struct TravelState {
    valve: u64,
    time_left: u32,
}

impl TravelState {
    fn arrived(&self) -> bool { self.time_left == 0 }
}

#[derive(Debug, Clone)]
struct CombinedItem {
    time: u32,
    current_flow: u32,
    total: u32,
    current_valve_me: TravelState,
    current_valve_elephant: TravelState,
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
    //let mut work : VecDeque<Item> = VecDeque::new();

    //let mut start = Item {
    //    time: 0,
    //    current_flow: 0,
    //    total: 0,
    //    unopened: valves.keys().cloned().collect(),
    //    current_valve: calculate_hash(&"AA".to_string()),
    //};

    //// never open 0 flow
    //for valve in valves.values() {
    //    if valve.flow == 0 {
    //        start.unopened.remove(&valve.name);
    //    }
    //}
    //work.push_front(start);

    //let mut best = 0;
    //while let Some(current) = work.pop_front() {
    //    // always an option to do nothing
    //    best = best.max(current.total + (30 - current.time) * current.current_flow);

    //    // open the current valve if it is not open yet
    //    if current.time < 30 && current.unopened.contains(&current.current_valve) {
    //        let mut new = current.clone();
    //        new.unopened.remove(&current.current_valve);
    //        new.total += new.current_flow;
    //        new.current_flow += valves[&current.current_valve].flow;
    //        new.time += 1;
    //        work.push_back(new);
    //    } else {
    //        // go to an unopened valve
    //        for target in current.unopened.iter() {
    //            let dis = distances.get(&(current.current_valve, *target)).unwrap();
    //            if current.time + dis < 30 {
    //                let mut new = current.clone();
    //                new.time += dis;
    //                new.total += dis * new.current_flow;
    //                new.current_valve = *target;
    //                work.push_back(new);
    //            }
    //        }
    //    }
    //}

    //dbg!(best);

    // part 2
    let mut work : VecDeque<CombinedItem> = VecDeque::new();

    let mut start = CombinedItem {
        time: 0,
        current_flow: 0,
        total: 0,
        current_valve_me: TravelState { valve: calculate_hash(&"AA".to_string()), time_left: 0 },
        current_valve_elephant: TravelState { valve: calculate_hash(&"AA".to_string()), time_left: 0 },
        unopened: valves.keys().cloned().collect(),
    };

    // never open 0 flow
    for valve in valves.values() {
        if valve.flow == 0 {
            start.unopened.remove(&valve.name);
        }
    }
    work.push_front(start);

    let mut best = 0;
    while let Some(state) = work.pop_back() {
        // always an option to do nothing
        let score = state.total + (26 - state.time) * state.current_flow;
        if score > best {
            best = score;
            // if we let the program run for a few minutes the last printed score
            // is our best guess, it was enough apparently
            // I do feel shame
            dbg!(score);
        }

        // both are at a valve
        if state.current_valve_me.arrived() && state.current_valve_elephant.arrived() {
            // item with pending increase
            let mut my_options : Vec<(CombinedItem, u32)> = Vec::new();

            if state.time < 26 && state.current_valve_me.arrived() && state.unopened.contains(&state.current_valve_me.valve) {
                // open the valve
                let mut new = state.clone();
                new.unopened.remove(&state.current_valve_me.valve);
                my_options.push((new, valves[&state.current_valve_me.valve].flow));
            } else {
                // pick a new valve
                for target in state.unopened.iter() {
                    let dis = distances.get(&(state.current_valve_me.valve, *target)).unwrap();
                    if state.time + dis < 26 {
                        let mut new = state.clone();
                        new.current_valve_me.valve = *target;
                        new.current_valve_me.time_left = *dis;
                        my_options.push((new, 0));
                    }
                }
            }

            let mut combined_options : Vec<(CombinedItem, u32)> = Vec::new();

            // combine all options with the elephant
            for (state, pending) in my_options {
                if state.time < 26 && state.current_valve_elephant.arrived() && state.unopened.contains(&state.current_valve_elephant.valve) {
                    // open the valve
                    let mut new = state.clone();
                    new.unopened.remove(&state.current_valve_elephant.valve);
                    combined_options.push((new, pending+valves[&state.current_valve_elephant.valve].flow));
                } else {
                    // pick a new valve
                    for target in state.unopened.iter() {
                        let dis = distances.get(&(state.current_valve_elephant.valve, *target)).unwrap();
                        if state.time + dis < 26 {
                            let mut new = state.clone();
                            new.current_valve_elephant.valve = *target;
                            new.current_valve_elephant.time_left = *dis;
                            combined_options.push((new, pending));
                        }
                    }
                }
            }

            for (mut option, pending) in combined_options {
                option.time += 1;
                option.total += option.current_flow;
                option.current_flow += pending;
                if option.current_valve_me.time_left > 0 {
                    option.current_valve_me.time_left -= 1;
                }
                if option.current_valve_elephant.time_left > 0 {
                    option.current_valve_elephant.time_left -= 1;
                }

                work.push_back(option)
            }
        } else if state.current_valve_me.arrived() {
            if state.time < 26 && state.current_valve_me.arrived() && state.unopened.contains(&state.current_valve_me.valve) {
                // open the valve
                let mut new = state.clone();
                new.unopened.remove(&state.current_valve_me.valve);
                new.time += 1;
                new.total += new.current_flow;
                new.current_flow += valves[&state.current_valve_me.valve].flow;
                if new.current_valve_elephant.time_left > 0 {
                    new.current_valve_elephant.time_left -= 1;
                }
                work.push_back(new);
            } else {
                // pick a new valve
                for target in state.unopened.iter() {
                    let dis = distances.get(&(state.current_valve_me.valve, *target)).unwrap();
                    if state.time + dis < 26 {
                        let mut new = state.clone();
                        new.current_valve_me.valve = *target;
                        new.current_valve_me.time_left = *dis;
                        work.push_back(new);
                    }
                }
            }
        } else if state.current_valve_elephant.arrived() {
            if state.time < 26 && state.current_valve_elephant.arrived() && state.unopened.contains(&state.current_valve_elephant.valve) {
                // open the valve
                let mut new = state.clone();
                new.unopened.remove(&state.current_valve_elephant.valve);
                new.time += 1;
                new.total += new.current_flow;
                new.current_flow += valves[&state.current_valve_elephant.valve].flow;
                if new.current_valve_me.time_left > 0 {
                    new.current_valve_me.time_left -= 1;
                }
                work.push_back(new);
            } else {
                // pick a new valve
                for target in state.unopened.iter() {
                    let dis = distances.get(&(state.current_valve_elephant.valve, *target)).unwrap();
                    if state.time + dis < 26 {
                        let mut new = state.clone();
                        new.current_valve_elephant.valve = *target;
                        new.current_valve_elephant.time_left = *dis;
                        work.push_back(new);
                    }
                }
            }
        } else {
            // jump forward in time till either one arrives
            let warp_length = state.current_valve_me.time_left.min(state.current_valve_elephant.time_left);
            let mut new = state.clone();
            new.time += warp_length;
            new.total += warp_length * new.current_flow;
            new.current_valve_me.time_left -= warp_length;
            new.current_valve_elephant.time_left -= warp_length;
            work.push_back(new);
        }
    }

    dbg!(best);
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
