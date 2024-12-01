use std::{collections::HashMap, io::BufRead};

use scanf::sscanf;

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut l = 0;
        let mut r = 0;
        sscanf!(&line, "{}   {}", l, r).unwrap();
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let diffsum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("total dist: {diffsum}");

    let mut righthist: HashMap<i32, i32> = HashMap::new();

    for el in right {
        *righthist.entry(el).or_default() += 1;
    }

    let mut histsum = 0;
    for x in left {
        histsum += x * *righthist.entry(x).or_default();
    }

    println!("simalilarity: {histsum}");
}
