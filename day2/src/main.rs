use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Path to the local input file
    let file_path = "./day2/src/input.txt";

    // Check if the file exists before trying to read it
    if !Path::new(file_path).exists() {
        eprintln!("Error: File '{}' not found", file_path);
        return Ok(());
    }

    // Attempt to read lines from the file
    let lines = read_lines(file_path)?;
    let mut powers = Vec::new();

    for line in lines.flatten() {
        if let Some((game_id, power)) = calculate_minimum_power(&line) {
            powers.push((game_id, power));
        }
    }

    let sum_of_powers: i32 = powers.iter().map(|&(_, power)| power).sum();
    println!("The sum of all game powers is: {}", sum_of_powers);

    Ok(())
}

// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function to calculate the minimum power needed for a game
fn calculate_minimum_power(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split(": ").collect();
    if parts.len() != 2 {
        return None;
    }

    let game_id: i32 = parts[0].trim_start_matches("Game ").parse().ok()?;
    let rounds: Vec<&str> = parts[1].split("; ").collect();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for round in rounds {
        let cubes: Vec<&str> = round.split(", ").collect();
        for cube in cubes {
            let (count, color) = parse_cube(cube)?;
            match color {
                "red" => max_red = max_red.max(count),
                "green" => max_green = max_green.max(count),
                "blue" => max_blue = max_blue.max(count),
                _ => (),
            }
        }
    }

    let power = max_red * max_green * max_blue;
    Some((game_id, power))
}

// Function to parse the count and color from a cube string
fn parse_cube(cube: &str) -> Option<(i32, &str)> {
    let parts: Vec<&str> = cube.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }

    let count: i32 = parts[0].parse().ok()?;
    let color: &str = parts[1];
    Some((count, color))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_minimum_power() {
        assert_eq!(calculate_minimum_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), Some((1, 48)));
        assert_eq!(calculate_minimum_power("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), Some((2, 12)));
        assert_eq!(calculate_minimum_power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), Some((3, 1560)));
        assert_eq!(calculate_minimum_power("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), Some((4, 630)));
        assert_eq!(calculate_minimum_power("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), Some((5, 36)));
    }

    #[test]
    fn test_parse_cube() {
        assert_eq!(parse_cube("3 blue"), Some((3, "blue")));
        assert_eq!(parse_cube("4 red"), Some((4, "red")));
        assert_eq!(parse_cube("2 green"), Some((2, "green")));
        assert_eq!(parse_cube("invalid cube"), None);
        assert_eq!(parse_cube("3blue"), None);
    }
}