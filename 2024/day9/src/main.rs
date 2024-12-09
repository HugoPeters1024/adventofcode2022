use std::io::BufRead;

fn main() {
    let input: Vec<u32> = std::io::stdin()
        .lock()
        .lines()
        .flat_map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut expanded: Vec<u32> = Vec::new();

    let mut file_id = 0;
    for (idx, c) in input.iter().enumerate() {
        if idx % 2 == 0 {
            // file
            for _ in 0..*c {
                expanded.push(file_id);
            }
            file_id += 1;
        } else {
            // free space
            for _ in 0..*c {
                expanded.push(u32::MAX);
            }
        }
    }

    let mut buffer = expanded.clone();
    let mut left_needle = 0;
    let mut right_needle = buffer.len() - 1;

    while left_needle < right_needle {
        if buffer[left_needle] != u32::MAX {
            left_needle += 1;
            continue;
        }

        if buffer[right_needle] == u32::MAX {
            right_needle -= 1;
            continue;
        }

        buffer[left_needle] = buffer[right_needle];
        buffer[right_needle] = u32::MAX;
        left_needle += 1;
        right_needle -= 1;
    }

    let mut checksum = 0;
    for (idx, file_id) in buffer.iter().enumerate() {
        if *file_id == u32::MAX {
            continue;
        }

        checksum += idx * *file_id as usize;
    }
    println!("Part 1: {checksum}");

    let mut buffer = expanded.clone();
    let mut right_needle = buffer.len() - 1;

    while right_needle > 0 {
        if buffer[right_needle] == u32::MAX {
            right_needle -= 1;
            continue;
        }

        let mut space_needed = 0;
        let mut look_ahead_right = right_needle;

        while look_ahead_right > 0 && buffer[look_ahead_right] == buffer[right_needle] {
            space_needed += 1;
            look_ahead_right -= 1;
        }

        let mut left_needle = 0;
        let mut space_found_tmp = 0;
        let mut space_found_at = None;

        while left_needle < right_needle {
            if space_found_tmp == space_needed {
                space_found_at = Some(left_needle - space_needed);
                break;
            }

            if buffer[left_needle] == u32::MAX {
                space_found_tmp += 1;
            } else {
                space_found_tmp = 0;
            }
            left_needle += 1;
        }

        if space_found_at.is_none() {
            right_needle -= space_needed;
            continue;
        } else {
            let space_found_at = space_found_at.unwrap();
            let fill_with = buffer[right_needle];
            for idx in 0..space_needed {
                buffer[space_found_at + idx] = fill_with;
                buffer[right_needle - idx] = u32::MAX;
            }
            right_needle -= space_needed;
        }
    }

    let mut checksum = 0;
    for (idx, file_id) in buffer.iter().enumerate() {
        if *file_id == u32::MAX {
            continue;
        }

        checksum += idx * *file_id as usize;
    }
    println!("Part 2: {checksum}");
}
