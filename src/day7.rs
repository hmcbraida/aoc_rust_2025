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

fn find_splits(input_data: &InputData) -> (u64, u64) {
    let w = input_data.splitter_rows[0].len();
    let initial_timelines: Vec<u64> = (0..w)
        .map(|idx| if idx == input_data.start_idx { 1 } else { 0 })
        .collect();

    let (final_timelines, split_count) = input_data.splitter_rows.iter().fold(
        (initial_timelines, 0),
        |(prev_timelines, split_count), row| {
            let mut new_row: Vec<u64> = (0..w).map(|_| 0).collect();
            let mut split_count = split_count;

            // println!("{:?}", prev_timelines);

            for (col_idx, is_splitter) in row.iter().enumerate() {
                let is_splitter = *is_splitter;

                let n_prev_timelines = prev_timelines[col_idx];

                if n_prev_timelines == 0 {
                    continue;
                }

                if is_splitter {
                    if let Some(r) = new_row.get_mut(col_idx + 1) {
                        *r += n_prev_timelines;
                    }
                    if let Some(l) = new_row.get_mut(col_idx - 1) {
                        *l += n_prev_timelines;
                    }

                    split_count += 1;
                } else {
                    new_row[col_idx] += n_prev_timelines;
                }
            }

            // println!("{:?}", new_row);

            (new_row, split_count)
        },
    );

    // println!("{:?}", final_timelines);

    (split_count, final_timelines.iter().sum())
}

pub fn go(input_path: &str) -> String {
    let input_data = parse_input(&input_path);

    let (n_splits, n_timelines) = find_splits(&input_data);

    // format!("{:?}\n{:?}", input_data, splits)
    format!(
        "Total splits: {}\nTotal timelines: {}",
        n_splits, n_timelines
    )
}
