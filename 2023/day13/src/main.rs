use std::io::BufRead;

use either::Either;

fn find_symmetry(input: &Vec<usize>, ignore: Option<usize>) -> Option<usize> {
    for i in 0..input.len() - 1 {
        let mut lhs = i;
        let mut rhs = i + 1;
        let mut flag = true;
        loop {
            if input[lhs] == input[rhs] {
                if lhs == 0 || rhs == input.len() - 1 {
                    break;
                }
                lhs -= 1;
                rhs += 1;
            } else {
                flag = false;
                break;
            }
        }

        if flag && Some(i) != ignore {
            return Some(i);
        }
    }

    None
}

fn get_column_hashes(input: &Vec<Vec<char>>) -> Vec<usize> {
    let width = input[0].len();
    let height = input.len();

    (0..width)
        .map(|column| {
            (0..height)
                .map(|row| if input[row][column] == '#' { 1 } else { 0 })
                .rev()
                .reduce(|a, b| (a << 1) | b)
                .unwrap()
        })
        .collect()
}

fn get_row_hashes(input: &Vec<Vec<char>>) -> Vec<usize> {
    let width = input[0].len();
    let height = input.len();

    (0..height)
        .map(|row| {
            (0..width)
                .map(|column| if input[row][column] == '#' { 1 } else { 0 })
                .rev()
                .reduce(|a, b| (a << 1) | b)
                .unwrap()
        })
        .collect()
}

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
    let inputs = inputs;

    let mut sum = 0;

    let mut reflection_lines: Vec<Either<usize, usize>> = Vec::new();

    for input in &inputs {
        let column_hashes = get_column_hashes(input);

        if let Some(column_sym) = find_symmetry(&column_hashes, None) {
            sum += column_sym + 1;
            reflection_lines.push(Either::Left(column_sym));
            continue;
        }

        let row_hashes = get_row_hashes(input);

        if let Some(row_sym) = find_symmetry(&row_hashes, None) {
            sum += (row_sym + 1) * 100;
            reflection_lines.push(Either::Right(row_sym));
        }
    }

    println!("Part 1: {}", sum);

    let mut sum = 0;
    for (i, input) in inputs.iter().enumerate() {
        let width = input[0].len();
        let height = input.len();
        let mut column_hashes = get_column_hashes(input);
        let mut row_hashes = get_row_hashes(input);

        let ignore_column_sym = if let Either::Left(known_column_sym) = reflection_lines[i] {
            Some(known_column_sym)
        } else {
            None
        };
        let ignore_row_sym = if let Either::Right(known_row_sym) = reflection_lines[i] {
            Some(known_row_sym)
        } else {
            None
        };

        'outer: for muty in 0..height {
            for mutx in 0..width {
                // flip the corresponding bit in the hashes
                row_hashes[muty] ^= 1 << mutx;
                column_hashes[mutx] ^= 1 << muty;

                if let Some(column_sym) = find_symmetry(&column_hashes, ignore_column_sym) {
                    sum += column_sym + 1;
                    break 'outer;
                }

                if let Some(row_sym) = find_symmetry(&row_hashes, ignore_row_sym) {
                    sum += (row_sym + 1) * 100;
                    break 'outer;
                }

                // flip the bits back
                row_hashes[muty] ^= 1 << mutx;
                column_hashes[mutx] ^= 1 << muty;
            }
        }
    }

    println!("Part 2: {}", sum);
}
