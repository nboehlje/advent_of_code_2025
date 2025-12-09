use std::fs;

pub fn run() {
    let file_path = "data/day2_puzzle_input.txt";

    println!("================= [BEGIN DAY 2] =================");
    println!("(PART 1):\n");
    part1(file_path);

    println!("\n(PART 2):\n");
}

fn part1(file_name: &str) -> () {
    let input: String = fs::read_to_string(file_name).unwrap();
    let split: Vec<&str> = input.split(",").collect();

    for id_range in split {
        let ids: Vec<&str> = id_range.split("-").collect();
        let lower: i64 = ids[0].trim().parse().expect("Invalid string, should be a number");
        let upper: i64 = ids[1].trim().parse().expect("Invalid string, should be a number");
        get_invalid_ids(lower, upper);
    }
}

fn get_invalid_ids(lower: i64, upper: i64) -> () {
    println!("lower: {lower}");
    println!("upper: {upper}");
}
