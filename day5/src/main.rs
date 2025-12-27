use std::{cmp::max, default, fs, io::Error, num, string};

fn main() {
    let input = read_input();
    let (intervals, ingredient_ids) = parse_input(input.trim());
    let mut fresh_ingredients = 0;
    for id in ingredient_ids {
        for interval in &intervals {
            if interval.contains(id) {
                fresh_ingredients += 1;
                break
            }
            else if interval.past(id) {
                // this only works if its sorted.. but its not
                // #optimizations
                // break
            }
        }
    }
    println!("{fresh_ingredients}")
}

struct Interval {
    start: i64,
    end: i64
}

impl Interval {
    fn new(start : i64,end : i64) -> Result<Interval, String> {
        if start > end {
            Err(String::from(format!("Interval start cannot be greater than interval end {}, {}", start, end)))?
        }
        Ok(Interval {start,end})
    }

    fn contains(&self, number :i64) -> bool {
        let result = number >= self.start && number <= self.end;
        if result {
            println!("{} is fresh, in range {} and {}", number, self.start, self.end)
        }
        result
    }

    fn past(&self, number :i64) -> bool {
        self.start > number // tells us when to prune
    }
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

fn parse_number(s: &str) -> Result<i64, std::num::ParseIntError> {
    let num = s.trim().parse::<i64>()?;
    Ok(num)
}

fn parse_input(input: &str) -> (Vec<Interval>, Vec<i64>) {
    // split by comma for each pair
    // split each pair by -
    let mut intervals : Vec<Interval> = Vec::new();
    // split by \n\n to get the two parts
    let (ranges,ingredients) = input.split_once("\n\n").unwrap();
    let mut intervals = Vec::new();
    for interval in ranges.lines() {
        let (start,end) = interval.split_once("-").unwrap();
        let interval_object = Interval::new(
            parse_number(start).expect("start bad"),
            parse_number(end).expect("end bad")
        ).unwrap();
        intervals.push(interval_object)
    }
    let ingredient_ids : Vec<i64> = ingredients.lines()
        .map(|x| parse_number(x).expect("ingredient bad"))
        .collect();
    (intervals, ingredient_ids)

}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use super::*; // Import items from parent module

    #[test]
    fn test_bank_to_max() {
   }

    #[test]
    fn test_greedy(){
    }

}
/*
 *
 // split down the middle
 // odd number skip
 *
 */
