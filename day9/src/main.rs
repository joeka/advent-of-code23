use std::path::PathBuf;

use aoc::errors::AOCError;
use aoc::{get_input_buffer, get_args, Part, exit_with_error};

fn main() {
    let options = get_args();

    let result: Result<i64, AOCError> = match options.part {
        Part::One => part1(&options.input),
        Part::Two => part2(&options.input)
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => exit_with_error(error)
    }
}

fn part1(input_file: &PathBuf) -> Result<i64, AOCError> {
    let mut sum: i64 = 0;
    for line in get_input_buffer(input_file) {
        if let Ok(line) = line {
            let values: Vec<i64> = line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            sum += predict_next_value(values);
        }
    }
    Ok(sum)
}

fn part2(input_file: &PathBuf) -> Result<i64, AOCError> {
    let mut sum: i64 = 0;
    for line in get_input_buffer(input_file) {
        if let Ok(line) = line {
            let values: Vec<i64> = line.split_ascii_whitespace()
                .rev()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            sum += predict_next_value(values);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let sum = part1(&PathBuf::from("tests/input.txt")).unwrap();
        assert_eq!(sum, 114);
    }
}
