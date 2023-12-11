use std::{path::PathBuf, io};
use aoc::{get_args, get_input_buffer, Part};

fn main() {
    let options = get_args();

    match options.part {
        Part::One => part1(&options.input),
        Part::Two => part2(&options.input)
    };
}

// v * (t -v) = d
// v^2 - tv + d = 0
// v = (t +- sqrt(t^2 - 4d)) / 2
fn race(time: u64, distance: u64) -> u64 {
    let t = time as f64;
    let d = distance as f64;

    let a = t / 2.0;
    let b = (t.powi(2) - 4.0 * d).sqrt() / 2.0;

    let lower = (a - b + 1.0).floor() as u64;
    let upper = (a + b - 1.0).ceil() as u64;

    upper - lower + 1
}

fn part1(input_path: &PathBuf) -> u64 {
    let lines = get_input(input_path);
    let split_lines: Vec<Vec<u64>> = lines.iter()
        .map(|line| line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect())
        .collect();
    split_lines[0].iter().zip(split_lines[1].iter())
        .map(|(l, r)| race(l.to_owned(), r.to_owned()))
        .product()
}

fn part2(input_path: &PathBuf) -> u64 {
    let lines = get_input(input_path);
    let inputs: Vec<u64> = lines.iter().map(|s| s.replace(' ', ""))
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    race(inputs[0], inputs[1])
}

fn get_input(input_path: &PathBuf) -> Vec<String> {
    get_input_buffer(input_path)
        .take(2)
        .map(io::Result::unwrap)
        .map(|s| s.split_once(':').unwrap().1.to_owned())
        .collect()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(&PathBuf::from("tests/input.txt"));
        assert_eq!(result, 71503);
    }

    #[test]
    fn test_part1() {
        let result = part1(&PathBuf::from("tests/input.txt"));
        assert_eq!(result, 288);
    }
}
