use std::hash::{Hash, Hasher};
use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    io::BufRead,
};

use scanf::sscanf;

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(Pulse),
    Conjunction(HashMap<u64, Pulse>),
    Broadcast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn flip(&mut self) {
        match self {
            Pulse::High => *self = Pulse::Low,
            Pulse::Low => *self = Pulse::High,
        }
    }
}

fn main() {
    let input: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut modules: HashMap<u64, ModuleType> = HashMap::new();
    let mut incoming_edges: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut outgoing_edges: HashMap<u64, Vec<u64>> = HashMap::new();

    let mut name = String::new();
    let mut rest = String::new();
    // First we discover all the nodes
    for line in &input {
        if sscanf!(&line, "&{} -> {}", name, rest).is_ok() {
            modules.insert(hash(&name), ModuleType::Conjunction(HashMap::new()));
        } else if sscanf!(&line, "%{} -> {}", name, rest).is_ok() {
            modules.insert(hash(&name), ModuleType::FlipFlop(Pulse::Low));
        } else if sscanf!(&line, "{} -> {}", name, rest).is_ok() {
            if name == "broadcaster" {
                modules.insert(hash(&name), ModuleType::Broadcast);
            } else {
                modules.insert(hash(&name), ModuleType::Broadcast);
            }
        } else {
            panic!("Unknown line: {}", line);
        }

        let outgoing: Vec<String> = rest.split(", ").map(|s| s.to_string()).collect();
        for out in &outgoing {
            outgoing_edges
                .entry(hash(&name))
                .or_insert(Vec::new())
                .push(hash(&out));
            incoming_edges
                .entry(hash(&out))
                .or_insert(Vec::new())
                .push(hash(&name));
        }
    }

    modules.insert(hash(&"button".to_string()), ModuleType::Broadcast);
    outgoing_edges.insert(
        hash(&"button".to_string()),
        vec![hash(&"broadcaster".to_string())],
    );
    incoming_edges.insert(
        hash(&"broadcaster".to_string()),
        vec![hash(&"button".to_string())],
    );

    modules.insert(hash(&"output".to_string()), ModuleType::Broadcast);
    outgoing_edges.insert(hash(&"output".to_string()), Vec::new());

    // Set the initial state for the conjuction modules
    for (name, module) in modules.iter_mut() {
        match module {
            ModuleType::Conjunction(pulses) => {
                for input in incoming_edges.get(name).unwrap() {
                    pulses.insert(input.clone(), Pulse::Low);
                }
            }
            _ => {}
        }
    }

    let mut low_count = 0;
    let mut high_count = 0;
    let rx = hash(&"rx".to_string());
    let button = hash(&"button".to_string());
    let broadcaster = hash(&"broadcaster".to_string());

    for i in 0usize..1000000000000000000 {
        if i % 100000 == 0 {
            println!("--------- ITERATION {} ---------", i);
        }
        let mut work: VecDeque<(u64, u64, Pulse)> = VecDeque::new();
        work.push_back((button, broadcaster, Pulse::Low));
        while let Some((origin, dst, pulse)) = work.pop_front() {
            if dst == rx && pulse == Pulse::Low {
                println!("Part 2: {}", (i+1));
                return
            }

            //println!("{} --{:?}--> {}", origin, pulse, dst);
            match pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            }
            let module = match modules.get_mut(&dst) {
                None => continue,
                Some(m) => m,
            };
            match module {
                ModuleType::Broadcast => {
                    let outgoing = outgoing_edges.get(&dst).unwrap();
                    for out in outgoing {
                        work.push_back((dst.clone(), out.clone(), pulse.clone()));
                    }
                }
                ModuleType::FlipFlop(state) => {
                    if pulse == Pulse::Low {
                        state.flip();
                        for out in outgoing_edges.get(&dst).unwrap() {
                            work.push_back((dst.clone(), out.clone(), state.clone()));
                        }
                    }
                }
                ModuleType::Conjunction(pulses) => {
                    *pulses.get_mut(&origin).unwrap() = pulse.clone();
                    let all_high = pulses.values().all(|p| *p == Pulse::High);
                    let output = if all_high { Pulse::Low } else { Pulse::High };
                    for out in outgoing_edges.get(&dst).unwrap() {
                        work.push_back((dst.clone(), out.clone(), output.clone()));
                    }
                }
            }
        }
    }

    println!("Low: {}, High: {}", low_count, high_count);
    println!("Part 1: {}", low_count * high_count);
}
