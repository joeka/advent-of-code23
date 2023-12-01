use std::{fmt, env, process};
use std::{path::Path, fs::File, io};
use std::io::BufRead;

static RADIX: u32 = 10;
static NUMBERS: &'static [&'static str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

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
    let first = find_first(&line);
    let last: Option<u32> = find_second(&line[first.1+1..]);
    return first.0 * 10 + last.unwrap_or(first.0);
}

fn find_first(line: &str) -> (u32, usize){
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(RADIX) {
            return (c.to_digit(RADIX).unwrap(), i);
        }
        for length in 3..6 {
            let end = i + 1;
            if end >= length {
                let value = check_substring(&line, end, length);
                if value.is_some() {
                    return (value.unwrap(), i);
                }
            } else {
                break;
            }
        }
    }
    panic!("Couldn't find first number!")
}

fn find_second(line: &str) -> Option<u32> {
    let mut last: Option<u32> = None;
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(RADIX) {
            last = c.to_digit(RADIX);
            continue;
        }
        for length in 3..6 {
            let end = i + 1;
            if end >= length {
                let value =  check_substring(&line, end, length);
                if value.is_some() {
                    last = value;
                    break;
                }
            }
        }
    }
    return last;
}

fn check_substring(line: &str, end: usize, length: usize) -> Option<u32> {
    let substring = &line[(end - length)..end];
    let pos = NUMBERS.iter()
        .position(|number| number == &substring);
    match pos {
        Some(pos) => Some((pos + 1) as u32),
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
