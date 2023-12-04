use std::collections::HashSet;
use std::io::BufRead;
use std::{env, fmt, fs::File, io, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit_with_usage();
    }

    let input_path = Path::new(&args[1]);
    if !input_path.exists() {
        eprintln!("Input file does not exist: {}", input_path.display());
        exit_with_usage();
    }

    let result = check_cards(&input_path);
    match result {
        Ok(sum) => println!("{sum}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

fn check_cards(input_file: &Path) -> Result<u32, CardError> {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(CardError::from("Could not read line."))
            };
            let content = line.split_once(':').unwrap().1;
            let (winning, own) = content.split_once('|').unwrap();
            let winning_numbers: HashSet<u32> = winning.split_whitespace()
                .map(str::parse::<u32>)
                .filter_map(Result::ok)
                .collect();
            let own_numbers: HashSet<u32> = own.split_whitespace()
                .map(str::parse::<u32>)
                .filter_map(Result::ok)
                .collect();
            let count = winning_numbers.intersection(&own_numbers).count();
            if count > 0 {
                sum += 1 << count - 1;
            }
        }
    }
    Ok(sum)
}

fn exit_with_usage() {
    println!("Usage: day4 INPUT_FILE");
    process::exit(1);
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct CardError {
    message: String,
}

impl CardError {
    fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl From<&str> for CardError {
    fn from(message: &str) -> Self {
        Self::new(String::from(message))
    }
}

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_cards() {
        let sum = check_cards(Path::new("tests/input.txt")).unwrap();
        assert_eq!(sum, 13);
    }
}
