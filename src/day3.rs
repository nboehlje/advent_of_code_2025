use std::fs;

pub fn run() {
    let file_path = "data/day3_puzzle_input.txt";

    println!("================= [BEGIN DAY 3] =================");
    println!("(PART 1):\n");
    let part1_ans = get_part1_answer(file_path);
    println!("ANS: [{part1_ans}]");

    println!("\n(PART 2):\n");
}

fn get_part1_answer(file_name: &str) -> u64 {
    let mut total_joltage:u64 = 0;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        total_joltage += get_max_joltage(line);
    }

    total_joltage
}

fn get_max_joltage(battery_bank: &str) -> u64 {
    println!("------------ {battery_bank} ------------");
    if battery_bank.len() == 0 {
        return 0;
    }

    let ascii_bytes = battery_bank.as_bytes();
    let mut iter = ascii_bytes.iter();
    let length = ascii_bytes.len();
    let mut index = 2;

    let mut first_digit: u8 = match iter.next() {
        Some(x) => *x,
        None => panic!("Invalid battery bank."),
    };
    let mut second_digit: u8 = match iter.next() {
        Some(x) => *x,
        None => panic!("Invalid battery bank."),
    };

    while let Some(i) = iter.next() {
        // special case for last index of the array
        if index == (length - 1) {
            if *i > second_digit {
                second_digit = *i;
            }

            break;
        }

        if *i > first_digit {
            first_digit = *i;
            second_digit = match iter.next() {
                Some(x) => *x,
                None => panic!("Invalid battery bank."),
            };
            index += 1;
        } else if *i > second_digit {
            second_digit = *i;
        }

        index += 1;
    }

    // convert ascii values to base 10
    ((first_digit - 48) * 10 + (second_digit - 48)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_max_joltage_should_return_correct_numbers() {
        let result0 = get_max_joltage("");
        let result1 = get_max_joltage("987654321111111");
        let result2 = get_max_joltage("811111111111119");
        let result3 = get_max_joltage("234234234234278");
        let result4 = get_max_joltage("818181911112111");
        let result5 = get_max_joltage("7466158214373377771857781284845741681685815142631524817317361384343713861153487433435244725151654819");

        assert_eq!(result0, 0);
        assert_eq!(result1, 98);
        assert_eq!(result2, 89);
        assert_eq!(result3, 78);
        assert_eq!(result4, 92);
        assert_eq!(result5, 89);
    }
}
