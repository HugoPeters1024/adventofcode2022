use std::{
    io::BufRead,
    ops::{Add, Mul},
};

use itertools::Itertools;
use scanf::sscanf;
use z3::{
    ast::{self, Ast},
    Config, Context, Optimize, SatResult, Solver,
};

#[derive(Debug, Clone)]
struct HailStone {
    start_x: i64,
    start_y: i64,
    start_z: i64,
    vel_x: i64,
    vel_y: i64,
    vel_z: i64,
}

trait Z3Eq {
    fn z3_eq(&self, other: &Self) -> ast::Bool;
}

impl<'a> Z3Eq for ast::Int<'a> {
    fn z3_eq(&self, other: &Self) -> ast::Bool {
        self._eq(other)
    }
}

//impl HailStone {
//    fn find_intersection(line1: &HailStone, line2: &HailStone) -> Option<(f64, f64)> {
//        // Check if the lines are parallel
//        let cross_product = line1.vel_x * line2.vel_y - line1.vel_y * line2.vel_x;
//
//        if cross_product.abs() < 0.0000000001 {
//            // Lines are parallel, no intersection
//            return None;
//        }
//
//        // Solve for t and u
//        let t = ((line2.start_x - line1.start_x) * line2.vel_y - (line2.start_y - line1.start_y) * line2.vel_x) / cross_product;
//
//        let u = -((line1.start_x - line2.start_x) * line1.vel_y - (line1.start_y - line2.start_y) * line1.vel_x) / cross_product;
//
//        if t >= 0.0 && u >= 0.0 {
//            let intersection_point = (line1.start_x + t * line1.vel_x, line1.start_y + t * line1.vel_y);
//
//            Some(intersection_point)
//        } else {
//            None
//        }
//    }
//}

const MIN: f64 = 200000000000000f64;
const MAX: f64 = 400000000000000f64;
//const MIN: f64 = 7.0;
//const MAX: f64 = 27.0;

fn find_perfect_throw(stones: &[HailStone]) -> (i64, i64, i64) {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let (bx, by, bz, bvx, bvy, bvz) = (
        ast::Int::fresh_const(&ctx, "bx"),
        ast::Int::fresh_const(&ctx, "bx"),
        ast::Int::fresh_const(&ctx, "bx"),
        ast::Int::fresh_const(&ctx, "bvx"),
        ast::Int::fresh_const(&ctx, "bvy"),
        ast::Int::fresh_const(&ctx, "bvz"),
    );

    for stone in stones {
        let t = ast::Int::fresh_const(&ctx, "t");
        let sx = ast::Int::from_i64(&ctx, stone.start_x);
        let sy = ast::Int::from_i64(&ctx, stone.start_y);
        let sz = ast::Int::from_i64(&ctx, stone.start_z);
        let vx = ast::Int::from_i64(&ctx, stone.vel_x);
        let vy = ast::Int::from_i64(&ctx, stone.vel_y);
        let vz = ast::Int::from_i64(&ctx, stone.vel_z);

        solver.assert(&sx.add(&t.clone().mul(&vx)).z3_eq(&bx.clone().add(&t.clone().mul(&bvx))));

        solver.assert(&sy.add(&t.clone().mul(&vy)).z3_eq(&by.clone().add(&t.clone().mul(&bvy))));

        solver.assert(&sz.add(&t.clone().mul(&vz)).z3_eq(&bz.clone().add(&t.clone().mul(&bvz))));
    }

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();

            return (
                model.eval(&bx, true).unwrap().as_i64().unwrap(),
                model.eval(&by, true).unwrap().as_i64().unwrap(),
                model.eval(&bz, true).unwrap().as_i64().unwrap(),
            );
        }
        z3::SatResult::Unsat => {
            panic!("No solution found.");
        }
        z3::SatResult::Unknown => {
            panic!("Solver returned unknown result.");
        }
    }
}

//fn find_perfect_throw_ilp(stones: &[HailStone], min: f64, max: f64) -> (f64, f64, f64) {
//    variables! {
//        vars:
//            xstart;
//            ystart;
//            zstart;
//            xvel;
//            yvel;
//            zvel;
//    };
//
//    let mut stone_vars = Vec::new();
//
//    for stone in stones.iter() {
//        let sx = vars.add(variable().min(stone.start_x).max(stone.start_x));
//        let sy = vars.add(variable().min(stone.start_y).max(stone.start_y));
//        let sz = vars.add(variable().min(stone.start_z).max(stone.start_z));
//        let vx = vars.add(variable().min(stone.vel_x).max(stone.vel_x));
//        let vy = vars.add(variable().min(stone.vel_y).max(stone.vel_y));
//        let vz = vars.add(variable().min(stone.vel_z).max(stone.vel_z));
//        let t = vars.add(variable().min(0.0));
//
//        stone_vars.push((sx, sy, sz, vx, vy, vz, t));
//    }
//
//    let mut solver = vars.maximise(xstart + ystart + zstart).using(default_solver);
//
//    //for (sx, sy, sz, vx, vy, vz, t) in stone_vars {
//    //    solver = solver.with(constraint!(sx + vx * t == xstart + xvel * t));
//    //}
//
//    todo!();
//}

fn main() {
    let hailstones: Vec<HailStone> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let (mut start_x, mut start_y, mut start_z, mut vel_x, mut vel_y, mut vel_z) = (0, 0, 0, 0, 0, 0);
            sscanf!(&line.unwrap(), "{}, {}, {} @ {}, {}, {}", start_x, start_y, start_z, vel_x, vel_y, vel_z).unwrap();
            HailStone {
                start_x,
                start_y,
                start_z,
                vel_x,
                vel_y,
                vel_z,
            }
        })
        .collect();

    let mut sum = 0;
    //let max_combi = hailstones.len() * (hailstones.len() - 1) / 2;
    //for (i, combi) in hailstones.iter().combinations(2).enumerate() {
    //    if i % 1000 == 0 {
    //        println!("{} / {} (sum: {})", i, max_combi, sum);
    //    }
    //    let lhs = combi[0];
    //    let rhs = combi[1];
    //    if let Some((x, y)) = HailStone::find_intersection(&lhs, &rhs) {
    //        if x >= MIN && x <= MAX && y >= MIN && y <= MAX {
    //            sum += 1;
    //        }
    //    }
    //}

    println!("Part 1: {}", sum);

    let (x, y, z) = find_perfect_throw(&hailstones);

    println!("Part 2: {}", x + y + z);
}
