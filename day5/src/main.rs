use std::path::PathBuf;

use aoc::errors::AOCError;
use aoc::{Part, get_args, exit_with_error, get_input_buffer};

static RADIX: u32 = 10;

fn main() {
    let options = get_args();

    let result: Result<i64, AOCError> = match options.part {
        Part::One => nearest_location(&options.input),
        Part::Two => part2(&options.input)
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => exit_with_error(error)
    }
}

fn nearest_location(input_file: &PathBuf) -> Result<i64, AOCError> {
    let seeds: Vec<i64>;
    let mut operations: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut nearest: i64 = i64::MAX;

    let mut lines = get_input_buffer(input_file);
    let line = lines.next().unwrap();
    let line = match line {
        Ok(line) => line,
        Err(_) => return Err(AOCError::from("Could not read line.")),
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
            Err(_) => return Err(AOCError::from("Could not read line.")),
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
    Ok(nearest)
}

fn part2(input_file: &PathBuf) -> Result<i64, AOCError> {
    let seeds: Vec<(i64, i64)>;
    let mut operations: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut nearest: i64 = i64::MAX;

    let mut lines = get_input_buffer(input_file);
    {
        let line = lines.next().unwrap();
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err(AOCError::from("Could not read line.")),
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
                Err(_) => return Err(AOCError::from("Could not read line.")),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let location = part2(&PathBuf::from("tests/input.txt")).unwrap();
        assert_eq!(location, 46);
    }

    #[test]
    fn test_nearest_location() {
        let location = nearest_location(&PathBuf::from("tests/input.txt")).unwrap();
        assert_eq!(location, 35);
    }
}
