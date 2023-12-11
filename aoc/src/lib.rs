use std::io::BufRead;
use std::path::PathBuf;
use std::{env, fs::File, io, path::Path, process};

use errors::AOCError;
pub mod errors;

#[derive(PartialEq)]
pub enum Part {
    One,
    Two
}

pub struct Options {
    pub part: Part,
    pub input: PathBuf
}

pub fn get_args() -> Options {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    if args.len() < 2 || args.len() > 3 || args[1] == "--help" {
        exit_with_usage(program_name);
    }

    let part = if args[1] == "--part2" {Part::Two} else {Part::One};
    if (part == Part::Two && args.len() != 3) || (part == Part::One && args.len() != 2) {
        exit_with_usage(program_name);
    }

    let path_argument_position = if part == Part::One {1} else {2};
    let input_path = PathBuf::from(&args[path_argument_position]);
    if !input_path.exists() {
        eprintln!("Input file does not exist: {}", input_path.display());
        exit_with_usage(&args[0]);
    }

    Options {
        part: part,
        input: input_path
    }
}

pub fn get_input_buffer(path: &PathBuf) -> io::Lines<io::BufReader<File>> {
    if let Ok(lines) = read_lines(path) {
        return lines
    }

    eprintln!("Could read file: {path:?}");
    process::exit(1);
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn exit_with_error(error: AOCError) {
    eprintln!("{error}");
    process::exit(1);
}

fn exit_with_usage(program_name: &str) {
    println!("Usage: [--part2] {program_name} INPUT_FILE");
    process::exit(1);
}
