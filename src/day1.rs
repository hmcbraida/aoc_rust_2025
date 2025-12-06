use std::fs;

type InputData = Vec<i64>;

fn parse_file(file_path: &str) -> InputData {
    let contents = fs::read_to_string(file_path).unwrap();

    let contents = contents
        .split("\n")
        .map(|x| {
            if x.len() == 0 {
                return 0;
            }
            let direction: char = x.chars().collect::<Vec<_>>()[0];
            let rest: i64 = (&x[1..]).parse().unwrap();

            let datum = match direction {
                'L' => -rest,
                'R' => rest,
                _ => panic!(),
            };

            return datum;
        })
        .collect();

    return contents;
}

fn count_zeros(input: InputData) -> u64 {
    let mut count = 0;
    let mut current_pos = 50;

    for datum in input {
        current_pos += datum;
        current_pos = current_pos % 100;
        if current_pos == 0 {
            count += 1;
        }
    }

    return count;
}

pub fn go(file_path: &str) -> String {
    let input = parse_file(file_path);
    let count = count_zeros(input);

    return count.to_string();
}
