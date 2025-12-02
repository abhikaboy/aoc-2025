use std::fs;

/*
 * PLAN:
 *  read input
    parse input
    mutable variable to represent current position
    function to process each step
    mutable variable to represent the count
*/


fn main() {
    let raw_commands = read_input();
    let commands: Vec<i32> = parse_commands(raw_commands);
    let mut position = 50;
    let mut count = 0;
    const TARGET:i32 = 0;
    // apply all commands
    for c in commands {
        position = apply_command(position, c);
        if position.eq(&TARGET){
            count += 1
        }
    }
    println!("{count}")

}

fn apply_command(pos:i32, command:i32) -> i32 {
    return dial_wrap(pos + command);
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
    let num = s.parse::<i32>()?;
    Ok(num)
}

fn parse_commands(raw_commands: String) -> Vec<i32> {
    let mut commands: Vec<i32> = Vec::new();
    const POSITIVE_DIRECTION:&str = "R";
    for item in raw_commands.split("\n") {
        if item.len() < 2 {break};
        let (direction, magnitude) = item.split_at(1);
        let direction_integer = if direction.eq(POSITIVE_DIRECTION) { 1 } else { -1 };
        let manitude_integer:i32 = parse_number(magnitude).unwrap();
        commands.push(manitude_integer * direction_integer);
    }
    commands
}

fn dial_wrap(pos: i32) -> i32 {
    match pos {
        0..=99 => pos, // in range
        pos if pos < 0=> dial_wrap(100 + pos), // negative
        pos => dial_wrap(pos - 100), // overflow, 100+
    }
}
