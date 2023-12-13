use std::io::BufRead;

fn main() {
    let mut inputs: Vec<Vec<Vec<char>>> = Vec::new();

    let mut current_input: Vec<Vec<char>> = Vec::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            inputs.push(current_input);
            current_input = Vec::new();
            continue;
        }

        current_input.push(line.chars().collect());
    }
    inputs.push(current_input);

    dbg!(&inputs);
}
