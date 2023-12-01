use std::{io::BufRead, collections::VecDeque};

use scanf::sscanf;

#[derive(Debug, Clone)]
enum Op {
    Add(usize),
    Mul(usize),
    AddId,
    MulId,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Op,
    test: usize,
    yes: usize,
    no: usize,
    count: usize,
}

fn main() {
    let lines : Vec<String> = std::io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut monkeys : Vec<Monkey> = Vec::new();

    let mut line_idx = 0;
    while line_idx < lines.len() {
        // skip header
        line_idx += 1;
        let items = lines[line_idx]
            .chars().skip(18)
            .collect::<String>()
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        line_idx += 1;
        let mut op: String = "".to_string();
        let mut opval : String = "".to_string();

        sscanf!(&lines[line_idx], "  Operation: new = old {} {}", op, opval).unwrap();

        let op = if let Ok(val) = opval.parse::<usize>() {
            if op == "+" { Op::Add(val) } else { Op::Mul(val) }
        } else {
            if op == "+" { Op::AddId } else { Op::MulId }
        };

        line_idx += 1;
        let mut test = 0;
        sscanf!(&lines[line_idx], "  Test: divisible by {}", test).unwrap();

        line_idx+=1;
        let mut yes = 0;
        sscanf!(&lines[line_idx], "   If true: throw to monkey {}", yes).unwrap();

        line_idx+=1;
        let mut no = 0;
        sscanf!(&lines[line_idx], "   If false: throw to monkey {}", no).unwrap();

        monkeys.push(Monkey { items, operation: op, test, yes, no, count: 0 });
        line_idx += 2;
    }

    let composite : usize = monkeys.iter().map(|x| x.test).product();

    for round in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_idx].items.pop_front() {
                let new_item = match monkeys[monkey_idx].operation {
                    Op::Add(val) => item + val,
                    Op::Mul(val) => item * val,
                    Op::AddId => item + item,
                    Op::MulId => item * item,
                };

                // part1
 //               let new_item = new_item / 3;

                let new_item = new_item % composite;

                monkeys[monkey_idx].count += 1;

                if new_item % monkeys[monkey_idx].test == 0 {
                    let to = monkeys[monkey_idx].yes;
                    monkeys[to].items.push_back(new_item);
                } else {
                    let to = monkeys[monkey_idx].no;
                    monkeys[to].items.push_back(new_item);
                }
            }
        }
    }

    dbg!(&monkeys);
    let mut counts : Vec<usize> = monkeys.iter().map(|x| x.count).collect();
    dbg!(&counts);
    counts.sort();
    counts.reverse();
    dbg!(counts[0] * counts[1]);
}
