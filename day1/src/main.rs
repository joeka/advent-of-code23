use std::collections::HashMap;
use std::sync::OnceLock;
use std::{fmt, env, process};
use std::{path::Path, fs::File, io};
use std::io::BufRead;

static RADIX: u32 = 10;

fn number_lookup() -> &'static HashMap<&'static str, u32> {
    static HASHMAP: OnceLock<HashMap<&str, u32>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map: HashMap<&str, u32> = HashMap::new();
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
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} INPUT_FILE", args[0]);
        process::exit(1);
    }
    match sum_of_calibration_values(Path::new(&args[1])) {
        Ok(sum) => println!("{sum}"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}

fn sum_of_calibration_values(input_file: &Path) -> Result<u32, CalibrationError> {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            match line {
                Ok(line) => sum += extract_number(line),
                Err(_) => return Err(CalibrationError)
            }
        }
    }
    Ok(sum)
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_number(line: String) -> u32 {
    let numbers = find_numbers(&line);
    return numbers.first().unwrap() * 10 + numbers.last().unwrap();
}

fn find_numbers(line: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(RADIX) {
            numbers.push(c.to_digit(RADIX).unwrap());
            continue;
        }
        for length in 3..6 {
            let end = i + 1;
            if end >= length {
                let value =  check_substring(&line, end, length);
                if value.is_some() {
                    numbers.push(value.unwrap());
                    break;
                }
            }
        }
    }
    return numbers;
}

fn check_substring(line: &str, end: usize, length: usize) -> Option<u32> {
    let substring = &line[(end - length)..end];
    let number = number_lookup().get(substring);
    match number {
        Some(pos) => Some(*pos),
        None => None
    }
}

#[derive(Debug, Clone)]
struct CalibrationError;

impl fmt::Display for CalibrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "calibration failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_calibration_values2() {
        let result = sum_of_calibration_values(Path::new("./tests/input2.txt"));
        assert_eq!(result.unwrap(), 281);
    }

    #[test]
    fn test_extract_number2() {
        assert_eq!(extract_number(String::from("two1nine")), 29);
    }

    #[test]
    fn test_sum_of_calibration_values() {
        let result = sum_of_calibration_values(Path::new("./tests/input.txt"));
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_read_lines() {
        let file_contains_expected_line = read_lines(&Path::new("./tests/input.txt"))
            .expect("file should contain lines")
            .any(|line| line.unwrap() == "a1b2c3d4e5f");
        assert!(file_contains_expected_line);
    }

    #[test]
    fn test_extract_number() {
        assert_eq!(extract_number(String::from("1abc2")), 12);
        assert_eq!(extract_number(String::from("pqr3stu8vwx")), 38);
        assert_eq!(extract_number(String::from("a1b2c3d4e5f")), 15);
        assert_eq!(extract_number(String::from("treb7uchet")), 77);
    }
}
