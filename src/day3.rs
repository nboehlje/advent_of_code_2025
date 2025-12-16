use std::fs;

pub fn run() {
    let file_path = "data/day3_puzzle_input.txt";

    println!("================= [BEGIN DAY 3] =================");
    println!("(PART 1):\n");
    let part1_ans = get_part1_answer(file_path);
    dbg!(part1_ans);
    println!("ANS: [{part1_ans}]");

    println!("\n(PART 2):\n");
}

fn get_part1_answer(file_name: &str) -> u64 {
    let mut total_joltage:u64 = 0;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        dbg!(line);
        let joltage_mine = get_max_joltage(line);
        let joltage_gpt = get_max_joltage2(line);
        assert_eq!(joltage_mine, joltage_gpt);
        total_joltage += joltage_mine;
    }

    total_joltage
}

fn get_max_joltage(battery_bank: &str) -> u64 {
    if battery_bank.len() == 0 {
        return 0;
    }

    let ascii_bytes = battery_bank.as_bytes();
    let mut iter = ascii_bytes.iter();
    let length = ascii_bytes.len();
    let mut index = 1;

    let mut first_digit: u8 = match iter.next() {
        Some(x) => *x,
        None => panic!("Invalid battery bank."),
    };

    // WARNING: what if this digit should be assigned to first_digit instead
    let mut second_digit: u8 = match iter.next() {
        Some(x) => *x,
        None => panic!("Invalid battery bank."),
    };

    while let Some(i) = iter.next() {
        index += 1;
        // special case for last index of the array
        if index == (length - 1) {
            if *i > second_digit {
                second_digit = *i;
            }

            break;
        }

        if *i > first_digit {
            first_digit = *i;
            // WARNING: what if this next digit should be assigned to first_digit instead
            second_digit = match iter.next() {
                Some(x) => *x,
                None => panic!("Invalid battery bank."),
            };
            index += 1;
            continue;

        } else if *i > second_digit {
            second_digit = *i;
        }
    }

    // convert ascii values to base 10
    ((first_digit - 48) * 10 + (second_digit - 48)).into()
}

pub fn get_max_joltage2(battery_bank: &str) -> u64 {
    // Convert string to a vector of u32 digits, ignoring non-numeric chars
    let digits: Vec<u32> = battery_bank
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let mut max_num = 0;

    // Iterate through every digit to consider it as the "Tens" place
    for (i, &tens_digit) in digits.iter().enumerate() {
        // Look at all digits AFTER the current one (the slice [i+1..])
        // to find the largest available "Ones" place.
        if let Some(&max_ones_digit) = digits[i + 1..].iter().max() {
            let current_num = (tens_digit * 10) + max_ones_digit;
            if current_num as u64 > max_num {
                max_num = current_num as u64;
            }
        }
    }

    max_num
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
        let result6 = get_max_joltage("2532532222252232322255334212432242532324342222122211222152521232153352132332124222242252115222223232");
        let result7 = get_max_joltage("2241331515235252543314125323552321114343514433315352155455222144425531342431345525542255155155246789");
        let result8 = get_max_joltage("387899");
        let result9 = get_max_joltage("2433433442543562123444379344444372533533472544342724442335624533743422456332212544345323353443735275");

        assert_eq!(result0, 0);
        assert_eq!(result1, 98);
        assert_eq!(result2, 89);
        assert_eq!(result3, 78);
        assert_eq!(result4, 92);
        assert_eq!(result5, 89);
        assert_eq!(result6, 55);
        assert_eq!(result7, 89);
        assert_eq!(result8, 99);
        assert_eq!(result9, 97);
    }

    #[test]
    fn get_max_joltage2_should_return_correct_numbers() {
        let result1 = get_max_joltage2("987654321111111");
        let result2 = get_max_joltage2("811111111111119");
        let result3 = get_max_joltage2("234234234234278");
        let result4 = get_max_joltage2("818181911112111");
        let result5 = get_max_joltage2("7466158214373377771857781284845741681685815142631524817317361384343713861153487433435244725151654819");
        let result6 = get_max_joltage2("2532532222252232322255334212432242532324342222122211222152521232153352132332124222242252115222223232");
        let result7 = get_max_joltage2("2241331515235252543314125323552321114343514433315352155455222144425531342431345525542255155155246789");
        let result8 = get_max_joltage2("387899");

        assert_eq!(result1, 98);
        assert_eq!(result2, 89);
        assert_eq!(result3, 78);
        assert_eq!(result4, 92);
        assert_eq!(result5, 89);
        assert_eq!(result6, 55);
        assert_eq!(result7, 89);
        assert_eq!(result8, 99);
    }
}
