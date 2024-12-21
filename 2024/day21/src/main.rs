use std::{collections::{HashSet, VecDeque}, io::BufRead};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct SearchState {
    output: Vec<char>,
    buttons_pressed: usize,
    // (player ->) d1 -> d2 (--> numeric)
    dir_positions: [isize; 2],
    numeric_position: isize,
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

    fn _press_direction_button(&mut self, idx: usize, button: char) -> bool {
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
                    true
                }
                'v' => {
                    self.dir_positions[idx] += 3;
                    true
                }
                '>' => {
                    self.dir_positions[idx] += 1;
                    (self.dir_positions[idx] - 1) % 3 != 2
                }
                '<' => {
                    self.dir_positions[idx] -= 1;
                    (self.dir_positions[idx] + 1) % 3 != 0
                }
                'A' => match self.dir_positions[idx] {
                    0 => panic!(),
                    1 => self._press_direction_button(idx + 1, '^'),
                    2 => self._press_direction_button(idx + 1, 'A'),
                    3 => self._press_direction_button(idx + 1, '<'),
                    4 => self._press_direction_button(idx + 1, 'v'),
                    5 => self._press_direction_button(idx + 1, '>'),
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }
    }

    fn player_press_button(&self, button: char) -> Option<Self> {
        let mut ret = self.clone();
        if !ret._press_direction_button(0, button) {
            return None;
        }
        ret.buttons_pressed += 1;
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
        buttons_pressed: 0,
        dir_positions: [2; 2],
        numeric_position: 11,
    };

    let mut score = 0;

    for line in std::io::stdin().lock().lines() {
        let target: Vec<char> = line.unwrap().chars().collect();
        let mut visited: HashSet<SearchState> = HashSet::new();
        let mut work: VecDeque<(SearchState, usize)> = VecDeque::new();
        work.push_front((initial_state.clone(), 0));

        'outer: while let Some((state, ticks_since_A)) = work.pop_front() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());

            for button in ['^', 'v', '<', '>', 'A'] {
                if let Some(state) = state.player_press_button(button) {
                    if button == 'A' {
                        let interested = &target[..state.output.len()];
                        if state.output != interested {
                            continue;
                        }
                        if state.output.len() == target.len() {
                            score += state.buttons_pressed * (target.iter().take(3).collect::<String>()).parse::<usize>().unwrap();
                            dbg!(state.buttons_pressed);
                            break 'outer;
                        }
                        work.push_back((state, 0));
                    } else {
                        // not pressing A more than 3 times makes no sense
                        if ticks_since_A < 3 {
                            work.push_back((state, ticks_since_A + 1));
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {score}");
}
