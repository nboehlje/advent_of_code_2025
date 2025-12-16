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
        total_joltage +=  get_max_joltage(line);
    }

    total_joltage
}

fn get_max_joltage(battery_bank: &str) -> u64 {
    if battery_bank.len() == 0 {
        return 0;
    }

    let ascii_bytes = battery_bank.as_bytes();
    let mut iter = ascii_bytes.iter();
    let mut index = 0;

    let mut first_digit: u8 = match iter.next() {
        Some(x) => *x,
        None => panic!("Invalid battery bank."),
    }; 
    let mut second_digit: u8 = 0;

    while let Some(i) = iter.next() {
        index += 1;
        // special case for last index of the array
        if index == (ascii_bytes.len() - 1) {
            if *i > second_digit {
                second_digit = *i;
            }

            break;
        }

        if *i > first_digit {
            first_digit = *i;
            second_digit = 0; // zero out the second digit b/c it must come *after*
                              // the first digit in the sequence
        } else if *i > second_digit {
            second_digit = *i;
        }
    }

    // convert ascii bytes to the representative number
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

}
