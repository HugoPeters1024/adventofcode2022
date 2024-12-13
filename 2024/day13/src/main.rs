use line_intersection::LineInterval;
use num_integer::gcd_lcm;
use scanf::sscanf;
use std::io::BufRead;
use z3::{
    ast::{self, Ast},
    Config, Context, Optimize,
};

pub fn line_intersection(
    a: (isize, isize),
    adir: (isize, isize),
    b: (isize, isize),
    bdir: (isize, isize),
) -> Option<isize> {
    let (ax, ay) = a;
    let (adir_x, adir_y) = adir;
    let (bx, by) = b;
    let (bdir_x, bdir_y) = bdir;

    let det = adir_x * bdir_y - adir_y * bdir_x;

    if det == 0 {
        // parallel
        return None;
    }

    // parametrize
    let dx = bx - ax;
    let dy = by - ay;

    let t_num = dx * bdir_y - dy * bdir_x;
    let u_num = dx * adir_y - dy * adir_x;

    if t_num % det != 0 || u_num % det != 0 {
        // not whole number intersection
        return None;
    }

    let t = t_num / det;
    Some(t)
}

fn main() {
    let mut sum = 0;

    for quad in std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .chunks(4)
    {
        let mut ax: usize = 0;
        let mut ay: usize = 0;
        let mut bx: usize = 0;
        let mut by: usize = 0;
        let mut px: usize = 0;
        let mut py: usize = 0;

        sscanf!(&quad[0], "Button A: X+{}, Y+{}", ax, ay).unwrap();
        sscanf!(&quad[1], "Button B: X+{}, Y+{}", bx, by).unwrap();
        sscanf!(&quad[2], "Prize: X={}, Y={}", px, py).unwrap();

        let px = px + 10000000000000;
        let py = py + 10000000000000;

        //let cfg = Config::new();
        //let ctx = Context::new(&cfg);
        //let solver = Optimize::new(&ctx);

        //let ax = ast::Int::from_u64(&ctx, ax as u64);
        //let ay = ast::Int::from_u64(&ctx, ay as u64);

        //let bx = ast::Int::from_u64(&ctx, bx as u64);
        //let by = ast::Int::from_u64(&ctx, by as u64);

        //let px = ast::Int::from_u64(&ctx, px as u64);
        //let py = ast::Int::from_u64(&ctx, py as u64);

        //let an = ast::Int::new_const(&ctx, "an");
        //let bn = ast::Int::new_const(&ctx, "bn");

        //let anax = z3::ast::Int::mul(&ctx, &[&an, &ax]);
        //let bnbx = z3::ast::Int::mul(&ctx, &[&bn, &bx]);
        //solver.assert(&(anax + bnbx)._eq(&px));

        //let anay = z3::ast::Int::mul(&ctx, &[&an, &ay]);
        //let bnby = z3::ast::Int::mul(&ctx, &[&bn, &by]);
        //solver.assert(&(anay + bnby)._eq(&py));

        //solver.minimize(&an);
        //if solver.check(&[]) == z3::SatResult::Sat {
        //    let model = solver.get_model().unwrap();

        //    let an = model.eval(&an, true).unwrap().as_u64().unwrap();
        //    let bn = model.eval(&bn, true).unwrap().as_u64().unwrap();
        //    sum += 3 * an + 1 * bn;
        //}
        //
        //let (gcd, _) = gcd_lcm(ax, ay);
        //let ax = ax / gcd;
        //let ay = ay / gcd;

        //let (gcd, _) = gcd_lcm(bx, by);
        //let bx = bx / gcd;
        //let by = by / gcd;

        if let Some(an) = line_intersection(
            (0, 0),
            (ax as isize, ay as isize),
            (px as isize, py as isize),
            (bx as isize, by as isize),
        ) {
            let dx = px - an as usize * ax;
            let bn = dx / bx;
            sum += an as usize * 3 + bn;
        }
    }

    println!("Part 2: {sum}");
}
