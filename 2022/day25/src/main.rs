use std::io::BufRead;

fn main() {
//    dbg!(snafu_to_decimal(&"1=-0-2".to_string()));
//    dbg!(decimal_to_snafu(198));
    
    let mut total = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let d = snafu_to_decimal(&line);
        total += d;
    }

    dbg!(total);
    dbg!(decimal_to_snafu(total));
}

fn snafu_to_decimal(x: &String) -> i64 {
    let mut cursor = 1;
    let mut result = 0;

    for c in x.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            result += digit as i64 * cursor;
        } else {
            if c == '-' {
                result -= cursor;
            } else if c == '=' {
                result -= 2 * cursor
            } else {
                panic!();
            }
        }
        cursor *= 5;
    }

    result
}

fn decimal_to_snafu(x: i64) -> String {
    let mut result = String::new();
    let mut x = x;
    let mut carry = 0;

    loop {
        let rest = x % 5;

        if rest == 0 {
            result.push('0');
            carry = 0;
        } else if rest == 1 {
            result.push('1');
            carry = 0;
        } else if rest == 2 {
            result.push('2');
            carry = 0;
        } else if rest == 3 {
            result.push('=');
            carry = 1;
        } else if rest == 4 {
            result.push('-');
            carry = 1;
        }

        x -= rest;
        x /= 5;
        x += carry;

        if x == 0 {
            break;
        }
    }

    result.chars().rev().collect()
}
