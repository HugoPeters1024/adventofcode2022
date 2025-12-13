use std::{
    collections::HashSet,
    io::BufRead,
    sync::atomic::AtomicUsize,
};

use bitvec::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sscanf::sscanf;

fn main() {
    let mut presents: Vec<BitVec> = Vec::new();
    let mut targets: Vec<((usize, usize), Vec<usize>)> = Vec::new();

    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    let mut input_needle = 0;
    loop {
        if input_needle >= input.len() {
            break;
        }
        if let Ok((width, height, rest)) =
            sscanf!(&input[input_needle], "{usize}x{usize}: {String}")
        {
            targets.push((
                (width, height),
                rest.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            ));
            input_needle += 1;
        } else if sscanf!(&input[input_needle], "{usize}:").is_ok() {
            let mut present = bitvec![0b0;9];
            for y in 0..3 {
                for (x, c) in input[input_needle + y + 1].chars().enumerate() {
                    if c == '#' {
                        present.set(y * 3 + x, true);
                    }
                }
            }
            presents.push(present);
            input_needle += 5
        }
    }

    let sum = AtomicUsize::new(0);
    targets.par_iter().for_each(|((width, height), amounts)| {
        let mut presents = presents.clone();
        let mut to_go = amounts.clone();
        let mut state = bitvec![0b0; width * height];
        let mut cache = HashSet::new();

        if feasible(
            &mut presents,
            &mut state,
            *width,
            *height,
            &mut to_go,
            &mut cache,
        ) {
            let total = sum.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            println!();
            println!("yes (total now {})", total + 1);
            for y in 0..*height {
                for x in 0..*width {
                    print!(
                        "{}",
                        if state.get(y * width + x).unwrap() == true {
                            'X'
                        } else {
                            '.'
                        }
                    );
                }
                println!();
            }
        } else {
            println!();
            println!("no");
        }
    });

    println!(
        "Part 1: {}/{}",
        sum.load(std::sync::atomic::Ordering::Relaxed),
        targets.len()
    );
}

fn rotate_clockwise_90deg(present: &mut BitVec) {
    let mut temp = bitvec![0b0;9];
    for y in 0..3 {
        for x in 0..3 {
            temp.set(y * 3 + x, present.get((2 - x) * 3 + y).unwrap() == true);
        }
    }
    *present = temp;
}

fn feasible(
    presents: &mut Vec<BitVec>,
    state: &mut BitVec,
    width: usize,
    height: usize,
    to_go: &mut Vec<usize>,
    cache: &mut HashSet<(BitVec, Vec<usize>)>,
) -> bool {
    if cache.contains(&(state.clone(), to_go.clone())) {
        return false;
    }

    if to_go.iter().all(|x| *x == 0) {
        return true;
    }

    for present_choice in 0..to_go.len() {
        if to_go[present_choice] == 0 {
            continue;
        }
        to_go[present_choice] -= 1;

        for _ in 0..4 {
            rotate_clockwise_90deg(&mut presents[present_choice]);
            for y in 0..height - 2 {
                'next_try: for x in 0..width - 2 {
                    let old_state = state.clone();
                    for dy in 0..3 {
                        for dx in 0..3 {
                            if presents[present_choice].get(dy * 3 + dx).unwrap() == true {
                                if state.get((y + dy) * width + x + dx).unwrap() == true {
                                    *state = old_state;
                                    continue 'next_try;
                                }
                                state.set((y + dy) * width + x + dx, true);
                            }
                        }
                    }

                    if feasible(presents, state, width, height, to_go, cache) {
                        return true;
                    }

                    *state = old_state;
                }
            }
        }
        to_go[present_choice] += 1;
    }

    cache.insert((state.clone(), to_go.clone()));
    false
}
