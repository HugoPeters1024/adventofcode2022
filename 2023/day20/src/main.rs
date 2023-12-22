use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

use scanf::sscanf;

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(Pulse),
    Conjunction(usize),
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

    let mut modules: Vec<ModuleType> = vec![ModuleType::Broadcast; input.len() * 2];
    let mut incoming_edges: Vec<Vec<usize>> = vec![Vec::new(); input.len() * 2];
    let mut outgoing_edges: Vec<Vec<usize>> = vec![Vec::new(); input.len() * 2];

    let mut name = String::new();
    let mut rest = String::new();

    for line in &input {
        if sscanf!(&line, "&{} -> {}", name, rest).is_ok() {
            modules[register(&name)] = ModuleType::Conjunction(0xffffffff);
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
    outgoing_edges[output] = Vec::new();

    modules[rx] = ModuleType::Broadcast;

    // Set the initial state for the conjuction modules
    for (name, module) in modules.iter_mut().enumerate() {
        match module {
            ModuleType::Conjunction(pulses) => {
                for input in &incoming_edges[name] {
                    *pulses ^= 1 << *input;
                }
            }
            _ => {}
        }
    }

    let mut low_count = 0;
    let mut high_count = 0;

    let bg = register(&"bg".to_string());
    let ls = register(&"ls".to_string());
    let qq = register(&"qq".to_string());
    let sj = register(&"sj".to_string());

    dbg!(&max_id);

    let mut work: VecDeque<(usize, usize, Pulse)> = VecDeque::new();

    let mut prev: HashMap<usize, usize> = HashMap::new();
    let mut to_lcm: Vec<usize> = Vec::new();
    let mut count = vec![0usize; max_id + 100];

    for i in 1usize..1000000 {
        if i & (1024 * 1024 - 1) == 1 {
            println!("--------- ITERATION {} ---------", i);
        }
        work.push_back((button, broadcaster, Pulse::Low));
        while let Some((origin, dst, pulse)) = work.pop_front() {
            if pulse == Pulse::Low {
                if let Some(prev_tick) = prev.get(&dst) {
                    if count[dst] == 2 && (dst == bg || dst == ls || dst == qq || dst == sj) {
                        println!(
                            "cycle for {} = {}",
                            id_to_name.get(&dst).unwrap(),
                            i - prev_tick
                        );
                        to_lcm.push(i - prev_tick);
                    }
                }
                prev.insert(dst, i);
                count[dst] += 1;
            }

            if to_lcm.len() == 4 {
                dbg!(&to_lcm);
                let lcm = lcm(&to_lcm);
                println!("Part 2: {}", lcm);
                return;
            }
            // Too low: 11678800
            // Too high: 15705176939338545716
            //println!("Part 2: {}", (i + 1));
            //return;

            //println!("{} --{:?}--> {}", id_to_name.get(&origin).unwrap(), pulse, id_to_name.get(&dst).unwrap());

            // sleep 1s
            //std::thread::sleep(std::time::Duration::from_millis(1000));

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
                    match pulse {
                        // Set the bit to 0
                        Pulse::Low => *pulses &= !(1 << origin),
                        // Set the bit to 1
                        Pulse::High => *pulses |= 1 << origin,
                    }
                    let all_high = *pulses == 0xffffffff;
                    let output = if all_high { Pulse::Low } else { Pulse::High };
                    for out in &outgoing_edges[dst] {
                        work.push_back((dst.clone(), out.clone(), output.clone()));
                    }
                }
            }
        }
    }

    println!("Low: {}, High: {}", low_count, high_count);
    println!("Part 1: {}", low_count * high_count);
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
