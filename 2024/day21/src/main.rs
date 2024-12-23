use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

const NUM_ROBOTS: usize = 4;

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

fn main() {
    let initial_state = SearchState {
        output: Vec::new(),
        dir_positions: [2; NUM_ROBOTS],
        numeric_position: 11,
    };

    let mut score = 0;

    for line in std::io::stdin().lock().lines().take(1) {
        let target: Vec<char> = line.unwrap().chars().take(1).collect();
        let mut visited: HashSet<SearchState> = HashSet::new();
        let mut work: VecDeque<(SearchState, usize, [u8; NUM_ROBOTS], char, Vec<char>)> = VecDeque::new();
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
                                for p in &path {
                                    print!("{}", p);
                                }
                                println!();
                                println!("#A: {}", path.iter().filter(|x| **x == 'A').count());
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

}
