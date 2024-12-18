use std::{collections::VecDeque, io::BufRead};

use scanf::sscanf;

fn run_program(mut a: usize, mut b: usize, mut c: usize, program: &Vec<usize>) -> Vec<usize> {
    let mut ip = 0;
    let mut output = Vec::new();

    loop {
        if ip >= program.len() - 1 {
            break;
        }

        let op = program[ip];
        let literal = program[ip + 1];
        ip += 2;
        let combo = match literal {
            0 | 1 | 2 | 3 => literal,
            4 => a,
            5 => b,
            6 => c,
            7 => 0,
            _ => panic!(),
        };

        match op {
            // adv
            0 => a = a >> combo,
            // bxl
            1 => b = b ^ literal,
            // bst
            2 => b = combo % 8,
            // jnz
            3 => {
                if a != 0 {
                    ip = literal;
                }
            }
            // bxc
            4 => b = b ^ c,
            // out
            5 => {
                output.push(combo % 8);
            }
            // bdv
            6 => b = a >> combo,
            // cdv
            7 => c = a >> combo,
            _ => panic!(),
        }
    }

    output
}

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut program_str = String::new();
    sscanf!(&lines.next().unwrap().unwrap(), "Register A: {}", a).unwrap();
    sscanf!(&lines.next().unwrap().unwrap(), "Register B: {}", b).unwrap();
    sscanf!(&lines.next().unwrap().unwrap(), "Register C: {}", c).unwrap();
    lines.next();
    sscanf!(&lines.next().unwrap().unwrap(), "Program: {}", program_str).unwrap();

    let program: Vec<usize> = program_str
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let part1 = run_program(a, b, c, &program);
    print!("Part 1: ");
    for d in part1 {
        print!("{},", d);
    }
    println!();

    let mut work = VecDeque::new();
    work.push_front(0);

    'outer: while let Some(base) = work.pop_back() {
        for i in 0..8 {
            let output = run_program(base + i, b, c, &program);
            let suffix = &program[program.len() - output.len()..];

            if output == program {
                println!("Part 2: {}", base + i);
                break 'outer;
            }

            if output == suffix {
                work.push_front((base + i) * 8);
                for d in output {
                    print!("{},", d);
                }
                println!();
            };
        }
    }
}
