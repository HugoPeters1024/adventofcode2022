use std::hash::Hash;
use std::{io::BufRead, collections::HashMap};
use nom::*;
use nom::combinator::{map, map_res};
use nom::multi::many0;
use nom::character::complete::{digit1,char};
use nom::branch::alt;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug, Clone)]
enum Instr {
    Move(u32),
    TurnClockwise,
    TurnCounterClockwise,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Dir {
    Up, Down, Left, Right
}

impl Dir {
    fn clockwise(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn counter_clockwise(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }
}



fn main() {
    let mut grid : HashMap<(i32, i32), Tile> = HashMap::new();
    let mut visisted : HashMap<(i32,i32),Dir> = HashMap::new();
    let mut y = 1;

    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let mut maxx = 0;
    while y < lines.len()-1 {
        let line = &lines[y-1];
        let mut x = 1;
        for c in line.chars() {
            if c == '#' {
                grid.insert((x, y as i32), Tile::Wall);
            }

            if c == '.' {
                grid.insert((x, y as i32), Tile::Empty);
            }

            x += 1;
            maxx = maxx.max(x);
        }
        y+=1;
    }

    let maxx = maxx as i32;
    let maxy = (y-1) as i32;

    let xstart = {
        let mut x = 1;
        while !grid.contains_key(&(x, 1)) {
            x += 1;
        }
        x
    };

    let (_, instrs) = parse_instrs(&lines[lines.len()-1]).unwrap();
    let mut px = xstart;
    let mut py = 1;
    let mut pdir = Dir::Right;


    for instr in instrs {
        match instr {
            Instr::Move(n) => {
                for _ in 0..n {
                    let (mut nx, mut ny) = match pdir {
                        Dir::Up => (px, py-1),
                        Dir::Down => (px, py+1),
                        Dir::Left => (px-1, py),
                        Dir::Right => (px+1, py),
                    };

                    // wrap around
                    if !grid.contains_key(&(nx, ny)) {
                        match pdir {
                            Dir::Up => ny = maxy,
                            Dir::Down => ny = 1,
                            Dir::Left => nx = maxx,
                            Dir::Right => nx = 1,
                        }

                        while !grid.contains_key(&(nx, ny)) {
                            match pdir {
                                Dir::Up => ny -= 1,
                                Dir::Down => ny += 1,
                                Dir::Left => nx -= 1,
                                Dir::Right => nx += 1,
                            }
                        }
                    }

                    match grid.get(&(nx, ny)).unwrap() {
                        Tile::Empty => {
                            px = nx;
                            py = ny;
                        },
                        Tile::Wall => {
                            break;
                        }
                    }
                }
            },
            Instr::TurnClockwise => pdir = pdir.clockwise(),
            Instr::TurnCounterClockwise => pdir = pdir.counter_clockwise(),
        }
    }

    let heading_value = match pdir {
        Dir::Up => 3,
        Dir::Down => 1,
        Dir::Left => 2,
        Dir::Right => 0,
    };
    println!("password {}", 1000 * py + 4 * px + heading_value);

    // Part 2

    let mut wrapping_rules = HashMap::new();
    for y in 1..51 {
        // 1 -> 4
        wrapping_rules.insert((50, y, Dir::Left), (1, 151 - y, Dir::Right));
        // 4 -> 1
        wrapping_rules.insert((0, 151 - y, Dir::Left), (51, y, Dir::Right));

        // 2 -> 5
        wrapping_rules.insert((151, y, Dir::Right), (100, 151 - y, Dir::Left));
        // 5 -> 2
        wrapping_rules.insert((101, 151 - y, Dir::Right), (150, y, Dir::Left));
    }
    for x in 51..101 {
        // 1 -> 6
        wrapping_rules.insert((x, 0, Dir::Up), (1, 100 + x, Dir::Right));
        // 6 -> 1
        wrapping_rules.insert((0, 100 + x, Dir::Left), (x, 1, Dir::Down));

        // 5 -> 6
        wrapping_rules.insert((x, 151, Dir::Down), (50, 100 + x, Dir::Left));
        // 6 -> 5
        wrapping_rules.insert((51, 100 + x, Dir::Right), (x, 150, Dir::Up));
    }
    for x in 101..151 {
        // 2 -> 3
        wrapping_rules.insert((x, 51, Dir::Down), (100, x - 50, Dir::Left));
        // 3 -> 2
        wrapping_rules.insert((101, x - 50, Dir::Right), (x, 50, Dir::Up));

        // 2 -> 6
        wrapping_rules.insert((x, 0, Dir::Up), (x - 100, 200, Dir::Up));
        // 6 -> 2
        wrapping_rules.insert((x - 100, 201, Dir::Down), (x, 1, Dir::Down));
    }
    for y in 51..101 {
        // 3 -> 4
        wrapping_rules.insert((50, y, Dir::Left), (y - 50, 101, Dir::Down));
        // 4 -> 3
        wrapping_rules.insert((y - 50, 100, Dir::Up), (51, y, Dir::Right));
    }
}

fn parse_instrs(input: &str) -> IResult<&str, Vec<Instr>> {
    many0(parse_inst)(input)
}

fn parse_inst(input: &str) -> IResult<&str, Instr> {
    alt((
        map_res(digit1, |s: &str| s.parse::<u32>().map(Instr::Move)),
        map(char('L'), |_| Instr::TurnCounterClockwise),
        map(char('R'), |_| Instr::TurnClockwise),
    ))(input)
}
