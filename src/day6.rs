use std::fs;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Plus,
    Times,
}

struct InputData {
    rows: Vec<Vec<u64>>,
    operators: Vec<Operator>,
}

fn parse_input(input_path: &str) -> InputData {
    let mut lines: Vec<_> = fs::read_to_string(input_path)
        .unwrap()
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    let last_line = lines.pop().unwrap();

    let rows: Vec<Vec<u64>> = lines
        .iter()
        .map(|x| {
            x.split(" ")
                .filter(|x| x.len() > 0)
                .map(|n| n.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    let operators: Vec<_> = last_line
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| match x {
            "+" => Operator::Plus,
            "*" => Operator::Times,
            _ => panic!(),
        })
        .collect();

    // println!("{:?}, {:?}", rows, operators);

    InputData { rows, operators }
}

fn transpose(rows: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let w = rows[0].len();
    let h = rows.len();

    // println!("{},{}", w, h);

    (0..w)
        .map(|x| (0..h).map(|y| rows[y][x]).collect())
        .collect()
}

fn compute(row: &Vec<u64>, operator: Operator) -> u64 {
    match operator {
        Operator::Plus => row.iter().sum(),
        Operator::Times => row.iter().fold(1, |acc, val| acc * val),
    }
}

pub fn go(input_path: &str) -> String {
    let input_data = parse_input(input_path);
    let columns = transpose(&input_data.rows);

    let total: u64 = columns
        .iter()
        .zip(input_data.operators.iter())
        // .inspect(|x| println!("{:?}", x))
        .map(|(col, op)| compute(col, *op))
        // .inspect(|x| println!("{}", x))
        .sum();

    format!("Total: {}", total)
}
