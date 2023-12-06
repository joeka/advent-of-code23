use std::{env, path::Path, process, fs::File, io::{self, BufRead}, error};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

    match args.len() {
        2 => println!("{}", part1(&input_path)),
        3 => println!("{}", part2(&input_path)),
        _ => exit_with_usage()
    };
}

fn race(time: u64, distance: u64) -> u64 {
    let mut possibilities = 0;
    for v in 1..time {
        if v * (time - v) > distance {
            possibilities += 1;
        }
    }
    possibilities
}

fn part1(input_path: &Path) -> u64 {
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

fn part2(input_path: &Path) -> u64 {
    let lines = get_input(input_path);
    let inputs: Vec<u64> = lines.iter().map(|s| s.replace(' ', ""))
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    race(inputs[0], inputs[1])
}

fn get_input(input_path: &Path) -> Vec<String> {
    read_lines(input_path).unwrap()
        .take(2)
        .map(io::Result::unwrap)
        .map(|s| s.split_once(':').unwrap().1.to_owned())
        .collect()

}

fn exit_with_usage() {
    println!("Usage: day6 INPUT_FILE");
    println!("Part2: day6 INPUT_FILE part2");
    process::exit(1);
}

fn read_lines(path: &Path) -> Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(Path::new("tests/input.txt"));
        assert_eq!(result, 71503);
    }

    #[test]
    fn test_part1() {
        let result = part1(Path::new("tests/input.txt"));
        assert_eq!(result, 288);
    }
}
