mod day1;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = args[2].as_str();

    let target_fn = match args[1].as_str() {
        "day1" => day1::go,
        _ => panic!(),
    };

    println!("{}", target_fn(input_path));
}
