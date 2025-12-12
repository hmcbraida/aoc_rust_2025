mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_fn_str = args[1].as_str();
    let input_path = args[2].as_str();

    let target_fn = match target_fn_str {
        "day1" => day1::go,
        "day2" => day2::go,
        "day3" => day3::go,
        "day4" => day4::go,
        "day5" => day5::go,
        "day6" => day6::go,
        "day7" => day7::go,
        "day8" => day8::go,
        _ => panic!("Unexpected subcommand {}", target_fn_str),
    };

    println!("{}", target_fn(input_path));
}
