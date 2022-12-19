use std::{io::BufRead, collections::{VecDeque, HashSet}};
use scanf::sscanf;

#[derive(Debug, Default, Hash)]
struct BluePrint {
    id: u32,
    ore_cost_ore: u32,
    clay_cost_ore: u32,
    obsidian_cost_ore_clay: (u32, u32),
    geode_cost_ore_obisidion: (u32, u32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,

    ore_income: u32,
    clay_income: u32,
    obsidian_income: u32,
    geode_income: u32,

    time: u32,
}

fn main() {
    let mut blueprints : Vec<BluePrint> = Vec::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut id = 0;
        let mut ore_cost_ore = 0;
        let mut clay_cost_ore = 0;
        let mut obsidian_cost_ore_clay1 = 0;
        let mut obsidian_cost_ore_clay2 = 0;
        let mut geode_cost_ore_obisidion1 = 0;
        let mut geode_cost_ore_obisidion2 = 0;
        sscanf!(
            &line,
            "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            id,
            ore_cost_ore,
            clay_cost_ore,
            obsidian_cost_ore_clay1,
            obsidian_cost_ore_clay2,
            geode_cost_ore_obisidion1,
            geode_cost_ore_obisidion2
        ).unwrap();

        blueprints.push(BluePrint {
            id,
            ore_cost_ore,
            clay_cost_ore,
            obsidian_cost_ore_clay: (obsidian_cost_ore_clay1, obsidian_cost_ore_clay2),
            geode_cost_ore_obisidion: (geode_cost_ore_obisidion1, geode_cost_ore_obisidion2),
        });
    }

    let mut qualities: Vec<u32> = vec![0; blueprints.len()];

    for (bid, blueprint) in blueprints.iter().take(3).enumerate() {
        let mut work: VecDeque<State> = VecDeque::new();
        work.push_back(State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_income: 1,
            clay_income: 0,
            obsidian_income: 0,
            geode_income: 0,

            time: 0,
        });

        let mut visisted : HashSet<State> = HashSet::new();

        while let Some(state) = work.pop_front() {
            if state.time >= 32 {
                if state.geode > qualities[bid] {
                    qualities[bid] = state.geode;
                    dbg!(state.geode);
                }
                continue;
            }

            if visisted.contains(&state) {
                continue;
            }
            visisted.insert(state.clone());

            if best_outcome(&state) <= qualities[bid] {
                continue;
            }

            let mut new_state = state.clone();
            next_tick(&mut new_state);

            {
                work.push_front(new_state.clone());
            }

            if state.ore >= blueprint.ore_cost_ore {
                let mut state = new_state.clone();
                state.ore -= blueprint.ore_cost_ore;
                state.ore_income += 1;
                work.push_front(state);
            } 

            if state.ore >= blueprint.geode_cost_ore_obisidion.0 
            && state.obsidian >= blueprint.geode_cost_ore_obisidion.1 
            {
                let mut state = new_state.clone();
                state.ore -= blueprint.geode_cost_ore_obisidion.0;
                state.obsidian -= blueprint.geode_cost_ore_obisidion.1;
                state.geode_income += 1;
                work.push_front(state);
            }

            if state.ore >= blueprint.clay_cost_ore {
                let mut state = new_state.clone();
                state.ore -= blueprint.clay_cost_ore;
                state.clay_income += 1;
                work.push_front(state);
            }

            if state.ore >= blueprint.obsidian_cost_ore_clay.0 
            && state.clay >= blueprint.obsidian_cost_ore_clay.1 
            {
                let mut state = new_state.clone();
                state.ore -= blueprint.obsidian_cost_ore_clay.0;
                state.clay -= blueprint.obsidian_cost_ore_clay.1;
                state.obsidian_income += 1;
                work.push_front(state);
            } 

        }
    }

    dbg!(&qualities);

    let mut total = 0;
    for (i, quality) in qualities.iter().enumerate() {
        total += (i+1) * *quality as usize
    }
    dbg!(total);

    // part 2
    dbg!(qualities.iter().take(3).map(|x| *x as usize).product::<usize>());
}

fn next_tick(state: &mut State) {
    state.ore += state.ore_income;
    state.clay += state.clay_income;
    state.obsidian += state.obsidian_income;
    state.geode += state.geode_income;
    state.time += 1;
}

fn best_outcome(state: &State) -> u32 {
    let mut best = state.geode;
    let mut income = state.geode_income;
    let mut time = state.time;
    while time < 32 {
        best += income;
        income += 1;
        time += 1;
    }

    best
}
