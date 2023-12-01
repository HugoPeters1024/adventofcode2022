use std::{collections::HashMap, io::BufRead};

use scanf::sscanf;

use z3::{*, ast::Ast};

#[derive(Debug)]
enum Action {
    Lit(i64),
    Op(String, char, String),
}

fn main() {
    let mut monkeys : HashMap<String, Action> = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut monkey = "".to_string();
        let mut action_str = "".to_string();
        sscanf!(&line, "{}: {}", monkey, action_str).unwrap();
        monkeys.insert(monkey, parse_action(&action_str));
    }

    // part 1
    dbg!(eval(&monkeys, &"root".to_string()));

    let mut cfg = Config::new();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // part 2
    let (mleft, mright) = match monkeys.get(&"root".to_string()).unwrap() {
        Action::Op(mleft, _, mright) => (mleft, mright),
        _ => panic!("root is not an op"),
    };

    let mut humn = None;
    let lhs = eval_z3(&ctx, &monkeys, mleft, &mut humn);
    let rhs = eval_z3(&ctx, &monkeys, mright, &mut humn);
    solver.assert(&lhs._eq(&rhs));

    dbg!(solver.check());
    let m = solver.get_model().unwrap();
    let v = humn.unwrap();
    dbg!(m.eval(&v, true).unwrap().as_i64().unwrap());
}


fn parse_action(inp: &str) -> Action {
    let mut lit = 0;
    if sscanf!(inp, "{}", lit).is_ok() {
        return Action::Lit(lit);
    }

    let mut op_char : char = ' ';
    let mut mleft = "".to_string();
    let mut mright = "".to_string();

    if sscanf!(inp, "{} {} {}", mleft, op_char, mright).is_ok() {
        return Action::Op(mleft, op_char, mright);
    }

    panic!();
}

fn eval(monkeys: &HashMap<String, Action>, monkey: &String) -> i64 {
    match monkeys.get(monkey).unwrap() {
        Action::Lit(lit) => *lit,
        Action::Op(mleft, op_char, mright) => {
            let left = eval(monkeys, mleft);
            let right = eval(monkeys, mright);
            match op_char {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => panic!(),
            }
        }
    }
}

fn eval_z3<'a>(ctx: &'a Context, monkeys: &HashMap<String, Action>, monkey: &String, humn: &mut Option<z3::ast::Int<'a>>) -> z3::ast::Int<'a> {
    let action = monkeys.get(monkey).unwrap();
    if monkey == "humn" {
        let v = z3::ast::Int::new_const(ctx, "x");
        humn.replace(v.clone());
        return v;
    }
    match action {
        Action::Lit(lit) => z3::ast::Int::from_i64(ctx, *lit),
        Action::Op(mleft, op_char, mright) => {
            let left = eval_z3(ctx, monkeys, mleft, humn);
            let right = eval_z3(ctx, monkeys, mright, humn);
            match op_char {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => panic!(),
            }
        }
    }
}
