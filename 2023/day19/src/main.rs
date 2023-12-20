use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

use scanf::sscanf;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Condition {
    Condition {
        var: char,
        op: char,
        val: isize,
        next: String,
    },
    Always {
        next: String,
    },
}

#[derive(Debug)]
struct Item {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

#[derive(Debug, Clone)]
struct ItemRange {
    x: (isize, isize),
    m: (isize, isize),
    a: (isize, isize),
    s: (isize, isize),
}

impl ItemRange {
    fn get_ptr<'a>(&'a self, var: char) -> &'a (isize, isize) {
        match var {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => panic!("unknown var"),
        }
    }

    fn process_smaller_than(&self, var: char, v: isize) -> Option<ItemRange> {
        let (min, max) = self.get_ptr(var);

        // Contradiction, no solution
        if v < *min {
            return None;
        }

        let newmax = std::cmp::min(*max, v-1);

        return Some(ItemRange {
            x: if var == 'x' {
                (self.x.0, newmax)
            } else {
                self.x.clone()
            },
            m: if var == 'm' {
                (self.m.0, newmax)
            } else {
                self.m.clone()
            },
            a: if var == 'a' {
                (self.a.0, newmax)
            } else {
                self.a.clone()
            },
            s: if var == 's' {
                (self.s.0, newmax)
            } else {
                self.s.clone()
            },
        });
    }

    fn process_larger_than(&self, var: char, v: isize) -> Option<ItemRange> {
        let (min, max) = self.get_ptr(var);

        // Contradiction, no solution
        if v > *max {
            return None;
        }

        let newmin = std::cmp::max(*min, v+1);

        return Some(ItemRange {
            x: if var == 'x' {
                (newmin, self.x.1)
            } else {
                self.x.clone()
            },
            m: if var == 'm' {
                (newmin, self.m.1)
            } else {
                self.m.clone()
            },
            a: if var == 'a' {
                (newmin, self.a.1)
            } else {
                self.a.clone()
            },
            s: if var == 's' {
                (newmin, self.s.1)
            } else {
                self.s.clone()
            },
        });
    }

    fn process_equal(&self, var: char, v: isize) -> Option<ItemRange> {
        let (min, max) = self.get_ptr(var);

        // Contradiction, no solution
        if v < *min || v > *max {
            return None;
        }

        return Some(ItemRange {
            x: if var == 'x' { (v, v) } else { self.x.clone() },
            m: if var == 'm' { (v, v) } else { self.m.clone() },
            a: if var == 'a' { (v, v) } else { self.a.clone() },
            s: if var == 's' { (v, v) } else { self.s.clone() },
        });
    }
}

fn main() {
    let mut workflows: HashMap<String, Vec<Condition>> = HashMap::new();
    let mut items: Vec<Item> = Vec::new();

    let mut workflow_name = String::new();
    let mut conditions_str = String::new();

    let mut x = 0;
    let mut m = 0;
    let mut a = 0;
    let mut s = 0;

    let mut seen_empty = false;

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();

        if line == "" {
            seen_empty = true;
            continue;
        }

        if !seen_empty {
            if let Ok(_) = sscanf!(&line, "{}{{{}}}", workflow_name, conditions_str) {
                let conds_str = conditions_str.split(",").collect::<Vec<&str>>();

                let mut conditions = Vec::new();

                for cond_str in &conds_str[0..conds_str.len() - 1] {
                    let mut char_iter = cond_str.chars();
                    let var = char_iter.next().unwrap();
                    let op = char_iter.next().unwrap();
                    let mut val = 0;
                    let mut next = String::new();
                    let rest = char_iter.as_str();
                    sscanf!(rest, "{}:{}", val, next).unwrap();
                    conditions.push(Condition::Condition { var, op, val, next });
                }

                conditions.push(Condition::Always {
                    next: conds_str[conds_str.len() - 1].to_string(),
                });

                workflows.insert(workflow_name.clone(), conditions);
            }
        } else {
            if let Ok(_) = sscanf!(&line[1..line.len() - 1], "x={},m={},a={},s={}", x, m, a, s) {
                items.push(Item { x, m, a, s });
            }
        }
    }

    let mut sum = 0;
    for item in items {
        let mut key = "in".to_string();

        loop {
            if key == "A" {
                sum += item.a + item.s + item.m + item.x;
                break;
            }

            if key == "R" {
                break;
            }

            let conditions = workflows.get(&key).unwrap();
            for condition in conditions {
                match condition {
                    Condition::Always { next } => {
                        key = next.clone();
                        break;
                    }
                    Condition::Condition { var, op, val, next } => {
                        let v = match var {
                            'x' => item.x,
                            'm' => item.m,
                            'a' => item.a,
                            's' => item.s,
                            _ => panic!("unknown var"),
                        };

                        let ok = match op {
                            '>' => v > *val,
                            '<' => v < *val,
                            '=' => v == *val,
                            _ => panic!("unknown op"),
                        };

                        if ok {
                            key = next.clone();
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", sum);

    let mut accepted: Vec<ItemRange> = Vec::new();
    let mut work = VecDeque::new();
    work.push_back((
        "in".to_string(),
        0,
        Some(ItemRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }),
    ));

    while let Some((key, cindex, mranges)) = work.pop_front() {
        let ranges = match mranges {
            Some(ranges) => ranges,
            None => continue,
        };

        if key == "A" {
            accepted.push(ranges);
            continue;
        }

        if key == "R" {
            continue;
        }

        let condition = workflows.get(&key).unwrap()[cindex].clone();

        match condition {
            Condition::Always { next } => {
                work.push_back((next.clone(), 0, Some(ranges.clone())));
            }

            Condition::Condition { var, op, val, next } => match op {
                '=' => {
                    // If we match the condition we continue
                    work.push_back((next.clone(), 0, ranges.process_equal(var, val)));

                    // Otherwise we cannot match the condition, so we add the negation
                    // In this case we're either smaller than or larger than val
                    // respectively
                    work.push_back((
                        key.clone(),
                        cindex + 1,
                        ranges.process_smaller_than(var, val),
                    ));
                    work.push_back((
                        key.clone(),
                        cindex + 1,
                        ranges.process_larger_than(var, val),
                    ));
                }
                '>' => {
                    // We match
                    work.push_back((next.clone(), 0, ranges.process_larger_than(var, val)));

                    // We don't match, so we add the negation
                    work.push_back((
                        key.clone(),
                        cindex + 1,
                        ranges.process_smaller_than(var, val + 1),
                    ));
                }
                '<' => {
                    // We match
                    work.push_back((next.clone(), 0, ranges.process_smaller_than(var, val)));

                    // We don't match, so we add the negation
                    work.push_back((
                        key.clone(),
                        cindex + 1,
                        ranges.process_larger_than(var, val - 1),
                    ));
                }

                _ => panic!("unknown op"),
            },
        }
    }

    let mut sum = 0;
    for option in accepted {
        let optx = option.x.1 - option.x.0 + 1;
        let optm = option.m.1 - option.m.0 + 1;
        let opta = option.a.1 - option.a.0 + 1;
        let opts = option.s.1 - option.s.0 + 1;
        sum += optx * optm * opta * opts;
    }

    println!("Part 2: {}", sum);
}
