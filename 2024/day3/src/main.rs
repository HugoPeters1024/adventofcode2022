use std::io::BufRead;

use regex::Regex;
use scanf::sscanf;

fn main() {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut enabled = true;
    for input in std::io::stdin().lock().lines() {
        let input = input.unwrap();
        for result in regex.find_iter(&input) {
            let str = result.as_str();

            if str == "do()" {
                enabled = true;
                continue;
            }

            if str == "don't()" {
                enabled = false;
                continue;
            }

            let mut l: i64 = 0;
            let mut r: i64 = 0;
            sscanf!(str, "mul({},{})", l, r).unwrap();
            part1_sum += l * r;
            if enabled {
                part2_sum += l * r
            }
        }
    }

    println!("Part 1: {part1_sum}");
    println!("Part 2: {part2_sum}");
}
