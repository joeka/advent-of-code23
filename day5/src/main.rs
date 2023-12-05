use std::io::BufRead;
use std::{env, fmt, fs::File, io, path::Path, process};

static RADIX: u32 = 10;

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
        2 => nearest_location(&input_path),
        3 => part2(&input_path),
        _ => Err(GardeningError::from("Invalid number of arguments."))
    };

    match result {
        Ok(sum) => println!("{sum}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

fn nearest_location(input_file: &Path) -> Result<i64, GardeningError> {
    let seeds: Vec<i64>;
    let mut operations: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut nearest: i64 = i64::MAX;

    if let Ok(mut lines) = read_lines(input_file) {
        let line = lines.next().unwrap();
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err(GardeningError::from("Could not read line.")),
        };
        seeds = line
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(str::parse::<i64>)
            .filter_map(Result::ok)
            .collect();

        let mut current_operation: Vec<(i64, i64, i64)> = Vec::new();

        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(GardeningError::from("Could not read line.")),
            };

            if !line.chars().next().is_some_and(|c| c.is_digit(RADIX)) {
                if current_operation.len() > 0 {
                    operations.push(current_operation);
                    current_operation = Vec::new();
                }
            } else {
                let parts: Vec<i64> = line
                    .split_whitespace()
                    .map(str::parse::<i64>)
                    .filter_map(Result::ok)
                    .collect();
                let (destination, source, length) = (parts[0], parts[1], parts[2]);
                let upper_bound = source + length;
                let shift = destination - source;

                current_operation.push((source, upper_bound, shift));
            }
        }

        if current_operation.len() > 0 {
            operations.push(current_operation);
        }

        for seed in seeds {
            let mut value = seed;
            for operation in &operations {
                value = map(value, &operation);
            }
            if value < nearest {
                nearest = value;
            }
        }
    }
    Ok(nearest)
}

fn part2(input_file: &Path) -> Result<i64, GardeningError> {
    let seeds: Vec<(i64, i64)>;
    let mut operations: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut nearest: i64 = i64::MAX;

    if let Ok(mut lines) = read_lines(input_file) {
        let line = lines.next().unwrap();
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err(GardeningError::from("Could not read line.")),
        };
        seeds = line
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(str::parse::<i64>)
            .filter_map(Result::ok)
            .collect::<Vec<i64>>()
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
            .collect();

        let mut current_operation: Vec<(i64, i64, i64)> = Vec::new();

        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(GardeningError::from("Could not read line.")),
            };

            if !line.chars().next().is_some_and(|c| c.is_digit(RADIX)) {
                if current_operation.len() > 0 {
                    operations.push(current_operation);
                    current_operation = Vec::new();
                }
            } else {
                let parts: Vec<i64> = line
                    .split_whitespace()
                    .map(str::parse::<i64>)
                    .filter_map(Result::ok)
                    .collect();
                let (destination, source, length) = (parts[0], parts[1], parts[2]);
                let upper_bound = source + length;
                let shift = destination - source;

                current_operation.push((source, upper_bound, shift));
            }
        }

        if current_operation.len() > 0 {
            operations.push(current_operation);
        }

        for seed_range in seeds {
            for seed in seed_range.0..seed_range.1 {
                let mut value = seed;
                for operation in &operations {
                    value = map(value, &operation);
                }
                if value < nearest {
                    nearest = value;
                }
            }
        }
    }
    Ok(nearest)
}

fn map(input: i64, operation: &Vec<(i64, i64, i64)>) -> i64 {
    for mapping in operation {
        if input >= mapping.0 && input < mapping.1 {
            return input + mapping.2;
        }
    }
    input
}

fn exit_with_usage() {
    println!("Usage: day5 INPUT_FILE");
    println!("Part2: day5 INPUT_FILE part2");
    process::exit(1);
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct GardeningError {
    message: String,
}

impl GardeningError {
    fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl From<&str> for GardeningError {
    fn from(message: &str) -> Self {
        Self::new(String::from(message))
    }
}

impl fmt::Display for GardeningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let location = part2(Path::new("tests/input.txt")).unwrap();
        assert_eq!(location, 46);
    }

    #[test]
    fn test_nearest_location() {
        let location = nearest_location(Path::new("tests/input.txt")).unwrap();
        assert_eq!(location, 35);
    }
}
