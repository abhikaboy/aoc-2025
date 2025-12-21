use std::{cmp::max, default, fs, num, string};

fn main() {
    let input = read_input();
    let banks = parse_input(input.trim());
    let mut sum = 0;
    for bank in banks {
        sum += bank_to_max_power_greedy(&bank)
    }
    println!("{sum}")
}

fn read_input() -> String {
    match fs::read_to_string("input.txt") {
        Ok(contents) => {
            println!("Found input file!");
            return contents
        }
        Err(error) => {
            eprintln!("Could not read the input file: {}", error);
            return String::from("");
        }
    }
}

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    let num = s.trim().parse::<i32>()?;
    Ok(num)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    // split by comma for each pair
    // split each pair by -
    let mut banks:Vec<Vec<i32>> = Vec::new();
    for row in input.split("\n") {
        let bank:Vec<i32> = row.trim().chars().map(|s| parse_number(&format!("{}",s)).expect(
            &format!("cannot parse number [{}]",s))).collect();
        banks.push(bank);
    }
    banks
}
fn banksToPower(left:i32, right:i32) -> i32 {
    return left * 10 + right;
}

fn bank_to_max_power_greedy(input: &Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut max_power = 0;
    // two pointer algorithm
    while right < input.len()-1 {
        right += 1;
        max_power = max(banksToPower(input[left], input[right]),max_power);
        // need to shift
        if input[right] > input[left]{
            left = right;
        }
    }
    max_power
}

fn bank_to_max_power(input: &Vec<i32>) -> i32 {
    let mut max_power = 0;
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let power = input[i] * 10 + input[j];
            max_power = max_power.max(power);
        }
    }
    max_power
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use super::*; // Import items from parent module

    #[test]
    fn test_bank_to_max() {
        let banks = parse_input("5336553644444345344544134246423443634474453456455433543434354444344554344336446734443434424442135474");
        assert_eq!(bank_to_max_power(
            &banks[0]
        ),77);
        let banks = parse_input("2442432212422332232212684452222313252425265641254324242452246232226535522434643524124524537424423362");
        assert_eq!(bank_to_max_power_greedy(
            &banks[0]
        ),87);
        let case2 = parse_input("818181911112111");
        assert_eq!(bank_to_max_power(
            &case2[0]
        ),92);
        let case3 = parse_input("234234234234278");
        assert_eq!(bank_to_max_power(
            &case3[0]
        ),78);
    }

    #[test]
    fn test_greedy(){
        let input = read_input();
        let banks = parse_input(input.trim());
        for (i, bank) in banks.iter().enumerate() {
            let brute = bank_to_max_power(&bank);
            let greedy = bank_to_max_power_greedy(&bank);
            assert_eq!(
                greedy,
                brute,
                "Bank {} failed: {:?}\nGreedy got {}, brute is {}",
                i, bank, greedy, brute
            );
        }
    }
}


/*
 *
 // split down the middle
 // odd number skip
 *
 */
