use std::{io::BufRead};

#[derive(Debug, Clone)]
struct Node {
    value: i64,
    moved: bool,
    left: usize,
    right: usize,
}

fn main() {
    let numbers: Vec<i64> = std::io::stdin().lock().lines().map(|line| line.unwrap().parse().unwrap()).collect();

    let mut nodes = Vec::new();
    for (i,number) in numbers.iter().enumerate() {
        nodes.push(Node { value: *number, moved: false, left: (i+numbers.len()-1)%numbers.len(), right: (i+1)%numbers.len() });
    }

    let mut cursor = 0;
    let mut count = 0;
    while count < numbers.len() {
        let me = nodes[cursor].clone();

        if me.value == 0 {
            nodes[cursor].moved = true;
            count += 1;
            cursor = me.right;
            continue;
        }

        if me.moved {
            cursor = me.right;
            continue;
        }

        let mut to_move = me.value;
        let mut move_cursor = if to_move > 0 { me.right } else { to_move += 1; me.left };

        // unlink the current node
        nodes[me.left].right = me.right;
        nodes[me.right].left = me.left;

        while to_move > 0 {
            move_cursor = nodes[move_cursor].right;
            to_move -= 1;
        }

        while to_move < 0 {
            move_cursor = nodes[move_cursor].left;
            to_move += 1;
        }

        // I point right to the move cursor
        nodes[cursor].right = move_cursor;
        // I point left to where the move cursor pointed
        nodes[cursor].left = nodes[move_cursor].left;
        // The move cursor points left to me
        nodes[move_cursor].left = cursor;
        // The node to my left points right to me
        let left_idx = nodes[cursor].left;
        nodes[left_idx].right = cursor;

        nodes[cursor].moved = true;
        println!("{}: moves between {} and {}", me.value, nodes[nodes[cursor].left].value, nodes[nodes[cursor].right].value);

        cursor = me.right;
        count += 1;
    }

    // find zero
    let zero_idx;
    let mut cursor = 0;
    loop {
        if nodes[cursor].value == 0 {
            zero_idx = cursor;
            break;
        }

        cursor = nodes[cursor].left;
    }

    dbg!(zero_idx);

    let mut cursor = zero_idx;
    let mut total = 0;
    for tick in 0..numbers.len() {
        if tick == 1000 % numbers.len() {
            total += nodes[cursor].value;
        }
        if tick == 2000 % numbers.len() {
            total += nodes[cursor].value;
        }
        if tick == 3000 % numbers.len() {
            total += nodes[cursor].value;
        }
        cursor = nodes[cursor].right;
    }
    dbg!(total);
}

fn to_vec(nodes: &Vec<Node>) -> Vec<i64> {
    let mut cursor = 0;
    let mut count = 0;
    let mut result = Vec::new();
    while count < nodes.len() {
        result.push(nodes[cursor].value);
        cursor = nodes[cursor].right;
        count += 1;
    }
    result
}

