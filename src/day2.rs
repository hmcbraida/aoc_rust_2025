use std::{collections::HashSet, fs};

type InputData = Vec<(u64, u64)>;

fn parse_input(file_path: &str) -> InputData {
    let contents = fs::read_to_string(file_path).unwrap();

    let contents = contents
        .split(",")
        .filter_map(|x| {
            if x.len() == 0 {
                return None;
            }
            let split_val = x.trim_end().split("-").collect::<Vec<_>>();

            let first = split_val[0].parse::<u64>().unwrap();
            let last = split_val[1].parse::<u64>().unwrap();

            return Some((first, last));
        })
        .collect();

    return contents;
}

fn is_special1(val: u64) -> bool {
    let val_str = val.to_string();

    if val_str.len() % 2 == 1 {
        return false;
    }

    let split_idx = val_str.len() / 2;

    let first_half = &val_str[..split_idx];
    let second_half = &val_str[split_idx..];

    return first_half == second_half;
}

fn is_special2(val: u64) -> bool {
    let val_str = val.to_string();

    for pattern_size in 1..=(val_str.len() / 2) {
        if val_str.len() % pattern_size != 0 {
            continue;
        }

        let target_pattern = &val_str[..pattern_size];
        let mut success = true;
        for i in 1..val_str.len() / pattern_size {
            let test_pattern = &val_str[i * pattern_size..(i + 1) * pattern_size];
            if test_pattern != target_pattern {
                success = false;
                break;
            }
        }

        if success {
            return true;
        }
    }

    return false;
}

fn count_specials(interval: (u64, u64), seen: &mut HashSet<u64>) -> (u64, u64) {
    let (start_n, end_n) = interval;
    let mut count1 = 0;
    let mut count2 = 0;
    for n in start_n..=end_n {
        if seen.contains(&n) {
            continue;
        }
        seen.insert(n);
        if is_special1(n) {
            count1 += n;
        }
        if is_special2(n) {
            count2 += n;
        }
    }

    return (count1, count2);
}

pub fn go(file_path: &str) -> String {
    let input_data = parse_input(file_path);

    let mut seen = HashSet::new();
    let mut count1 = 0;
    let mut count2 = 0;
    for interval in input_data {
        let (delta1, delta2) = count_specials(interval, &mut seen);
        count1 += delta1;
        count2 += delta2;
    }

    return String::from(format!(
        "Count (part 1): {}; count (part 2): {}",
        count1, count2
    ));
}
