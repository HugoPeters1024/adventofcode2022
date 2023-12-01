use std::io::BufRead;

fn main() {
    let input: Vec<String> = std::io::stdin().lock().lines().map(|l| l.unwrap().chars().collect()).collect();

    let mut sum = 0;
    for line in &input {
        let digits : Vec<u32> = line.chars().map(|c| c.to_digit(10)).filter(|c| c.is_some()).map(|c| c.unwrap()).collect();
        sum += 10 * digits[0] + digits[digits.len()-1]
    }

    println!("{}", sum);

    // part 2
    let word_to_val = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ];

    sum = 0;
    for line in &input {
        'outer: for offset in 0..line.len() {
            if let Some(v) = line.chars().collect::<Vec<char>>()[offset].to_digit(10) {
                sum += 10 * v;
                break;
            }

            for (word, val) in &word_to_val {
                if line.chars().skip(offset).collect::<String>().starts_with(word) {
                    sum += 10 * val;
                    break 'outer;
                }
            }
        }

        'outer: for offset in 0..line.len() {
            if let Some(v) = line.chars().rev().collect::<Vec<char>>()[offset].to_digit(10) {
                sum += v;
                break;
            }

            for (word, val) in &word_to_val {
                let rev_word = word.to_string().chars().rev().collect::<String>();
                let rev_line = line.chars().rev().skip(offset).collect::<String>();

                if rev_line.starts_with(&rev_word) {
                    sum += val;
                    break 'outer;
                }
            }
        }
    }

    println!("{}", sum);
}
