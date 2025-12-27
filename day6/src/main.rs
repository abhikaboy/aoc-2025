use std::{cmp::max, default, fs, io::Error, num, panic, string};

fn main() {
    let input = read_input();
    let columns = parse_input(input.trim());
    let sum = columns.iter()
        .fold(0, |acc, col| acc + col.compute());
    println!("{sum}")

}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Multiply,
    Add,
    None,
}

struct Column {
    numbers: Vec<i64>,
    operation: Operation,
}

fn string_to_operation(s: &str) -> Operation{
    match s {
        "*" => Operation::Multiply,
        "+" => Operation::Add,
        _ => panic!("Invalid Operation [{}]", s)
    }
}

impl Column {
    fn new(numbers: Vec<i64>, op: Operation) -> Column {
        Column {numbers, operation: op}
    }

    fn append_number(&mut self, number : i64){
        self.numbers.push(number);
    }

    fn set_operation(&mut self, operation : &str){
        if self.operation != Operation::None { panic!("Cannot set operation once set"); }
        self.operation = string_to_operation(operation)
    }

    fn compute(&self) -> i64 {
        let res = match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
            Operation::None => 0
        };

        println!("{res} {}", self.numbers.len());
        res
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

fn parse_input(input: &str) -> Vec<Column> {
    let mut cols : Vec<Column> = Vec::new();
    let lines_lol = input.lines();
    for (index, lines) in lines_lol.enumerate() {
        if index == 4 {
            for (index, item) in lines.split(" ").filter(|s| s.len() > 0).enumerate() {
                let pop = &mut cols[index];
                pop.set_operation(item.trim());
            }
        } else {
            for (index2,item) in lines.split(" ").filter(|s| s.len()>0).enumerate() {
                let num = parse_number(item.trim()).unwrap();
                if index == 0 {
                    cols.push(Column::new(vec![num], Operation::None))
                } else {
                    cols[index2].append_number(num);
                }
            }
        }
    }
    cols
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
