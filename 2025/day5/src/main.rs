use std::io::BufRead;

use sscanf::scanf;

fn main() {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut ingredients: Vec<usize> = Vec::new();

    for line in std::io::stdin().lock().lines().map(|x| x.unwrap()) {
        if let Ok((lhs, rhs)) = scanf!(line, "{usize}-{usize}") {
            ranges.push((lhs, rhs));
        } else if let Ok(i) = scanf!(line, "{usize}") {
            ingredients.push(i);
        }
    }

    let mut sum = 0;
    for ingredient in ingredients.iter() {
        for (lhs, rhs) in ranges.iter() {
            if ingredient >= lhs && ingredient <= rhs {
                sum += 1;
                break;
            }
        }
    }

    println!("Part 1: {sum}");

    let mut sum = 0;
    let merged_ranges = merge_ranges(&ranges);

    for (lhs, rhs) in merged_ranges.iter() {
        sum += (rhs - lhs) + 1;
    }

    println!("Part 2: {sum}");
}

fn merge_ranges(ranges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_ranges: Vec<(usize, usize)> = Vec::new();
    for existing in ranges.iter() {
        let mut absorped = false;
        for seen in new_ranges.iter_mut() {
            // Check if ranges overlap or are adjacent
            if existing.0 <= seen.1 + 1 && existing.1 + 1 >= seen.0 {
                // Merge the ranges
                seen.0 = seen.0.min(existing.0);
                seen.1 = seen.1.max(existing.1);
                absorped = true;
                break;
            }
        }

        if !absorped {
            new_ranges.push(*existing);
        }
    }

    // run till fixpoint
    if new_ranges.len() < ranges.len() {
        return merge_ranges(&new_ranges);
    }
    new_ranges
}
