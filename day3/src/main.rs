use std::cmp::min;
use std::io::BufRead;
use std::{env, fmt, fs::File, io, path::Path, process};

static RADIX: u32 = 10;

struct SchematicSymbol {
    symbol: Option<char>,
    number: Option<u32>,
    is_part: bool,
    counted: bool
}

impl SchematicSymbol {
    fn new() -> Self {
        Self { symbol: None, number: None, is_part: false, counted: false }
    }
}

impl From<char> for SchematicSymbol {
    fn from(symbol: char) -> Self {
        match symbol {
            '.' => Self::new(),
            symbol if symbol.is_digit(RADIX) => Self { 
                symbol: None, number: symbol.to_digit(RADIX), is_part: false, counted: false },
            _ => Self { symbol: Some(symbol), number: None, is_part: false, counted: false }
        }
    }
}

impl From<u32> for SchematicSymbol {
    fn from(number: u32) -> Self {
        Self { symbol: None, number: Some(number), is_part: false, counted: false }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = Path::new(&args[1]);
    if !input_path.exists() {
        eprintln!("Input file does not exist: {}", input_path.display());
        exit_with_usage();
    }

    let result = match args.len() {
        2 => sum_of_parts(input_path),
        _ => Err(SchematicError::from("Invalid number of arguments"))
    };
    match result {
        Ok(sum) => println!("{sum}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

fn sum_of_parts(input_file: &Path) -> Result<u32, SchematicError> {
    let mut sum: u32 = 0;

    let mut empty_symbol: SchematicSymbol = SchematicSymbol::new();
    let mut predecessor_symbols: Vec<SchematicSymbol> = Vec::new();
    let mut predecessor_line:Vec<&SchematicSymbol> = Vec::new();
    let mut current_symbols: Vec<SchematicSymbol> = Vec::new();
    let mut current_line: Vec<&SchematicSymbol> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(SchematicError::from("Could not read line."))
            };

            let mut predecessor_symbol: &mut SchematicSymbol = &mut empty_symbol;
            for (i, c) in line.chars().enumerate() {
                if c == '.' {
                    current_symbols.push(SchematicSymbol::new());
                    predecessor_symbol = current_symbols.last_mut().unwrap();
                    current_line.push(predecessor_symbol);
                } else if c.is_digit(RADIX) {
                    let new_digit = c.to_digit(RADIX)
                            .ok_or(SchematicError::from("Could not read digit."))?;
                    if let Some(number) = predecessor_symbol.number {
                        predecessor_symbol.number = Some(number * 10 + new_digit);
                    } else {
                        let mut new_symbol = SchematicSymbol::from(new_digit);
                        if predecessor_symbol.symbol.is_some() {
                            new_symbol.is_part = true;
                        }

                        current_symbols.push(new_symbol);
                        predecessor_symbol = current_symbols.last_mut().unwrap();
                    }
                    let left = if i == 0 {0} else {i - 1};
                    for j in left..min(predecessor_symbols.len(), i+1) {
                        if let Some(symbol) = predecessor_symbols.get(j) {
                            if symbol.symbol.is_some() {
                                predecessor_symbol.is_part = true;
                            }
                        }
                    }
                } else {
                    let new_symbol = SchematicSymbol::from(c);
                    if predecessor_symbol.number.is_some() {
                        predecessor_symbol.is_part = true;
                    }
                    let left = if i == 0 {0} else {i - 1};
                    for j in left..min(predecessor_symbols.len(), i+1) {
                        if let Some(symbol) = predecessor_symbols.get_mut(j) {
                            if symbol.number.is_some() {
                                symbol.is_part = true;
                            }
                        }
                    }

                    current_symbols.push(new_symbol);
                    predecessor_symbol = current_symbols.last_mut().unwrap();
                }
            }

            for symbol in predecessor_symbols.iter_mut() {
                if symbol.is_part && !symbol.counted {
                    if let Some(number) = symbol.number {
                        sum += number;
                        symbol.counted = true;
                    }
                }
            }
            predecessor_symbols = current_symbols;
            current_symbols = Vec::new();
        }
    }
    for symbol in predecessor_symbols.iter_mut() {
        if symbol.is_part && !symbol.counted {
            if let Some(number) = symbol.number {
                sum += number;
                symbol.counted = true;
            }
        }
    }

    Ok(sum)
}

fn exit_with_usage() {
    println!("Usage: day3 INPUT_FILE");
    process::exit(1);
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct SchematicError {
    message: String
}

impl SchematicError {
    fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl From<&str> for SchematicError {
    fn from(message: &str) -> Self {
        Self::new(String::from(message))
    }
}

impl fmt::Display for SchematicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_parts() {
        let sum = sum_of_parts(Path::new("tests/input.txt")).unwrap();
        assert_eq!(sum, 4361);
    }
}
