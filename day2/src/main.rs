use std::{default, fs, num, string};

fn main() {
    let input = read_input();
    let parsedInput = parse_input(input);
    let mut sum = 0;
    for range in parsedInput{
        sum += invalids_in_range(range)
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

// directly returns the sum
fn invalids_in_range(range:(i64,i64)) -> i64 {
    let (start,end) = range;
    let mut sum = 0;
    for i in start..end{
        sum += check_valid(i);
    }
    sum
}

fn mirror_string(s: &str) -> bool {
    let (first,second) = s.split_at(s.len() / 2);
    return first == second
}

fn check_valid(num: i64) -> i64 {
    let string_num = num.to_string();
    let length = string_num.len();
    match length {
        n if n % 2 == 1 => 0,
        _ => if mirror_string(&string_num) {num} else {0}
    }
}

fn parse_number(s: &str) -> Result<i64, std::num::ParseIntError> {
    let num = s.trim().parse::<i64>()?;
    Ok(num)
}

fn parse_input(input: String) -> Vec<(i64,i64)> {
    // split by comma for each pair
    // split each pair by -
    let mut ranges:Vec<(i64,i64)> = Vec::new();
    for range in input.split(","){
        // let split_index = range.find("-").
        let stringified_range = range.split_at(range.find("-").expect("Misformatted Input Range"));
        let int_range = (parse_number(stringified_range.0).expect("First Input in Stringified Range cannot be int")
                        ,-1*parse_number(stringified_range.1).expect(&format!("Second Input in Stringified Range [{}] cannot be int", stringified_range.1))
        );
        ranges.push(int_range)
    }
    ranges
}


#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module

    #[test]
    fn test_mirror_string() {
        assert_eq!(mirror_string("1010"), true);
        assert_eq!(mirror_string("55"), true);
        assert_eq!(mirror_string("123"), false);
    }

    #[test]
    fn test_valid() {
        assert_eq!(check_valid(1010), 1010);
        assert_eq!(check_valid(55), 55);
        assert_eq!(check_valid(123), 0);
    }
}


/*
 *
 // split down the middle
 // odd number skip
 *
 */
