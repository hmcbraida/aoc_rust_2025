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

fn count_zeros(input: InputData) -> (u64, u64) {
    // count will represent the number of times we stop at 0.
    let mut count = 0;
    // n_clicks will represent the number of times we "click past" 0.
    let mut n_clicks = 0;
    // We use this to keep track of our position as we loop through the input.
    let mut current_pos = 50;

    for datum in input {
        let new_pos = current_pos + datum;
        // The number of clicks is (about) the Euclidean remainder.
        // See the correction factors below.
        let mut clicks = new_pos.div_euclid(100).abs() as u64;
        // New position is of course the Euclidean division.
        let new_pos = new_pos.rem_euclid(100);

        // We need a little correction factor or two here...
        // We should NOT count it as a click if we started at zero and loop back
        // round.
        if current_pos == 0 && datum < 0 {
            clicks -= 1;
        }

        // We SHOULD count it as a click if we started at say 75 and rotate 25
        // to reach 0.
        if new_pos == 0 && datum < 0 {
            clicks += 1;
        }

        // Update position
        current_pos = new_pos;

        if current_pos == 0 {
            count += 1;
        }
        n_clicks += clicks;
    }

    return (count, n_clicks);
}

pub fn go(file_path: &str) -> String {
    let input = parse_file(file_path);
    let (count, n_clicks) = count_zeros(input);

    return format!("stopped at 0 {} times.\npassed 0 {} times", count, n_clicks);
}
