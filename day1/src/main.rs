use std::{fmt, env, process};
use std::{path::Path, fs::File, io};
use std::io::BufRead;

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
    const RADIX: u32 = 10;
    let mut digits = line.chars()
        .filter(|c| c.is_digit(RADIX));
    let mut last = digits.next()
        .expect("there should be at least one digit")
        .to_digit(RADIX)
        .expect("digits should be converted to u32");
    let first = 10 * last;

    for c in digits {
        last = c.to_digit(RADIX).expect("digits should be converted to u32");
    }

    return first + last;
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
