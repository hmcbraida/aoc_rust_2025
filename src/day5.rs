use std::fs;

struct InputData {
    ranges: Vec<(u64, u64)>,
    stock: Vec<u64>,
}

fn parse_input(input_path: &str) -> InputData {
    let t: Vec<String> = fs::read_to_string(input_path)
        .unwrap()
        .split("\n\n")
        .map(|x| String::from(x))
        .collect();

    let range_contents = t[0].clone();
    let stock_contents = t[1].clone();

    let ranges: Vec<(u64, u64)> = range_contents
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|range_text| {
            let s: Vec<_> = range_text.split("-").collect();
            let first: u64 = s[0].parse().unwrap();
            let second: u64 = s[1].parse().unwrap();
            (first, second)
        })
        .collect();
    let ranges = merge_ranges(&ranges);
    let stock: Vec<u64> = stock_contents
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|stock_text| stock_text.parse().unwrap())
        .collect();

    InputData { ranges, stock }
}

fn merge_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut merged = Vec::new();

    for range in ranges {
        let mut intersectors: Vec<_> = merged
            .iter()
            .filter(|r| range_intersect(r, range))
            .map(|x| *x)
            .collect();
        intersectors.push(range.clone());

        let start = intersectors.iter().map(|x| x.0).min().unwrap();
        let end = intersectors.iter().map(|x| x.1).max().unwrap();

        let big_interval = (start, end);
        merged = merged
            .iter()
            .filter(|r| !range_intersect(&r, &big_interval))
            .map(|r| *r)
            .collect();
        merged.push(big_interval);
    }

    merged
}

fn range_intersect(r1: &(u64, u64), r2: &(u64, u64)) -> bool {
    return is_in_range(r2, r1.0)
        || is_in_range(r2, r1.1)
        || is_in_range(r1, r2.0)
        || is_in_range(r1, r2.1);
}

fn is_in_range(range: &(u64, u64), val: u64) -> bool {
    let (start, end) = range;
    return val >= *start && val <= *end;
}

fn is_in_ranges(ranges: &Vec<(u64, u64)>, val: u64) -> bool {
    for range in ranges {
        if is_in_range(&range, val) {
            return true;
        }
    }

    return false;
}

pub fn go(input_path: &str) -> String {
    let mut n_fresh = 0;

    let input_data = parse_input(&input_path);

    for stock_item in input_data.stock.iter() {
        if is_in_ranges(&input_data.ranges, *stock_item) {
            n_fresh += 1;
        }
    }

    let mut n_possible_fresh = 0;
    for range in input_data.ranges.iter() {
        n_possible_fresh += range.1 - range.0 + 1;
    }

    return format!(
        "Fresh items per stock: {}\nOverall: {}",
        n_fresh, n_possible_fresh
    );
}
