use scanf::sscanf;
use std::{collections::HashMap, io::BufRead};

#[derive(Clone, Debug)]
enum Expr {
    Const(bool),
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

fn eval(expr: &String, env: &mut HashMap<String, Expr>) -> bool {
    let res = match env.get(expr).unwrap().clone() {
        Expr::Const(c) => c,
        Expr::And(lhs, rhs) => eval(&lhs, env) && eval(&rhs, env),
        Expr::Or(lhs, rhs) => eval(&lhs, env) || eval(&rhs, env),
        Expr::Xor(lhs, rhs) => eval(&lhs, env) ^ eval(&rhs, env),
    };

    env.insert(expr.clone(), Expr::Const(res));
    res
}

fn part1(input: &Vec<String>) {
    let mut env: HashMap<String, Expr> = HashMap::new();

    for line in input {
        let mut const_val: u32 = 0;
        let mut const_name = String::new();
        if sscanf!(&line, "{}: {}", const_name, const_val).is_ok() {
            env.insert(const_name, Expr::Const(const_val == 1));
        }

        let mut lhs = String::new();
        let mut rhs = String::new();
        let mut assign_to = String::new();
        let mut op = String::new();

        if sscanf!(&line, "{} {} {} -> {}", lhs, op, rhs, assign_to).is_ok() {
            let expr = match op.as_str() {
                "OR" => Expr::Or,
                "XOR" => Expr::Xor,
                "AND" => Expr::And,
                _ => panic!(),
            };
            env.insert(assign_to, expr(lhs, rhs));
        }
    }

    let mut answer: usize = 0;
    for z_idx in (0..64).rev() {
        let varname = format!("z{:02}", z_idx);
        if !env.contains_key(&varname) {
            continue;
        }
        let res = eval(&varname, &mut env);
        if res {
            answer = (answer << 1) + 1;
        } else {
            answer = answer << 1;
        }
    }

    println!("Part 1: {answer}");
}

fn swap_output(lhs: String, rhs: String, env: &mut HashMap<String, Expr>) {
    let elhs = env.get(&lhs).unwrap().clone();
    let erhs = env.get(&rhs).unwrap().clone();
    env.insert(lhs, erhs);
    env.insert(rhs, elhs);
}

fn main() {
    let input: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();

    part1(&input);

    let mut env: HashMap<String, Expr> = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        let mut lhs = String::new();
        let mut rhs = String::new();
        let mut assign_to = String::new();
        let mut op = String::new();

        if sscanf!(&line, "{} {} {} -> {}", lhs, op, rhs, assign_to).is_ok() {
            let expr = match op.as_str() {
                "OR" => Expr::Or,
                "XOR" => Expr::Xor,
                "AND" => Expr::And,
                _ => panic!(),
            };
            //match op.as_str() {
            //    "OR" => println!("node{} [shape=invtriangle, label=\"OR\"]", i),
            //    "AND" => println!("node{} [shape=triangle, label=\"AND\"]", i),
            //    "XOR" => println!("node{} [shape=diamond, label=\"XOR\"]", i),
            //    _ => panic!(),
            //};
            //println!("{} -> node{}", lhs, i);
            //println!("{} -> node{}", rhs, i);
            //println!("node{} -> {}", i, assign_to);

            env.insert(assign_to, expr(lhs, rhs));
        }
    }

    swap_output("z10".to_string(), "vcf".to_string(), &mut env);
    swap_output("z17".to_string(), "fhg".to_string(), &mut env);
    swap_output("dvb".to_string(), "fsq".to_string(), &mut env);
    swap_output("z39".to_string(), "tnc".to_string(), &mut env);

    let mut swaps: Vec<&str> = vec!["z10", "vcf", "z17", "fhg", "dvb", "fsq", "z39", "tnc"];
    swaps.sort();
    println!("Part 2: {}", swaps.join(","));

    let mut env = env.clone();
    for idx in 0..46 {
        env.insert(format!("x{:02}", idx), Expr::Const(true));
        //env.insert(format!("x{:02}", idx), Expr::Const(idx==39));
        env.insert(format!("y{:02}", idx), Expr::Const(false));
    }

    let mut actual_output = 0;

    for z_idx in (0..46).rev() {
        let varname = format!("z{:02}", z_idx);
        let res = eval(&varname, &mut env);
        if res {
            actual_output = (actual_output << 1) + 1;
        } else {
            actual_output = actual_output << 1;
        }
    }

    let expected_output = 2usize.pow(45) - 1;
    //let expected_output: usize = 1 << 39;
    if expected_output != actual_output {
        println!("expected: {:064b}", expected_output);
        println!("actual:   {:064b}", actual_output);
        for bit_idx in 0..64 {
            let lhs = (expected_output >> bit_idx) & 1;
            let rhs = (actual_output >> bit_idx) & 1;
            if lhs != rhs {
                println!("first wrong bit_idx: {}", bit_idx);
                break;
            }
        }
    }
}
