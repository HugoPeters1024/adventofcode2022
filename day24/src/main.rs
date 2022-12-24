use std::{io::BufRead, collections::{VecDeque, HashMap, BinaryHeap, HashSet}, hash::Hash, fmt::Binary};

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
struct State {
    player: (i32, i32),
    tick: i32,
    distance: i32,
}

impl State {
    fn update_distance(&mut self, tx: i32, ty: i32) {
        self.distance = (tx - self.player.0).abs() + (ty - self.player.1).abs();
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
            .then_with(|| other.tick.cmp(&self.tick))
            .then_with(|| self.player.cmp(&other.player))
    }
}

#[derive(Clone, Debug)]
struct Storm {
    dir: i32,
    start: i32,
}

fn main() {
    let lines : Vec<String> = std::io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let WIDTH : i32 = lines[0].len() as i32 - 2;
    let HEIGHT : i32 = lines.len() as i32 - 2;
    let CYCLE = num::integer::lcm(WIDTH, HEIGHT);
    dbg!(WIDTH, HEIGHT, CYCLE);

    let mut rows : Vec<Vec<Storm>> = vec![Vec::new(); HEIGHT as usize];
    let mut columns : Vec<Vec<Storm>> = vec![Vec::new(); WIDTH as usize];

    for (y, line) in lines.iter().skip(1).take(HEIGHT as usize).enumerate() {
        for (x, c) in line.chars().skip(1).take(WIDTH as usize).enumerate() {
            if c == '>' {
                rows[y].push(Storm { dir: 1, start: x as i32 });
            } else if c == '<' {
                rows[y].push(Storm { dir: -1, start: x as i32 });
            } else if c == 'v' {
                columns[x].push(Storm { dir: 1, start: y as i32 });
            } else if c == '^' {
                columns[x].push(Storm { dir: -1, start: y as i32 });
            }
        }
    }

    let start = (0, -1);
    let target = (WIDTH-1, HEIGHT);
    let mut work : BinaryHeap<State> = BinaryHeap::new();
    let start_tick = 529;
    let mut initial = State { player: start, tick: start_tick, distance: 0 };
    initial.update_distance(target.0, target.1);
    work.push(initial);

    let mut best = std::i32::MAX;
    let mut visisted : HashSet<State> = HashSet::new();

    while let Some(state) = work.pop() {
        if state.tick >= best {
            continue;
        }

        if visisted.contains(&state) {
            continue;
        }
        visisted.insert(state.clone());

        if state.player == target {
            best = best.min(state.tick);
            println!("FINISHED IN {}", state.tick);
            continue;
        }

        let mut children: [State; 5] = [
            State { player: (state.player.0, state.player.1),     tick: state.tick + 1, distance: 0 },
            State { player: (state.player.0, state.player.1 + 1), tick: state.tick + 1, distance: 0 },
            State { player: (state.player.0, state.player.1 - 1), tick: state.tick + 1, distance: 0 },
            State { player: (state.player.0 + 1, state.player.1), tick: state.tick + 1, distance: 0 },
            State { player: (state.player.0 - 1, state.player.1), tick: state.tick + 1, distance: 0 },
        ];

        for child in children.iter_mut() {
            child.update_distance(target.0, target.1);
        }

        'child_loop: for child in children {
            if (child.player.0 < 0 || child.player.0 >= WIDTH || child.player.1 < 0 || child.player.1 >= HEIGHT) 
            && child.player != target 
            && child.player != start {
                continue 'child_loop;
            }

            // exclude storm collisions
            if child.player.1 >= 0 && child.player.1 < HEIGHT {
                for storm in &rows[child.player.1 as usize] {
                    if child.player.0 == (storm.start + storm.dir * child.tick).rem_euclid(WIDTH) {
                        continue 'child_loop;
                    }
                }
            }

            for storm in &columns[child.player.0 as usize] {
                if child.player.1 == (storm.start + storm.dir * child.tick).rem_euclid(HEIGHT) {
                    continue 'child_loop;
                }
            }

            work.push(child);
        }
    }
}

fn print_state(state: &State, WIDTH: i32, HEIGHT: i32, rows: &Vec<Vec<Storm>>, columns: &Vec<Vec<Storm>>) {
    for y in -1..1+HEIGHT {
        'outer: for x in -1..1+WIDTH {
            if x == state.player.0 && y == state.player.1 {
                print!("P");
                continue;
            }
            if x == 0 && y == -1 {
                print!(".");
                continue;
            }

            if x == WIDTH-1 && y == HEIGHT {
                print!(".");
                continue;
            }

            if x < 0 || y < 0 || x >= WIDTH || y >= HEIGHT {
                print!("#");
                continue;
            }

            for storm in &rows[y as usize] {
                if (storm.start + storm.dir*state.tick).rem_euclid(WIDTH) == x {
                    if storm.dir == 1 {
                        print!(">");
                    } else {
                        print!("<");
                    }
                    continue 'outer;
                }
            }

            for storm in &columns[x as usize] {
                if (storm.start + storm.dir*state.tick).rem_euclid(HEIGHT) == y {
                    if storm.dir == 1 {
                        print!("v");
                    } else {
                        print!("^");
                    }
                    continue 'outer;
                }
            }



            print!(".");
        }
        println!("");
    }
}
