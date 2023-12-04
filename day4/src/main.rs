use std::collections::{HashSet, VecDeque};
use std::io::BufRead;
use std::{env, fmt, fs::File, io, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        exit_with_usage();
    }

    let input_path = Path::new(&args[1]);
    if !input_path.exists() {
        eprintln!("Input file does not exist: {}", input_path.display());
        exit_with_usage();
    }

    let result = match args.len() {
        2 => check_cards(&input_path),
        3 => part2(&input_path),
        _ => Err(CardError::from("Invalid number of arguments."))
    };

    match result {
        Ok(sum) => println!("{sum}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

fn part2(input_file: &Path) -> Result<u32, CardError> {
    let mut sum: u32 = 0;
    let mut factors: VecDeque<u32> = VecDeque::new();

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

            let current_factor = factors.pop_front().or(Some(1)).unwrap();
            sum += current_factor;

            let count = winning_numbers.intersection(&own_numbers).count();
            if count > 0 {
                if count > factors.len() {
                    for _ in 0..(count - factors.len()) {
                        factors.push_back(1);
                    }
                }
                for _ in 0..current_factor {
                    for (i, factor) in factors.iter_mut().enumerate() {
                        *factor += 1;
                        if i == count - 1 {
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(sum)
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
    println!("Part2: day4 INPUT_FILE part2");
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
    fn test_part2() {
        let sum = part2(Path::new("tests/input.txt")).unwrap();
        assert_eq!(sum, 30);
    }

    #[test]
    fn test_check_cards() {
        let sum = check_cards(Path::new("tests/input.txt")).unwrap();
        assert_eq!(sum, 13);
    }
}
