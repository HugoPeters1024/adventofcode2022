use std::{collections::HashMap, io::BufRead, sync::Mutex};

#[derive(Clone, Debug)]
struct InputLine {
    map: Vec<char>,
    broken_info: Vec<usize>,
}

impl InputLine {
    fn is_consistent(&self) -> bool {
        let mut broken_stream = self.broken_info.iter();
        let mut current_streak = 0;
        for i in 0..self.map.len() {
            if self.map[i] == '.' {
                if current_streak > 0 {
                    if let Some(&constraint) = broken_stream.next() {
                        if current_streak != constraint {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                current_streak = 0;
            } else if self.map[i] == '#' {
                current_streak += 1;
            }
        }

        if current_streak > 0 {
            if let Some(&constraint) = broken_stream.next() {
                if current_streak != constraint {
                    return false;
                }
            } else {
                return false;
            }
        }

        broken_stream.next().is_none()
    }

    fn to_string(&self, needle: usize, streak_needle: usize) -> String {
        let mut result = String::new();
        for i in 0..self.map.len() {
            result.push(self.map[i]);
            if i == needle {
                result.push('<');
            }
        }
        result.push(' ');
        for i in streak_needle..self.broken_info.len() {
            result.push_str(&self.broken_info[i].to_string());
            result.push(',');
        }
        result
    }
}

fn count_configs(input: &mut InputLine, needle: usize) -> usize {
    if needle == input.map.len() {
        if input.is_consistent() {
            return 1;
        } else {
            return 0;
        }
    }

    if input.map[needle] == '?' {
        input.map[needle] = '.';
        let mut count = count_configs(input, needle + 1);
        input.map[needle] = '#';
        count += count_configs(input, needle + 1);
        input.map[needle] = '?';
        return count;
    } else {
        return count_configs(input, needle + 1);
    }
}

fn count_configs2(
    input: &mut InputLine,
    cache: &mut HashMap<(usize, Vec<char>, Vec<usize>), usize>,
    needle: usize,
    streak: usize,
    streak_needle: usize,
) -> usize {
    //println!("update: {}: {} {} {}", input.to_string(needle, streak_needle), needle, streak, streak_needle);
    if needle == input.map.len() {
        if streak == 0 {
            // Made it to the end without a streak
            if streak_needle == input.broken_info.len() {
                //println!("consistent: {}", input.to_string(needle, streak_needle));
                return 1;
            } else {
                //println!("constraints left: {}", input.to_string(needle, streak_needle));
                return 0;
            }
        } else {
            // Made it to the end with a streak in progress, we need to check if it's satisfied
            if streak_needle + 1 == input.broken_info.len() {
                // We have the last constraint
                if streak == input.broken_info[streak_needle] {
                    //println!("consistent: {}", input.to_string(needle, streak_needle));
                    return 1;
                } else {
                    //println!("final streak mismatch: {}", input.to_string(needle, streak_needle));
                    return 0;
                }
            } else {
                //println!("missing last constraints: {}", input.to_string(needle, streak_needle));
                return 0;
            }
        }
    }

    if input.map[needle] == '?' {
        let map_key = input.map.iter().cloned().skip(needle).collect();
        let broken_key = input
            .broken_info
            .iter()
            .cloned()
            .skip(streak_needle)
            .collect();
        let streak_key = streak;
        let key = (streak_key, map_key, broken_key);
        if let Some(res) = cache.get(&key) {
            return *res;
        }

        input.map[needle] = '#';
        let mut count = count_configs2(input, cache, needle, streak, streak_needle);

        input.map[needle] = '.';
        count += count_configs2(input, cache, needle, streak, streak_needle);
        input.map[needle] = '?';

        cache.insert(key, count);
        return count;
    }

    if input.map[needle] == '#' {
        // Optimization: stop early if are about to start a streak but there are no constraints left
        if streak_needle >= input.broken_info.len() {
            //println!("already out of constraints: {}", input.to_string(needle, streak_needle));
            return 0;
        }

        // Optimization: stop early if we are about violate a constraint
        if streak == input.broken_info[streak_needle] {
            //println!("streak getting to long: {}", input.to_string(needle, streak_needle));
            return 0;
        }

        // Continue streak
        return count_configs2(input, cache, needle + 1, streak + 1, streak_needle);
    }

    if input.map[needle] == '.' {
        if streak > 0 {
            // Streak was in progress, we need to check it
            if streak_needle >= input.broken_info.len() {
                //println!("no more constraints: {}", input.to_string(needle, streak_needle));
                return 0;
            }
            if streak == input.broken_info[streak_needle] {
                //println!("streak accepted: {}", input.to_string(needle, streak_needle));
                return count_configs2(input, cache, needle + 1, 0, streak_needle + 1);
            } else {
                //println!("streak mismatch: {}", input.to_string(needle, streak_needle));
                return 0;
            }
        }

        // No streak in progress, just continue
        return count_configs2(input, cache, needle + 1, 0, streak_needle);
    }

    panic!("Invalid input");
}

fn main() {
    let input: Vec<InputLine> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split(' ');
            let map = parts.next().unwrap().chars().collect();
            let broken_info = parts
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            InputLine { map, broken_info }
        })
        .collect();

    let mut count = 0;
    for line in &input {
        let mut line = line.clone();
        let configs = count_configs(&mut line, 0);
        count += configs;
    }

    println!("Part 1 {}", count);

    let input_len = input.len();
    let mut input = input;

    for line in &mut input {
        let mut new_map: Vec<char> = Vec::new();
        for i in 0..5 {
            new_map.extend_from_slice(&line.map);
            if i < 4 {
                new_map.push('?');
            }
        }
        line.map = new_map;
        line.broken_info = line.broken_info.repeat(5);
    }

    let mut cache = HashMap::new();

    let total: Mutex<usize> = Mutex::new(0);
    let work_idx: Mutex<usize> = Mutex::new(0);

    let input = input;

    for line in input {
        let mut line = line.clone();
        let idx = {
            let mut c = work_idx.lock().unwrap();
            let idx = *c;
            *c += 1;
            idx
        };
        println!("{}/{}", idx, input_len);
        let configs = count_configs2(&mut line, &mut cache, 0, 0, 0);

        {
            let mut t = total.lock().unwrap();
            *t += configs;
            drop(t);
        }
    }

    println!("Part 2 {}", total.lock().unwrap());
}
