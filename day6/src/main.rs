use std::{io::BufRead, collections::HashSet};

fn main() {
    let chars : Vec<char> = std::io::stdin().lock().lines().next().unwrap().unwrap().chars().collect();

    // part 1
    for i in 0..chars.len()-3 {
        let set : HashSet<char> = HashSet::from_iter(chars.iter().skip(i).take(4).cloned());
        if set.len() == 4 {
            println!("{}", i+4);
            break;
        }
    }

    // part 2
    for i in 0..chars.len()-13 {
        let set : HashSet<char> = HashSet::from_iter(chars.iter().skip(i).take(14).cloned());
        if set.len() == 14 {
            println!("{}", i+14);
            break;
        }
    }
}
