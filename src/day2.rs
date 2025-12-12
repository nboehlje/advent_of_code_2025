use std::fs;

pub fn run() {
    let file_path = "data/day2_puzzle_input.txt";

    println!("================= [BEGIN DAY 2] =================");
    println!("(PART 1):\n");
    let part1_ans = get_part1_answer(file_path);
    println!("ANS: [{part1_ans}]");

    println!("\n(PART 2):\n");
    let part2_ans = get_part2_answer(file_path);
    println!("ANS: [{part2_ans}]");
}

fn get_part1_answer(file_name: &str) -> u64 {
    let input: String = fs::read_to_string(file_name).unwrap();
    let split: Vec<&str> = input.split(",").collect();
    let mut running_total: u64 = 0;

    for id_range in split {
        let ids: Vec<&str> = id_range.split("-").collect();
        let lower: u64 = ids[0]
            .trim()
            .parse()
            .expect("Invalid string, should be a number");
        let upper: u64 = ids[1]
            .trim()
            .parse()
            .expect("Invalid string, should be a number");
        running_total += sum_invalid_ids_within_range_part1(lower, upper);
    }

    running_total
}

fn get_part2_answer(file_name: &str) -> u64 {
    let input: String = fs::read_to_string(file_name).unwrap();
    let split: Vec<&str> = input.split(",").collect();
    let mut running_total: u64 = 0;

    for id_range in split {
        let ids: Vec<&str> = id_range.split("-").collect();
        let lower: u64 = ids[0]
            .trim()
            .parse()
            .expect("Invalid string, should be a number");
        let upper: u64 = ids[1]
            .trim()
            .parse()
            .expect("Invalid string, should be a number");

        running_total += sum_invalid_ids_within_range_part2(lower, upper);
    }

    running_total
}

fn sum_invalid_ids_within_range_part1(lower: u64, upper: u64) -> u64 {
    let mut running_total: u64 = 0;
    for id in lower..=upper {
        let id_as_str = id.to_string();
        let str_len = id_as_str.chars().count();

        // skip odd numbers
        if (str_len & 0b1usize) != 0 {
            continue;
        }

        let (lhs, rhs) = id_as_str.split_at(str_len / 2);
        if *lhs == *rhs {
            running_total += id;
        }
    }

    running_total
}

fn sum_invalid_ids_within_range_part2(lower: u64, upper: u64) -> u64 {
    let mut running_total: u64 = 0;
    for id in lower..=upper {
        let id_as_str = id.to_string();
        let str_len = id_as_str.len();

        for n in 1..=(str_len / 2) {
            if str_len % n != 0 {
                continue;
            }

            let mut iter = id_as_str.as_bytes().chunks_exact(n);
            let first_chunk = match iter.next() {
                Some(chunk) => chunk, 
                None => { continue; } 
            };

            if iter.all(|chunk| chunk == first_chunk) {
                running_total += id;
                break;
            }
        }
    }

    running_total
}
