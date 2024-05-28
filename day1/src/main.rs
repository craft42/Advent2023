use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    // Path to the local input file
    let file_path = "./day1/src/input.txt";
    
    match read_lines(file_path) {
        Ok(lines) => {
            let sum: i32 = lines.flatten()
                .filter_map(|line| get_calibration_value(&line)) // Apply get_calibration_value to each line, filtering out None values
                .sum();
            println!("ANSWER : The sum of all calibration values is {}", sum);
        },
        Err(e) => {
            println!("Error reading the file: {}", e);
        }
    }
    Ok(())
}

// Function to read lines from a file
// BufReader uses an internal buffer to reduce intermediate allocations.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function to get the calibration value from a line of text
fn get_calibration_value(line: &str) -> Option<i32> {
    let digit_map = create_digit_map();

    let first_digit = find_first_digit(line, &digit_map)?;
    let last_digit = find_last_digit(line, &digit_map)?;

    // Combine the first and last digits to form a two-digit number
    Some(first_digit * 10 + last_digit)
}

// Create a map of spelled-out digits to their numerical counterparts
fn create_digit_map() -> HashMap<&'static str, i32> {
    let mut map = HashMap::new();
    map.insert("zero", 0);
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map
}

// Find the first digit (numeric or spelled-out) in the line
fn find_first_digit(line: &str, digit_map: &HashMap<&str, i32>) -> Option<i32> {
    let mut chars = line.char_indices().peekable();
    while let Some((index, ch)) = chars.peek() {
        if ch.is_ascii_digit() {
            return ch.to_digit(10).map(|d| d as i32);
        }
        else {
            for (key, &value) in digit_map {
                if line[*index..].starts_with(key) {
                    return Some(value);
                }
            }
        }
        chars.next();
    }
    None
}

// Find the last digit (numeric or spelled-out) in the line
fn find_last_digit(line: &str, digit_map: &HashMap<&str, i32>) -> Option<i32> {
    let mut chars = line.char_indices().rev().peekable();
    while let Some((index, ch)) = chars.peek() {
        if ch.is_ascii_digit() {
            return ch.to_digit(10).map(|d| d as i32);
        } else {
            for (key, &value) in digit_map {
                if line[..=*index].ends_with(key) {
                    return Some(value);
                }
            }
        }
        chars.next();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calibration_value() {
        assert_eq!(get_calibration_value("1abc2"), Some(12));
        assert_eq!(get_calibration_value("pqr3stu8vwx"), Some(38));
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), Some(15));
        assert_eq!(get_calibration_value("treb7uchet"), Some(77));
        assert_eq!(get_calibration_value("no_digits"), None);
        assert_eq!(get_calibration_value("1n0"), Some(10));
        assert_eq!(get_calibration_value("two1nine"), Some(29));
        assert_eq!(get_calibration_value("eightwothree"), Some(83));
        assert_eq!(get_calibration_value("abcone2threexyz"), Some(13));
        assert_eq!(get_calibration_value("xtwone3four"), Some(24));
        assert_eq!(get_calibration_value("4nineeightseven2"), Some(42));
        assert_eq!(get_calibration_value("zoneight234"), Some(14));
       assert_eq!(get_calibration_value("7pqrstsixteen"), Some(76)); 
    }

    #[test]
    fn test_get_calibration_value_empty() {
        assert_eq!(get_calibration_value(""), None);
    }
}
