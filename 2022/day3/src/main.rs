use std::{io::BufRead, collections::HashSet};

fn score(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
}

fn main() {
    let lines : Vec<String> = std::io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let charlines : Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

    {
        let mut total = 0;
        for charline in &charlines {
            let line_length = charline.len();
            let mut left : HashSet<char> = HashSet::new();
            let mut right : HashSet<char> = HashSet::new();
            for cl in charline.iter().take(line_length/2) {
                left.insert(*cl);
            }

            for cr in charline.iter().skip(line_length/2) {
                right.insert(*cr);
            }

            let mut isect = left.intersection(&right);
            let x = isect.next().unwrap();
            total += score(*x);
        }
        dbg!(total);
    }

    {
        let mut total = 0;
        // find the common character is each group of 3 lines
        for i in (0..charlines.len()).step_by(3) {
            let set1 : HashSet<char> = charlines[i+0].iter().cloned().collect();
            let set2 : HashSet<char> = charlines[i+1].iter().cloned().collect();
            let set3 : HashSet<char> = charlines[i+2].iter().cloned().collect();
            let mut isect = set1.intersection(&set2).cloned().collect::<HashSet<char>>();
            isect = isect.intersection(&set3).cloned().collect::<HashSet<char>>();
            let x = isect.iter().next().unwrap();
            total += score(*x);
        }

        dbg!(total);
    }
}
