use std::{collections::HashMap, io::BufRead};

use scanf::sscanf;
use z3::{
    ast::{self, Ast},
    Config, Context, Optimize, Solver,
};

fn main() {
    let mut cfg = Config::new();
    cfg.set_bool_param_value("type_check", true);
    cfg.set_bool_param_value("stats", true);
    let ctx = Context::new(&cfg);
    let solver = Optimize::new(&ctx);

    let mut inputs: Vec<ast::Int> = Vec::new();
    for i in 0..1 {
        let digit = ast::Int::new_const(&ctx, format!("d{}", i));
        solver.assert(&digit.le(&ast::Int::from_u64(&ctx, 9)));
        solver.assert(&digit.ge(&ast::Int::from_u64(&ctx, 1)));
        inputs.push(digit);
    }

    let mut env: HashMap<char, ast::Int> = HashMap::new();
    let mut input_needle = 0;

    env.insert('x', ast::Int::from_u64(&ctx, 0));
    env.insert('y', ast::Int::from_u64(&ctx, 0));
    env.insert('z', ast::Int::from_u64(&ctx, 0));
    env.insert('w', ast::Int::from_u64(&ctx, 0));

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        println!("{}", line);

        if line == "inp w" {
            env.insert('w', inputs[input_needle%inputs.len()].clone());
            input_needle += 1;
            continue;
        }

        let mut op = String::new();
        let mut lhs_name = '.';
        let mut rhs_name = String::new();
        sscanf!(&line, "{} {} {}", op, lhs_name, rhs_name).unwrap();

        let lhs = env.get(&lhs_name).unwrap();
        let rhs = match rhs_name.as_str() {
            "x" => env.get(&'x').unwrap(),
            "y" => env.get(&'y').unwrap(),
            "z" => env.get(&'z').unwrap(),
            "w" => env.get(&'w').unwrap(),
            _ => &ast::Int::from_i64(&ctx, rhs_name.as_str().parse::<i64>().unwrap()),
        };

        env.insert(
            lhs_name,
            match op.as_str() {
                "add" => {
                    if lhs.as_i64() == Some(0) {
                        rhs.clone()
                    } else if rhs.as_i64() == Some(0) {
                        lhs.clone()
                    } else {
                        lhs + rhs
                    }
                }
                "mul" => {
                    if rhs.as_i64() == Some(0) {
                        rhs.clone()
                    } else {
                        lhs * rhs
                    }
                }
                "div" => {
                    if lhs.as_i64() == Some(0) {
                        lhs.clone()
                    } else if rhs.as_i64() == Some(1) {
                        lhs.clone()
                    } else {
                        solver.assert(&rhs._eq(&ast::Int::from_u64(&ctx, 0)).not());
                        lhs / rhs
                    }
                }
                "mod" => lhs - (lhs / rhs) * rhs,
                "eql" => {
                    if lhs.as_u64().is_some()
                        && rhs.as_u64().is_some()
                        && lhs.as_u64() == rhs.as_u64()
                    {
                        ast::Int::from_u64(&ctx, 1)
                    } else {
                        ast::Bool::ite(
                            &lhs._eq(&rhs),
                            &ast::Int::from_u64(&ctx, 1),
                            &ast::Int::from_u64(&ctx, 0),
                        )
                    }
                }
                _ => panic!(),
            },
        );
    }

    //solver.assert(&env.get(&'z').unwrap()._eq(&ast::Int::from_u64(&ctx, 0)));

    let score = inputs.iter().fold(ast::Int::from_u64(&ctx, 1), |acc, v| {
        acc * ast::Int::from_u64(&ctx, 10) + v
    });

    //solver.assert(&score.gt(&ast::Int::from_u64(&ctx, 11111111111111)));

    solver.maximize(&score);

    if solver.check(&[]) == z3::SatResult::Sat {
        let model = solver.get_model().unwrap();

        for digit in inputs {
            let v = model.eval(&digit, true).unwrap().as_i64().unwrap();
            print!("{}", v);
        }
        println!();

        for (reg, expr) in env {
            let v = model.eval(&expr, true).unwrap().as_i64().unwrap();
            println!("{} = {}", reg, v);
        }

        let v = model.eval(&score, true).unwrap().as_i64().unwrap();
        println!("score = {}", v);

        println!();
    } else {
        println!("unsat!");
    }
}
