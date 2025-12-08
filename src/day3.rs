use std::fs;

type InputData = Vec<Vec<u64>>;

fn parse_input(input_path: &str) -> InputData {
    fs::read_to_string(input_path)
        .unwrap()
        .split("\n")
        .filter_map(|line| {
            if line.len() == 0 {
                return None;
            }

            Some(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

fn max_w_idx(bank: &[u64]) -> (u64, usize) {
    let mut max_pos: usize = 0;
    let mut max: u64 = 0;

    for (pos, d) in bank.iter().enumerate() {
        if *d > max {
            max = *d;
            max_pos = pos;
        }
    }

    (max, max_pos)
}

fn bank_max(bank: &[u64], s: usize) -> u64 {
    let mut maxes = Vec::with_capacity(12);

    let mut current_pos: usize = 0;
    for n in 0..s {
        let end = bank.len() - (s - n - 1);
        // println!("{},{}", current_pos, end);
        let range = &bank[current_pos..end];

        let (max, max_pos) = max_w_idx(range);
        current_pos += max_pos + 1;
        maxes.push(max);
    }

    // println!("{:?}", maxes);

    let mut total: u64 = 0;
    for (n, max) in maxes.iter().rev().enumerate() {
        let ex = n as u32;
        total += ((10 as u64).pow(ex)) * max;
    }

    total
}

pub fn go(input_path: &str) -> String {
    let input_data = parse_input(input_path);

    let count2: u64 = input_data.iter().map(|bank| bank_max(bank, 2)).sum();
    let count12: u64 = input_data.iter().map(|bank| bank_max(bank, 12)).sum();

    format!("Total power (len 2): {}\n(len 12): {}", count2, count12)
}
