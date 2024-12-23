use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

use itertools::Itertools;

const NUM_ROBOTS: usize = 2;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct SearchState {
    output: Vec<char>,
    // (player ->) d1 -> d2 (--> numeric)
    dir_positions: [i8; NUM_ROBOTS],
    numeric_position: i8,
}

impl SearchState {
    fn valid(&self) -> bool {
        for dir_position in self.dir_positions {
            if dir_position < 0 || dir_position >= 6 || dir_position == 0 {
                return false;
            }
        }

        if self.numeric_position < 0 || self.numeric_position >= 12 || self.numeric_position == 9 {
            return false;
        }

        return true;
    }

    fn _press_numeric_button(&mut self) {
        match self.numeric_position {
            0 => self.output.push('7'),
            1 => self.output.push('8'),
            2 => self.output.push('9'),
            3 => self.output.push('4'),
            4 => self.output.push('5'),
            5 => self.output.push('6'),
            6 => self.output.push('1'),
            7 => self.output.push('2'),
            8 => self.output.push('3'),
            9 => panic!(),
            10 => self.output.push('0'),
            11 => self.output.push('A'),
            _ => panic!(),
        }
    }

    fn _press_direction_button(
        &mut self,
        idx: usize,
        button: char,
        ticks_since_A: &mut [u8; NUM_ROBOTS],
    ) -> bool {
        if idx == self.dir_positions.len() {
            match button {
                '^' => {
                    self.numeric_position -= 3;
                    true
                }
                'v' => {
                    self.numeric_position += 3;
                    true
                }
                '>' => {
                    self.numeric_position += 1;
                    (self.numeric_position - 1) % 3 != 2
                }
                '<' => {
                    self.numeric_position -= 1;
                    (self.numeric_position + 1) % 3 != 0
                }
                'A' => {
                    self._press_numeric_button();
                    true
                }
                _ => panic!(),
            }
        } else {
            match button {
                '^' => {
                    self.dir_positions[idx] -= 3;
                    ticks_since_A[idx] += 1;
                    true
                }
                'v' => {
                    self.dir_positions[idx] += 3;
                    ticks_since_A[idx] += 1;
                    true
                }
                '>' => {
                    self.dir_positions[idx] += 1;
                    ticks_since_A[idx] += 1;
                    (self.dir_positions[idx] - 1) % 3 != 2
                }
                '<' => {
                    self.dir_positions[idx] -= 1;
                    ticks_since_A[idx] += 1;
                    (self.dir_positions[idx] + 1) % 3 != 0
                }
                'A' => {
                    ticks_since_A[idx] = 0;
                    match self.dir_positions[idx] {
                        0 => panic!(),
                        1 => self._press_direction_button(idx + 1, '^', ticks_since_A),
                        2 => self._press_direction_button(idx + 1, 'A', ticks_since_A),
                        3 => self._press_direction_button(idx + 1, '<', ticks_since_A),
                        4 => self._press_direction_button(idx + 1, 'v', ticks_since_A),
                        5 => self._press_direction_button(idx + 1, '>', ticks_since_A),
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
    }

    fn player_press_button(
        &self,
        button: char,
        ticks_since_A: &mut [u8; NUM_ROBOTS],
    ) -> Option<Self> {
        let mut ret = self.clone();
        if !ret._press_direction_button(0, button, ticks_since_A) {
            return None;
        }
        if ret.valid() {
            Some(ret)
        } else {
            None
        }
    }
}

fn all_sensible_paths(a: i8, b: i8, gap: i8) -> Vec<String> {
    let bx = b % 3;
    let by = b / 3;

    let mut results: Vec<String> = Vec::new();

    let mut work: VecDeque<(i8, String)> = VecDeque::new();
    work.push_front((a, String::new()));

    while let Some((pos, path)) = work.pop_front() {
        if pos == gap {
            // on a gap
            continue;
        }

        if pos == b {
            let mut path = path.clone();
            path.extend(std::iter::once('A'));
            results.push(path);
        }

        let px = pos % 3;
        let py = pos / 3;

        if px < bx {
            let mut path = path.clone();
            path.extend(std::iter::once('>'));
            work.push_back((pos + 1, path));
        }

        if px > bx {
            let mut path = path.clone();
            path.extend(std::iter::once('<'));
            work.push_back((pos - 1, path));
        }

        if py < by {
            let mut path = path.clone();
            path.extend(std::iter::once('v'));
            work.push_back((pos + 3, path));
        }

        if py > by {
            let mut path = path.clone();
            path.extend(std::iter::once('^'));
            work.push_back((pos - 3, path));
        }
    }

    results
}

fn keypad_char_to_idx(c: char) -> i8 {
    match c {
        '7' => 0,
        '8' => 1,
        '9' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '1' => 6,
        '2' => 7,
        '3' => 8,
        '0' => 10,
        'A' => 11,
        _ => panic!(),
    }
}

fn dirpad_char_to_idx(c: char) -> i8 {
    match c {
        '^' => 1,
        'A' => 2,
        '<' => 3,
        'v' => 4,
        '>' => 5,
        _ => panic!(),
    }
}

fn min_presses(
    code: String,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(usize, String), usize>,
) -> usize {
    if let Some(cached) = cache.get(&(depth, code.clone())) {
        return *cached;
    }

    let ret = std::iter::once('A')
        .chain(code.chars())
        .tuple_windows()
        .map(|(a, b)| {
            let gap_at = if depth == 0 { 9 } else { 0 };
            let a = if depth == 0 {
                keypad_char_to_idx(a)
            } else {
                dirpad_char_to_idx(a)
            };
            let b = if depth == 0 {
                keypad_char_to_idx(b)
            } else {
                dirpad_char_to_idx(b)
            };
            let paths = all_sensible_paths(a, b, gap_at);

            if depth == max_depth {
                paths.iter().map(|x| x.len()).min().unwrap()
            } else {
                paths
                    .iter()
                    .map(|x| min_presses(x.clone(), depth + 1, max_depth, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum::<usize>();

    cache.insert((depth, code), ret);
    ret
}

fn main() {
    let initial_state = SearchState {
        output: Vec::new(),
        dir_positions: [2; NUM_ROBOTS],
        numeric_position: 11,
    };

    let mut score = 0;

    let inputs: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();

    for line in &inputs {
        let target: Vec<char> = line.chars().collect();
        let mut visited: HashSet<SearchState> = HashSet::new();
        let mut work: VecDeque<(SearchState, usize, [u8; NUM_ROBOTS], char, Vec<char>)> =
            VecDeque::new();
        work.push_front((initial_state.clone(), 0, [0; NUM_ROBOTS], ' ', Vec::new()));

        'outer: while let Some((state, buttons_pressed, ticks_since_A, prev_button, path)) =
            work.pop_front()
        {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());

            let output_len_before = state.output.len();

            for button in ['^', 'v', '<', '>', 'A'] {
                if button == '<' && prev_button == '>' {
                    continue;
                }
                if button == '>' && prev_button == '<' {
                    continue;
                }
                if button == '^' && prev_button == 'v' {
                    continue;
                }
                if button == 'v' && prev_button == '^' {
                    continue;
                }

                let mut ticks_since_A = ticks_since_A.clone();
                let mut path = path.clone();
                path.push(button);

                if let Some(state) = state.player_press_button(button, &mut ticks_since_A) {
                    let buttons_pressed = buttons_pressed + 1;
                    if button == 'A' {
                        if state.output.len() > output_len_before {
                            let interested = &target[..state.output.len()];
                            if state.output != interested {
                                continue;
                            }
                            work.clear();
                            visited.clear();

                            if state.output.len() == target.len() {
                                score += buttons_pressed
                                    * target
                                        .iter()
                                        .take(3)
                                        .collect::<String>()
                                        .parse::<usize>()
                                        .unwrap();
                                dbg!(buttons_pressed);
                                //for p in &path {
                                //    print!("{}", p);
                                //}
                                //println!();
                                //println!("#A: {}", path.iter().filter(|x| **x == 'A').count());
                                break 'outer;
                            }
                        }
                        work.push_back((state, buttons_pressed, ticks_since_A, button, path));
                    } else {
                        // not pressing A more than 3 times makes no sense
                        if ticks_since_A.iter().any(|x| *x > 3) {
                            continue;
                        }
                        work.push_back((state, buttons_pressed, ticks_since_A, button, path));
                    }
                }
            }
        }
    }

    println!("Part 1: {score}");

    let mut sum = 0;
    for line in inputs {
        let mut cache = HashMap::new();
        let answer = min_presses(line.clone(), 0, 25, &mut cache);
        sum += answer
            * line
                .chars()
                .take(3)
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
    }

    println!("Part 2: {sum}")
}
