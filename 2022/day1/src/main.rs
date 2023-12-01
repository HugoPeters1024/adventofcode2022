use std::io::BufRead;

fn main() {
    let lines : Vec<String> = std::io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut totals : Vec<i32> = Vec::new();
    let mut current = 0;
    for line in lines {
        if line.len() == 0 {
            totals.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("max: {}", totals.iter().max().unwrap());
    totals.sort();
    totals.reverse();
    println!("max3: {}", totals.iter().take(3).sum::<i32>());
}
