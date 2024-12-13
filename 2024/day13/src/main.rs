use scanf::sscanf;
use std::io::BufRead;
use z3::{
    ast::{self, Ast},
    Config, Context, Optimize,
};

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

        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Optimize::new(&ctx);

        let ax = ast::Int::from_u64(&ctx, ax as u64);
        let ay = ast::Int::from_u64(&ctx, ay as u64);

        let bx = ast::Int::from_u64(&ctx, bx as u64);
        let by = ast::Int::from_u64(&ctx, by as u64);

        let px = ast::Int::from_u64(&ctx, px as u64);
        let py = ast::Int::from_u64(&ctx, py as u64);

        let an = ast::Int::new_const(&ctx, "an");
        let bn = ast::Int::new_const(&ctx, "bn");

        let anax = z3::ast::Int::mul(&ctx, &[&an, &ax]);
        let bnbx = z3::ast::Int::mul(&ctx, &[&bn, &bx]);
        solver.assert(&(anax + bnbx)._eq(&px));

        let anay = z3::ast::Int::mul(&ctx, &[&an, &ay]);
        let bnby = z3::ast::Int::mul(&ctx, &[&bn, &by]);
        solver.assert(&(anay + bnby)._eq(&py));

        solver.minimize(&an);
        if solver.check(&[]) == z3::SatResult::Sat {
            let model = solver.get_model().unwrap();

            let an = model.eval(&an, true).unwrap().as_u64().unwrap();
            let bn = model.eval(&bn, true).unwrap().as_u64().unwrap();
            sum += 3 * an + 1 * bn;
        }
    }

    println!("Part 2: {sum}");
}
