use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let user_choice = &args[1];
        match user_choice.as_str() {
            "day1" => day1::run(),
            "day2" => day2::run(),
            _ => panic!("Invalid argument"),
        }
    } else {
        println!("No argument given, exiting...");
    }
}
