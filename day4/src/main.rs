use std::{cmp::max, default, fs, num, panic, string};

fn main() {
    let input = read_input();
    let grid = parse_input(input.trim());
    let mut count = 0;

    for (x,row) in grid.iter().enumerate() {
        for (y,item) in row.iter().enumerate() {
            if (item != &(1 as i32)) {continue}
            count += check_neighbors((x as i32,y as i32), &grid)
        }
    }
    println!("{count}");
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

fn check_neighbors((x,y) : (i32,i32), input : &Vec<Vec<i32>>) -> i32 {
    // loop from -1 to 1 y
    // loop from -1 to 1 x
    // just minus self
    let mut count = 0;
    for xDelta in -1..=1 {
        for yDelta in -1..=1 {
            if (x + xDelta < 0 || x + xDelta > (input.len()-1) as i32) {continue};
            if (y + yDelta < 0 || y + yDelta > (input[0].len()-1) as i32) {continue};
            count += input[(x + xDelta) as usize][(y + yDelta) as usize];
        }
    }
    count -= 1; // ignore self
    return match count {
        0..=3 => 1, // between 0 and 3 neighbors inclusive
        _ => 0
    };
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    // nesting af
    let mut grid:Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let mut row:Vec<i32> = Vec::new();
        for item in line.chars(){
            row.push(match item {
                '@' => 1,
                '.' => 0,
                _ => panic!("Illegal character in input")
            })
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use std::{ops::Index, vec};

    use super::*; // Import items from parent module

    #[test]
    fn test_parse_input() {
        let grid = "@....@@@.@.@@.@\n@..@@@.@@@.@.@@";
        let output = vec![vec![1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1], vec![1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1]];
        assert_eq!(parse_input(grid), output);
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
