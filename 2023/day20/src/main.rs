use std::{
    collections::{VecDeque, HashMap},
    io::BufRead,
};

use scanf::sscanf;

#[derive(Debug)]
enum ModuleType {
    NonEx,
    FlipFlop(Pulse),
    Conjunction(HashMap<usize, Pulse>),
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

    let mut name_to_id: HashMap<String, usize> = HashMap::new();
    let mut id_to_name: HashMap<usize, String> = HashMap::new();
    let mut max_id = 0;

    let mut register = |name: &String| {
        if let Some(id) = name_to_id.get(name) {
            return *id;
        }
        let id = max_id;
        name_to_id.insert(name.clone(), id);
        id_to_name.insert(id, name.clone());
        max_id += 1;
        return id;
    };


    let mut modules: Vec<ModuleType> = Vec::new();
    for i in 0..input.len()*2 {
        modules.insert(i, ModuleType::NonEx);
    }
    let mut incoming_edges: Vec<Vec<usize>> = vec![Vec::new(); input.len()*2];
    let mut outgoing_edges: Vec<Vec<usize>> = vec![Vec::new(); input.len()*2];

    let mut name = String::new();
    let mut rest = String::new();
    // First we discover all the nodes
    for line in &input {
        if sscanf!(&line, "&{} -> {}", name, rest).is_ok() {
            modules[register(&name)] = ModuleType::Conjunction(HashMap::new());
        } else if sscanf!(&line, "%{} -> {}", name, rest).is_ok() {
            modules[register(&name)] = ModuleType::FlipFlop(Pulse::Low);
        } else if sscanf!(&line, "{} -> {}", name, rest).is_ok() {
            if name == "broadcaster" {
                modules[register(&name)] = ModuleType::Broadcast;
            } else {
                modules[register(&name)] = ModuleType::Broadcast;
            }
        } else {
            panic!("Unknown line: {}", line);
        }

        let outgoing: Vec<String> = rest.split(", ").map(|s| s.to_string()).collect();
        for out in &outgoing {
            outgoing_edges[register(&name)].push(register(&out));
            incoming_edges[register(&out)].push(register(&name));
        }
    }

    let rx = register(&"rx".to_string());
    let button = register(&"button".to_string());
    let broadcaster = register(&"broadcaster".to_string());
    let output = register(&"output".to_string());

    modules[button] = ModuleType::Broadcast;
    outgoing_edges[button] = vec![broadcaster];
    incoming_edges[broadcaster] = vec![button];

    modules[output] = ModuleType::Broadcast;
    outgoing_edges[output] = vec![rx];

    // Set the initial state for the conjuction modules
    for (name, module) in modules.iter_mut().enumerate() {
        match module {
            ModuleType::Conjunction(pulses) => {
                for input in &incoming_edges[name] {
                    pulses.insert(input.clone(), Pulse::Low);
                }
            }
            _ => {}
        }
    }

    let mut low_count = 0;
    let mut high_count = 0;

    dbg!(&max_id);

    let mut work: VecDeque<(usize, usize, Pulse)> = VecDeque::new();

    for i in 0usize..1000 {
        if i % 100000 == 0 {
            println!("--------- ITERATION {} ---------", i);
        }
        work.clear();
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
            let module = &mut modules[dst];
            match module {
                ModuleType::Broadcast => {
                    let outgoing = &outgoing_edges[dst];
                    for out in outgoing {
                        work.push_back((dst.clone(), out.clone(), pulse.clone()));
                    }
                }
                ModuleType::FlipFlop(state) => {
                    if pulse == Pulse::Low {
                        state.flip();
                        for out in &outgoing_edges[dst] {
                            work.push_back((dst.clone(), out.clone(), state.clone()));
                        }
                    }
                }
                ModuleType::Conjunction(pulses) => {
                    *pulses.get_mut(&origin).unwrap() = pulse.clone();
                    let all_high = pulses.values().all(|p| *p == Pulse::High);
                    let output = if all_high { Pulse::Low } else { Pulse::High };
                    for out in &outgoing_edges[dst] {
                        work.push_back((dst.clone(), out.clone(), output.clone()));
                    }
                },
                ModuleType::NonEx => {}
            }
        }
    }

    println!("Low: {}, High: {}", low_count, high_count);
    println!("Part 1: {}", low_count * high_count);
}
