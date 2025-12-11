use std::fs;

#[derive(Debug)]
struct InputData {
    splitter_rows: Vec<Vec<bool>>,
    start_idx: usize,
}

fn parse_input(input_path: &str) -> InputData {
    let contents = fs::read_to_string(input_path).unwrap();

    let splitter_rows = contents
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|text_row| {
            text_row
                .chars()
                .map(|c| match c {
                    '^' => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    let start_idx = contents
        .split("\n")
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    InputData {
        splitter_rows,
        start_idx,
    }
}

fn find_splits(input_data: &InputData) -> u64 {
    let w = input_data.splitter_rows[0].len();
    let initial_splits: Vec<bool> = (0..w)
        .map(|idx| {
            if idx == input_data.start_idx {
                true
            } else {
                false
            }
        })
        .collect();

    let (_, split_count) = input_data.splitter_rows.iter().fold(
        (initial_splits, 0),
        |(prev_beams, split_count), row| {
            let mut new_row: Vec<bool> = (0..w).map(|_| false).collect();
            let mut split_count = split_count;

            for (col_idx, is_splitter) in row.iter().enumerate() {
                let is_splitter = *is_splitter;

                if !prev_beams[col_idx] {
                    continue;
                }

                if is_splitter {
                    if let Some(r) = new_row.get_mut(col_idx + 1) {
                        *r = true;
                    }
                    if let Some(l) = new_row.get_mut(col_idx - 1) {
                        *l = true;
                    }

                    split_count += 1;
                } else {
                    new_row[col_idx] = true;
                }
            }

            // println!("{:?}", new_row);

            (new_row, split_count)
        },
    );

    split_count
}

pub fn go(input_path: &str) -> String {
    let input_data = parse_input(&input_path);

    let n_splits = find_splits(&input_data);

    // format!("{:?}\n{:?}", input_data, splits)
    format!("Total splits: {}", n_splits)
}
