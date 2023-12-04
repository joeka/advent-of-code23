use std::collections::VecDeque;
use std::io::BufRead;
use std::{env, fmt, fs::File, io, path::Path, process};

static RADIX: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = Path::new(&args[1]);
    if !input_path.exists() {
        eprintln!("Input file does not exist: {}", input_path.display());
        exit_with_usage();
    }

    let result = match args.len() {
        2 => sum_of_parts(input_path),
        _ => Err(SchematicError::from("Invalid number of arguments"))
    };
    match result {
        Ok(sum) => println!("{sum}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

fn sum_of_parts(input_file: &Path) -> Result<u64, SchematicError> {
    let mut sum: u64 = 0;

    if let Ok(mut lines) = read_lines(input_file) {
        let mut window: VecDeque<Vec<char>> = VecDeque::with_capacity(3);
        let line = match lines.next() {
            None => return Err(SchematicError::from("Could not read line.")),
            Some(line) => match line {
                Ok(line) => line,
                Err(_) => return Err(SchematicError::from("Could not read line."))
            }
        };
        window.push_back(line.chars().collect());
        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(SchematicError::from("Could not read line."))
            };
            window.push_back(line.chars().collect());
            sum += parse_line(&window)?;
        }
        let tmp = window.pop_front()
            .ok_or(SchematicError::from("Could not parse last line."))?;
        window.push_back(tmp);
        sum += parse_line(&window)?;
    }

    Ok(sum)
}

#[derive(PartialEq, Eq)]
enum ParseState {
    Searching,
    Symbol,
    Number,
    PartNumber
}

fn parse_line(window: &VecDeque<Vec<char>>) -> Result<u64, SchematicError> {
    let mut current_sum: u64 = 0;
    let mut current_number: u64 = 0;
    let mut start: usize = 0;

    let mut state = ParseState::Searching;

    let center = window.len() - 2;

    let current_line = &window[center];
    let line_len = current_line.len();
    for (i, c) in current_line.iter().enumerate() {
        if c == &'.' {
            match state {
                ParseState::PartNumber => {
                    current_sum += current_number;
                    current_number = 0;
                },
                ParseState::Number => {
                    let left = if start > 0 {start - 1} else {0};
                    let right = if i + 1 < line_len {i + 1} else {line_len};
                    if symbol_in_range(&window[center + 1], left, right) ||
                            (center > 0 && symbol_in_range(&window[center - 1], left, right)) {
                        current_sum += current_number;
                    }
                    current_number = 0;
                },
                _ => ()
            }
            state = ParseState::Searching;
            
        } else if c.is_digit(RADIX) {
            let new_digit = c.to_digit(RADIX)
                    .ok_or(SchematicError::from("Could not read digit."))? as u64;
            current_number = current_number * 10 + new_digit;
            
            if state == ParseState::Searching || state == ParseState::Symbol {
                start = i;
            }
            state = match state {
                ParseState::Symbol => ParseState::PartNumber,
                ParseState::PartNumber => ParseState::PartNumber,
                _ => ParseState::Number
            };
        } else {
            if state == ParseState::Number || state == ParseState::PartNumber {
                current_sum += current_number;
                current_number = 0;
            }
            state = ParseState::Symbol;
        }
    }
    match state {
        ParseState::PartNumber => {
            current_sum += current_number;
        },
        ParseState::Number => {
            let left = if start > 0 {start - 1} else {0};
            if symbol_in_range(&window[center + 1], left, line_len) ||
                    (center > 0 && symbol_in_range(&window[center - 1], left, line_len)) {
                current_sum += current_number;
            }
        },
        _ => ()
    }
    Ok(current_sum)
}

fn symbol_in_range(line: &Vec<char>, start: usize, end: usize) -> bool {
    for c in &line[start..end] {
        match c {
            '0'..='9' | '.' => (),
            _ => return true
        }
    }
    false
}

fn exit_with_usage() {
    println!("Usage: day3 INPUT_FILE");
    process::exit(1);
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct SchematicError {
    message: String
}

impl SchematicError {
    fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl From<&str> for SchematicError {
    fn from(message: &str) -> Self {
        Self::new(String::from(message))
    }
}

impl fmt::Display for SchematicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_parts() {
        let sum = sum_of_parts(Path::new("tests/input.txt")).unwrap();
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_sum_of_parts3() {
        let sum = sum_of_parts(Path::new("tests/input3.txt")).unwrap();
        assert_eq!(sum, 333);
    }

    #[test]
    fn test_sum_of_parts2() {
        let sum = sum_of_parts(Path::new("tests/input2.txt")).unwrap();
        assert_eq!(sum, 3306);
    }

    #[test]
    fn test_parse_line() {
        let mut window: VecDeque<Vec<char>> = VecDeque::with_capacity(3);
        window.push_back(".*...".chars().collect());
        window.push_back(".12.3".chars().collect());
        window.push_back(".....".chars().collect());

        assert_eq!(parse_line(&window).unwrap(), 12);
    }

    #[test]
    fn test_parse_start_line() {
        let mut window: VecDeque<Vec<char>> = VecDeque::with_capacity(3);
        window.push_back(".52.3".chars().collect());
        window.push_back("#....".chars().collect());

        assert_eq!(parse_line(&window).unwrap(), 52);
    }

    #[test]
    fn test_parse_example_lines() {
        let mut window: VecDeque<Vec<char>> = VecDeque::with_capacity(3);
        window.push_back("467..114..".chars().collect());
        window.push_back("...*......".chars().collect());
        assert_eq!(parse_line(&window).unwrap(), 467);

        window.push_back("..35..633.".chars().collect());
        window.push_back("......#...".chars().collect());
        assert_eq!(parse_line(&window).unwrap(), 668);

        window.push_back("617*......".chars().collect());
        window.push_back(".....+.58.".chars().collect());
        assert_eq!(parse_line(&window).unwrap(), 617);

        window.push_back("..592.....".chars().collect());
        assert_eq!(parse_line(&window).unwrap(), 0);

        window.push_back("......755.".chars().collect());
        assert_eq!(parse_line(&window).unwrap(), 592);

        window.push_back("...$.*....".chars().collect());
        assert_eq!(parse_line(&window).unwrap(), 755);

        window.push_back(".664.598..".chars().collect());
        window.push_back("..........".chars().collect());
        assert_eq!(parse_line(&window).unwrap(), 1262);
    }

    #[test]
    fn test_symbol_in_range() {
        assert!(symbol_in_range(&"..12.#.".chars().collect(), 4, 6));
        assert!(!symbol_in_range(&"..12.#.".chars().collect(), 0, 4));
    }
}
