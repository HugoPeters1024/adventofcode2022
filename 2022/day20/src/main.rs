use std::io::BufRead;

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
        nodes.push(Node { value: number.clone() * 811589153, moved: false, left: (i+numbers.len()-1)%numbers.len(), right: (i+1)%numbers.len() });
    }

    // keep track of the nodes we visit in the first round to reuse in the later rounds
    let mut cursors = Vec::new();
    let mut cursor = 0;
    while cursors.len() < numbers.len() {
        let me = nodes[cursor].clone();

        if me.moved {
            cursor = me.right;
            continue;
        }

        cursors.push(cursor);

        let mut to_move = me.value.rem_euclid((numbers.len() -1) as i64);

        let mut move_cursor = me.right;

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
    }

    // 9 more rounds
    for _ in 0..9 {
        for cursor in cursors.iter().cloned() {
            let me = nodes[cursor].clone();

            let mut to_move = me.value.rem_euclid((numbers.len() -1) as i64);

            let mut move_cursor = me.right;

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

            println!("{}: moves between {} and {}", me.value, nodes[nodes[cursor].left].value, nodes[nodes[cursor].right].value);
        }
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
