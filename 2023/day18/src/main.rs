use scanf::sscanf;
use std::io::BufRead;

struct Instr {
    dir: char,
    dist: i64,
    color: String,
}

fn area(vertices: &Vec<(i64, i64)>) -> i64 {
    let mut area = 0;
    let mut perimeter = 0;
    for i in 0..vertices.len() {
        area += vertices[i].0 * vertices[(i + 1) % vertices.len()].1;
        area -= vertices[i].1 * vertices[(i + 1) % vertices.len()].0;

        perimeter += (vertices[i].0 - vertices[(i + 1) % vertices.len()].0).abs();
        perimeter += (vertices[i].1 - vertices[(i + 1) % vertices.len()].1).abs();
    }

    area.abs() / 2 + perimeter / 2 + 1
}

fn main() {
    let input: Vec<Instr> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut dir = ' ';
            let mut dist = 0;
            let mut color = String::new();
            sscanf!(&line, "{} {} ({})", dir, dist, color).unwrap();
            Instr { dir, dist, color }
        })
        .collect();

    let mut vertices: Vec<(i64, i64)> = Vec::new();

    let mut x = 0;
    let mut y = 0;
    for instr in &input {
        vertices.push((x, y));
        match instr.dir {
            'U' => y -= instr.dist,
            'D' => y += instr.dist,
            'L' => x -= instr.dist,
            'R' => x += instr.dist,
            _ => panic!("Unknown direction"),
        }
    }

    println!("Part 1: {}", area(&vertices));

    let mut vertices: Vec<(i64, i64)> = Vec::new();

    let mut x = 0;
    let mut y = 0;

    for instr in &input {
        vertices.push((x, y));

        // parse color as hex num
        let hexnum: usize = usize::from_str_radix(&instr.color[1..], 16).unwrap();

        let steps = hexnum / 16;
        let dir = hexnum % 16;

        match dir {
            0 => x += steps as i64,
            1 => y += steps as i64,
            2 => x -= steps as i64,
            3 => y -= steps as i64,
            _ => panic!("Unknown direction"),
        }
    }

    println!("Part 2: {}", area(&vertices));
}
