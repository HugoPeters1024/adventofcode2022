use std::io::BufRead;
use scanf::sscanf;

fn main() {
    let lines : Vec<String> = std::io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let ranges = lines.into_iter().map(|l| {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        sscanf!(l.as_str(), "{}-{},{}-{}", a, b, c, d).unwrap();
        ((a, b), (c, d))
    }).collect::<Vec<_>>();

    // day1
    let mut total = 0;
    for ((a,b), (c,d)) in ranges.clone() {
        if a >= c && b <= d {
            total += 1;
        } else if c >= a && d <= b {
            total += 1;
        }
    }
    dbg!(total);

    // day1
    let mut total = 0;
    for ((a,b), (c,d)) in ranges.clone() {
        if a >= c && a <= d {
            total += 1;
        } else if b >= c && b <= d {
            total += 1;
        } else if c >= a && c <= b {
            total += 1;
        } else if d >= a && d <= b {
            total += 1;
        }
    }
    dbg!(total);
}
