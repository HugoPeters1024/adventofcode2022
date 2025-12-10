use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

use bitvec::prelude::*;
use nom::{
    IResult, Parser,
    bytes::complete::take_while1,
    character::complete::{char, digit1, space0},
    combinator::map_res,
    multi::separated_list1,
    sequence::delimited,
};
use z3::{
    Optimize,
    ast::{self, Int},
};

#[derive(Debug)]
struct Lights {
    desired: BitVec,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn parse_state(input: &str) -> IResult<&str, BitVec> {
    let (input, state_str) =
        delimited(char('['), take_while1(|c| c == '.' || c == '#'), char(']')).parse(input)?;

    let mut bv = bitvec![0b0; state_str.len()];
    for (idx, v) in state_str.chars().map(|c| c == '#').enumerate() {
        bv.set(idx, v);
    }
    Ok((input, bv))
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>()).parse(input)
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('('),
        separated_list1(char(','), parse_number),
        char(')'),
    )
    .parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('{'),
        separated_list1(char(','), parse_number),
        char('}'),
    )
    .parse(input)
}

fn parse_lights(input: &str) -> IResult<&str, Lights> {
    let (input, state) = parse_state(input)?;
    let (input, _) = space0(input)?;

    let (input, buttons) = separated_list1(char(' '), parse_number_list).parse(input)?;

    let (input, _) = space0(input)?;
    let (input, joltage) = parse_joltage(input)?;

    Ok((
        input,
        Lights {
            desired: state,
            buttons,
            joltage,
        },
    ))
}

fn main() {
    let lights: Vec<Lights> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse_lights(&l.unwrap()).unwrap().1)
        .collect();

    let mut sum = 0;
    for light in lights.iter() {
        sum += min_presses_part1(&light);
    }

    println!("Part 1 {sum}");

    let mut sum = 0;
    for light in lights.iter() {
        sum += min_presses_part2(&light);
    }

    println!("Part 2 {sum}");
}

fn min_presses_part1(lights: &Lights) -> usize {
    let mut work: VecDeque<(BitVec, usize)> = VecDeque::new();
    let mut seen: HashMap<BitVec, usize> = HashMap::new();
    work.push_front((bitvec![0b0; lights.desired.len()], 0));

    while let Some((state, nr_presses)) = work.pop_front() {
        if let Some(best) = seen.get(&state) {
            if nr_presses >= *best {
                continue;
            }
        }

        seen.insert(state.clone(), nr_presses);

        if state == lights.desired {
            return nr_presses;
        }

        for button in lights.buttons.iter() {
            let mut new_state = state.clone();
            for light in button.iter() {
                new_state.set(*light, !state.get(*light).unwrap());
            }

            work.push_back((new_state, nr_presses + 1));
        }
    }

    panic!()
}

fn min_presses_part2(lights: &Lights) -> usize {
    let solver = Optimize::new();

    let mut presses = Vec::new();
    let mut total_presses = Int::from_u64(0);
    for i in 0..lights.buttons.len() {
        let v = ast::Int::new_const(format!("button_{}", i));
        solver.assert(&v.ge(Int::from_u64(0)));
        total_presses = &total_presses + &v;
        presses.push(v);
    }

    for joltage_idx in 0..lights.joltage.len() {
        let mut expr = Int::from_u64(0);
        for (button_idx, press_var) in presses.iter().enumerate() {
            let button = &lights.buttons[button_idx];
            for x in button.iter() {
                if *x == joltage_idx {
                    expr = &expr + press_var;
                }
            }
        }

        solver.assert(&expr.eq(Int::from_u64(lights.joltage[joltage_idx] as u64)));
    }

    solver.minimize(&total_presses);

    assert_eq!(solver.check(&[]), z3::SatResult::Sat);
    let model = solver.get_model().unwrap();

    model.eval(&total_presses, true).unwrap().as_u64().unwrap() as usize
}
