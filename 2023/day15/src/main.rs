use scanf::sscanf;
use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    hash::Hasher,
    io::BufRead,
};

fn label_to_box_nr(label: &str) -> usize {
    let mut box_nr = 0;
    for c in label.to_string().chars() {
        box_nr = (17 * (box_nr + c as usize)) & 255;
    }
    box_nr
}

fn mk_label(label: &str) -> Label {
    let unique = {
        let mut hasher = DefaultHasher::new();
        hasher.write(label.as_bytes());
        hasher.finish() as usize
    };

    Label {
        unique,
        box_nr: label_to_box_nr(label),
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Label {
    unique: usize,
    box_nr: usize,
}

type Lens = usize;

#[derive(Debug)]
enum Instr {
    Remove(Label),
    Insert(Label, Lens),
}

fn main() {
    let mut instructions: Vec<Instr> = Vec::new();
    let mut sum = 0;
    for piece in std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
    {
        sum += label_to_box_nr(piece);

        let mut label = "".to_string();
        let mut lens = 0;
        if sscanf!(piece, "{}={}", label, lens).is_ok() {
            instructions.push(Instr::Insert(mk_label(&label), lens));
        } else if sscanf!(piece, "{}-", label).is_ok() {
            instructions.push(Instr::Remove(mk_label(&label)));
        } else {
            panic!("Unknown instruction: {}", piece);
        }
    }

    println!("Part 1: {}", sum);

    let mut boxes: Vec<Vec<(Label, usize)>> = vec![Vec::new(); 256];
    for instr in &instructions {
        match instr {
            Instr::Remove(label) => {
                if let Some(find) = boxes[label.box_nr]
                    .iter()
                    .position(|(l, _)| l.unique == label.unique)
                {
                    boxes[label.box_nr].remove(find);
                }
            }
            Instr::Insert(label, lens) => {
                if let Some(find) = boxes[label.box_nr]
                    .iter()
                    .position(|(l, _)| l.unique == label.unique)
                {
                    boxes[label.box_nr][find].1 = *lens;
                } else {
                    boxes[label.box_nr].push((label.clone(), *lens));
                }
            }
        }
    }

    let mut sum = 0;
    for (box_nr, box_) in boxes.iter().enumerate() {
        for (lens_idx, (_, lens)) in box_.iter().enumerate() {
            let d = (box_nr + 1) * (lens_idx + 1) * lens;
            sum += d;
        }
    }

    println!("Part 2: {}", sum);
}
