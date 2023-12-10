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

    let result: Result<i64, SensorError> = match args.len() {
        2 => part1(&input_path),
        3 => part2(&input_path),
        _ => Err(SensorError::from("Invalid number of arguments."))
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

fn part1(input_file: &Path) -> Result<i64, SensorError> {
    let mut sum: i64 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(line) = line {
                let values: Vec<i64> = line.split_ascii_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
                sum += predict_next_value(values);
            }
        }
    }
    Ok(sum)
}

fn part2(input_file: &Path) -> Result<i64, SensorError> {
    let mut sum: i64 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(line) = line {
                let values: Vec<i64> = line.split_ascii_whitespace()
                    .rev()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
                sum += predict_next_value(values);
            }
        }
    }
    Ok(sum)
}

fn predict_next_value(values: Vec<i64>) -> i64 {
    if values.iter().all(|x| *x == 0i64) {
        return 0;
    }

    let mut differences: Vec<i64> = Vec::new();
    for window in values.windows(2) {
        differences.push(window[1] - window[0]);
    }

    values.last().unwrap() + predict_next_value(differences)
}

fn exit_with_usage() {
    println!("Usage: day9 INPUT_FILE");
    println!("Part2: day9 INPUT_FILE part2");
    process::exit(1);
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct SensorError {
    message: String,
}

impl SensorError {
    fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl From<&str> for SensorError {
    fn from(message: &str) -> Self {
        Self::new(String::from(message))
    }
}

impl fmt::Display for SensorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let sum = part1(Path::new("tests/input.txt")).unwrap();
        assert_eq!(sum, 114);
    }
}
