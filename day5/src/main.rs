use std::{io::BufRead, collections::VecDeque};
use scanf::sscanf;

const MAX_HEIGHT : usize = 8;
const NR_STACKS : usize  = 9;

fn main() {
    let lines : Vec<String> = std::io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::default(); NR_STACKS];

    for line in lines.clone().iter().take(MAX_HEIGHT) {
        for i in 0..NR_STACKS {
            let c = line.chars().nth(4*i+1).unwrap();
            if c != ' ' {
                stacks[i].push_back(c);
            }
        }
    }

    for line in lines.clone().iter().skip(MAX_HEIGHT+2) {
        let mut quant = 0;
        let mut from = 0;
        let mut to = 0;
        sscanf!(line, "move {} from {} to {}", quant, from, to).unwrap();

        // Part 1
        //for _ in 0..quant {
        //    if let Some(cr) = stacks[from-1].pop_front() {
        //        stacks[to-1].push_front(cr);
        //    }
        //}

        // Part 2
        let mut tmp = VecDeque::default();
        for _ in 0..quant {
            if let Some(cr) = stacks[from-1].pop_front() {
                tmp.push_front(cr);
            }
        }
        for _ in 0..quant {
            if let Some(cr) = tmp.pop_front() {
                stacks[to-1].push_front(cr);
            }
        }
    }

    for stack in stacks.iter_mut() {
        print!("{}", stack.pop_front().unwrap());
    }
    println!("");
}
