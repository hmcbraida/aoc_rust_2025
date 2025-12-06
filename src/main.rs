mod day1;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_fn_str = args[1].as_str();
    let input_path = args[2].as_str();

    let target_fn = match target_fn_str {
        "day1" => day1::go,
        _ => panic!("Unexpected subcommand {}", target_fn_str),
    };

    println!("{}", target_fn(input_path));
}
