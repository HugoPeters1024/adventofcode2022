use std::{io::BufRead, collections::VecDeque};

use itertools::Itertools;

fn main() {
    let inputs: Vec<VecDeque<i64>> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for input in &inputs {
        let mut layers: Vec<VecDeque<i64>> = vec![input.clone()];
        loop {
            let next_layer: VecDeque<i64> = layers
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(x, y)| (y - x))
                .collect();
            if next_layer.iter().all(|x| *x == 0) {
                layers.push(next_layer);
                break;
            }
            layers.push(next_layer);
        }

        layers.last_mut().unwrap().push_back(0);
        layers.last_mut().unwrap().push_front(0);
        for i in 1..layers.len() {
            let i = layers.len() - i - 1;
            let newval_right = layers[i].iter().last().unwrap() + layers[i + 1].iter().last().unwrap();
            let newval_left = layers[i][0] - layers[i + 1][0];
            layers[i].push_back(newval_right);
            layers[i].push_front(newval_left);
        }

        sum1 += layers[0].iter().last().unwrap();
        sum2 += layers[0][0];
    }

    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum2);
}
