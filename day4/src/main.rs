use std::collections::{HashSet, VecDeque};
use std::io::BufRead;
use std::path::PathBuf;
use std::{fs::File, io, path::Path};

use aoc::errors::AOCError;
use aoc::{get_args, Part, exit_with_error, get_input_buffer};

fn main() {
    let options = get_args();

    let result: Result<u32, AOCError> = match options.part {
        Part::One => check_cards(&options.input),
        Part::Two => part2(&options.input)
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => exit_with_error(error)
    }
}

fn part2(input_file: &Path) -> Result<u32, AOCError> {
    let mut sum: u32 = 0;
    let mut factors: VecDeque<u32> = VecDeque::new();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(AOCError::from("Could not read line."))
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

fn check_cards(input_file: &PathBuf) -> Result<u32, AOCError> {
    let mut sum: u32 = 0;
    for line in get_input_buffer(input_file) {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err(AOCError::from("Could not read line."))
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
    Ok(sum)
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let sum = part2(&PathBuf::from("tests/input.txt")).unwrap();
        assert_eq!(sum, 30);
    }

    #[test]
    fn test_check_cards() {
        let sum = check_cards(&PathBuf::from("tests/input.txt")).unwrap();
        assert_eq!(sum, 13);
    }
}
