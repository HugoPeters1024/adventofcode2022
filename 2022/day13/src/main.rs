use std::io::BufRead;
use serde_json::{Result,Value};

fn main() {
    let mut pairs: Vec<(Value, Value)> = Vec::new();

    let input : Vec<String> = std::io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut line_idx = 0;

    while line_idx < input.len() {
        let lhs: Value = serde_json::from_str(&input[line_idx]).unwrap();
        line_idx += 1;
        let rhs: Value = serde_json::from_str(&input[line_idx]).unwrap();
        line_idx += 2;
        pairs.push((lhs,rhs));
    }

    let mut in_order_idxs : Vec<u32> = Vec::new();
    for (i, (lhs,rhs)) in pairs.iter().enumerate() {
        if in_order(lhs, rhs).is_in_order() {
            in_order_idxs.push(i as u32 + 1);
        }
    }

    dbg!(in_order_idxs.iter().sum::<u32>());

    // part 2
    let marker1 : Value = serde_json::from_str("[[2]]").unwrap();
    let marker2 : Value = serde_json::from_str("[[6]]").unwrap();

    let mut all_items: Vec<Value> = Vec::new();
    for pair in pairs {
        all_items.push(pair.0);
        all_items.push(pair.1);
    }
    all_items.push(marker1.clone());
    all_items.push(marker2.clone());

    all_items.sort_by(|a,b| in_order(a,b).to_ordering());
    let mut marker1_idx = 0;
    let mut marker2_idx = 0;

    for (i, item) in all_items.iter().enumerate() {
        if *item == marker1 {
            marker1_idx = i + 1;
        }
        if *item == marker2 {
            marker2_idx = i + 1;
        }
    }

    dbg!(marker1_idx * marker2_idx);
}

enum Res {
    InOrder,
    NotInOrder,
    Continue,
}

impl Res {
    fn is_in_order(&self) -> bool {
        match self {
            Res::InOrder => true,
            _ => false,
        }
    }

    fn to_ordering(&self) -> std::cmp::Ordering {
        match self {
            Res::InOrder => std::cmp::Ordering::Less,
            Res::NotInOrder => std::cmp::Ordering::Greater,
            Res::Continue => panic!(),
        }
    }
}

fn in_order(lhs: &Value, rhs: &Value) -> Res {
    match (lhs,rhs) {
        (Value::Number(lhs), Value::Array(rhs)) => in_order(&Value::Array(vec![Value::Number(lhs.clone())]), &Value::Array(rhs.clone())),
        (Value::Array(lhs), Value::Number(rhs)) => in_order(&Value::Array(lhs.clone()), &Value::Array(vec![Value::Number(rhs.clone())])),
        (Value::Number(lhs), Value::Number(rhs)) => {
            if lhs.as_f64() < rhs.as_f64() {
                Res::InOrder
            } else if lhs.as_f64() == rhs.as_f64() {
                Res::Continue
            } else {
                Res::NotInOrder
            }
        }
        (Value::Array(lhs), Value::Array(rhs)) => {
            let mut lhs_iter = lhs.iter();
            let mut rhs_iter = rhs.iter();
            loop {
                match (lhs_iter.next(), rhs_iter.next()) {
                    (Some(lhs), Some(rhs)) => {
                        match in_order(lhs, rhs) {
                            Res::InOrder => return Res::InOrder,
                            Res::NotInOrder => return Res::NotInOrder,
                            Res::Continue => continue,
                        }
                    },
                    (None, _) => return Res::InOrder,
                    (_, None) => return Res::NotInOrder,
                }
            }
        },
        _ => panic!(),
    }
}
