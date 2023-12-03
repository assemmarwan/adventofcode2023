use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() {
    let lines = filename_to_string("./src/days/day1/day1_input.txt").unwrap();

    let sum: i32 = lines.into_iter().map(|l| extract_number(l)).sum();

    println!("The sum is {sum}");
}

fn filename_to_string(s: &str) -> io::Result<Vec<String>> {
    let file = File::open(s)?;

    let buffer = BufReader::new(file);

    let lines_buf = buffer.lines();

    let lines: Vec<String> = lines_buf.map(|l| l.expect("Failed to parse")).collect();

    return Ok(lines);
}

fn extract_number(code: String) -> i32 {
    let mut numbers_in_string: Vec<i32> = vec![];
    for char in code.chars() {
        let parsed = char.to_string().parse::<i32>();
        let result = match parsed {
            Ok(digit) => digit,
            Err(_) => -1,
        };

        if result == -1 {
            continue;
        }

        if numbers_in_string.is_empty() || numbers_in_string.len() == 1 {
            numbers_in_string.push(result);
        }

        if numbers_in_string.len() == 2 {
            numbers_in_string.remove(numbers_in_string.len() - 1);
            numbers_in_string.push(result);
        }
    }
    let _result = (numbers_in_string.first().unwrap() * 10) + numbers_in_string.last().unwrap();

    return _result;
}
