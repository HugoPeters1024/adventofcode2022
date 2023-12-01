use std::{io::BufRead, collections::{VecDeque}};
use scanf::sscanf;


#[derive(Clone, Debug)]
struct Ctx {
    dir: String,
    files: Vec<(String, usize)>,
    parent: Option<usize>,
    children: Vec<usize>,
}


fn main() {
    let lines : Vec<String> = std::io::stdin().lock().lines().map(|x| x.unwrap()).collect();

    let mut all_ctxs = vec![Ctx {
        dir: "/".to_string(),
        files: Vec::new(),
        parent: None,
        children: Vec::new(),
    }];

    let mut ctx : usize = 0;

    let mut idx = 0;
    while idx < lines.len() {
        let mut arg = "".to_string();
        if let Ok(_) = sscanf!(&lines[idx], "$ cd {}", arg) {
            if arg == "/" {
                ctx = 0;
            } else if arg == ".." {
                if let Some(parent) = all_ctxs[ctx].parent {
                    ctx = parent;
                }
            } else {
                let child = Ctx {
                    dir: arg,
                    files: Vec::new(),
                    parent: Some(ctx),
                    children: Vec::new(),
                };

                all_ctxs.push(child);
                let child_idx = all_ctxs.len()-1;
                all_ctxs[ctx].children.push(child_idx);
                ctx = child_idx;
            }
        } else if lines[idx] == "$ ls" {
            while idx < lines.len()-1 {
                idx += 1;
                let mut fname = "".to_string();
                let mut fsize = 0;
                if let Ok(_) = sscanf!(&lines[idx], "{} {}", fsize, fname) {
                    all_ctxs[ctx].files.push((fname.to_string(), fsize));
                } else if let Ok(_) = sscanf!(&lines[idx], "dir {}", fname) {

                } else {
                    idx-=1;
                    break;
                }
            }
        }

        idx += 1;
    }

    // part 1
    let mut total = 0;
    for i in 0..all_ctxs.len() {
        let size = ctx_size(&all_ctxs, i);

        println!("size of {} = {}", all_ctxs[i].dir, size);

        if size <= 100000 {
            total += size;
        }
    }
    dbg!(total);

    // part 2
    let total_size = ctx_size(&all_ctxs, 0);
    let free_space = 70000000 - total_size;
    let needed = 30000000 - free_space;

    let mut all_sizes = Vec::new();
    for i in 0..all_ctxs.len() {
        let size = ctx_size(&all_ctxs, i);
        all_sizes.push(size);
    }

    let best_dir = all_sizes.iter().filter(|x| **x >= needed).min();
    dbg!(best_dir);
}

fn ctx_size(all_ctxs: &Vec<Ctx>, ctx: usize) -> usize {
    let mut total = 0;
    let mut work: VecDeque<usize> = VecDeque::new();
    work.push_front(ctx);
    while let Some(ctx) = work.pop_front() {
        for (_,s) in all_ctxs[ctx].clone().files {
            total += s;
        }

        for child in all_ctxs[ctx].clone().children {
            work.push_front(child);
        }
    }
    total
}
