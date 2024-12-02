use std::io::BufRead;

fn is_safe(nums: &[i64]) -> bool {
    let mut deltas = Vec::new();
    for i in 0..nums.len() - 1 {
        deltas.push(nums[i + 1] - nums[i])
    }

    let strictly_decreasing = deltas.iter().all(|x| x.is_positive());
    let strictly_increasing = deltas.iter().all(|x| x.is_negative());
    let not_to_steep = deltas.iter().all(|x| x.abs() >= 1 && x.abs() <= 3);

    return not_to_steep && (strictly_decreasing || strictly_increasing);
}

fn is_safe_lenient(nums: &[i64]) -> bool {
    for mut_index in 0..nums.len() {
        let mut mutated: Vec<i64> = nums.iter().cloned().collect();
        mutated.remove(mut_index);

        if is_safe(&mutated) {
            return true;
        }
    }

    return false;
}

fn main() {
    let mut safe_count = 0;
    let mut safe_lenient_count = 0;
    for line in std::io::stdin().lock().lines() {
        let nums: Vec<i64> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        if is_safe(&nums) {
            safe_count += 1;
        }

        if is_safe_lenient(&nums) {
            safe_lenient_count += 1;
        }
    }

    println!("safe count: {safe_count}");
    println!("safe lenient count: {safe_lenient_count}");
}
