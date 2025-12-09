use std::fs;

pub fn run() {
    let file_path = "data/day1_puzzle_input.txt";

    println!("================= [BEGIN DAY 1] =================");
    println!("(PART 1):\n");
    println!("Reading file: {file_path}");
    let password = get_password_part_1(file_path);
    println!("Part 1 ANS: [{password}]");

    println!("\n(PART 2):\n");
    println!("Reading file: {file_path}");
    let password = get_password_part_2(file_path);
    println!("Part 2 ANS: [{password}]");
}

fn get_password_part_1(file_name: &str) -> u32 {
    let mut current_position: i64 = 50;
    // the number of times the dial is left pointing at 0 after a rotation
    let mut num_zero_instances: u32 = 0;

    for line in fs::read_to_string(file_name).unwrap().lines() {
        let owned_line = line.to_string();
        let left = owned_line.strip_prefix("L");
        let right = owned_line.strip_prefix("R");

        let rotation_value: i32 = match (left, right) {
            (Some(_l), Some(_r)) => panic!("Invalid rotation value"),
            (Some(l), None) => {
                -(l.parse::<i32>()
                    .expect("Invalid string, should be a number"))
            }
            (None, Some(r)) => r.parse().expect("Invalid string, should be a number"),
            (None, None) => panic!("Invalid rotation value"),
        };

        current_position += rotation_value as i64;

        if current_position % 100 == 0 {
            num_zero_instances += 1;
        }
    }

    num_zero_instances
}

fn get_password_part_2(file_name: &str) -> i64 {
    let mut current_position: i32 = 50;
    // the number of times the dial is left pointing at 0 after a rotation
    let mut num_zero_instances: i64 = 0;

    for line in fs::read_to_string(file_name).unwrap().lines() {
        let owned_line = line.to_string();
        let left = owned_line.strip_prefix("L");
        let right = owned_line.strip_prefix("R");

        let rotation_value: i32 = match (left, right) {
            (Some(_l), Some(_r)) => panic!("Invalid rotation value"),
            (Some(l), None) => {
                -(l.parse::<i32>()
                    .expect("Invalid string, should be a number"))
            }
            (None, Some(r)) => r.parse().expect("Invalid string, should be a number"),
            (None, None) => panic!("Invalid rotation value"),
        };

        let num_times_passed_zero: i32;
        if rotation_value >= 0 {
            num_times_passed_zero = (current_position + rotation_value) / 100;
        } else {
            let negative_starting_point = if current_position == 0 {
                0
            } else {
                current_position - 100
            };
            num_times_passed_zero = ((negative_starting_point + rotation_value) / 100).abs();
        }
        num_zero_instances += num_times_passed_zero as i64;

        current_position = (current_position + rotation_value) % 100;
        if current_position < 0 {
            current_position += 100;
        }
    }

    num_zero_instances
}
