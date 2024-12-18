use std::io::BufRead;

use scanf::sscanf;
use z3::{Config, Context, Solver};

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
            0 => a = a / (2usize.pow(combo as u32)),
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
            6 => b = a / (2usize.pow(combo as u32)),
            // cdv
            7 => c = a / (2usize.pow(combo as u32)),
            _ => panic!(),
        }
    }

    output
}

fn run_program2(mut a: usize, mut b: usize, mut c: usize, program: &Vec<usize>) -> bool {
    let mut ip = 0;
    let mut output_len = 0;

    loop {
        if ip >= program.len() - 1 {
            break;
        }

        if output_len > program.len() {
            return false;
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
            0 => a = a / (2usize.pow(combo as u32)),
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
                let output = combo % 8;
                if output_len >= program.len() || program[output_len] != output {
                    return false;
                }
                output_len += 1;
            }
            // bdv
            6 => b = a / (2usize.pow(combo as u32)),
            // cdv
            7 => c = a / (2usize.pow(combo as u32)),
            _ => panic!(),
        }
    }

    output_len == program.len()
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

    let mut a = 0;
    loop {
        if run_program2(a, b, c, &program) {
            println!("Part 2: {}", a);
            break;
        }
        a += 1;
        if a % 10000000 == 0 {
            println!("{}", a);
        }
    }
}
