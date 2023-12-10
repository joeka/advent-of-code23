use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::{env, fmt, fs::File, io, path::Path, process};
use num::integer::lcm;

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

    let result: Result<u64, GhostError> = match args.len() {
        2 => get_out(&input_path),
        3 => part2(&input_path),
        _ => Err(GhostError::from("Invalid number of arguments."))
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

fn part2(input_file: &Path) -> Result<u64, GhostError> {
    let mut steps: u64 = 0;
    let mut current: Vec<String> = Vec::new();
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    if let Ok(mut lines) = read_lines(input_file) {
        let directions: Vec<char> = lines.next().unwrap().unwrap()
            .chars()
            .collect();
        lines.next();

        for line in lines {
            if let Ok(line) = line {
                let (mut node, rest) = line.split_once('=').unwrap();
                node = node.trim();
                let choices: (&str, &str) = rest[2..(rest.len()-1)].split_once(", ").unwrap();
                graph.insert(node.to_owned(), (choices.0.to_owned(), choices.1.to_owned()));
                
                if node.chars().last().unwrap() == 'A' {
                    current.push(node.to_owned());
                }
            }
        }

        let mut ghost_steps: HashSet<u64> = HashSet::new();

        for direction in directions.iter().cycle() {
            steps += 1;
            let mut i: usize = 0;
            while i < current.len() {
                let node_id = current.get(i).unwrap().clone();
                let node = graph.get(&node_id).unwrap();
                let next_node = if *direction == 'L' {node.0.clone()} else {node.1.clone()};
                
                if next_node.chars().last().unwrap() == 'Z' {
                    ghost_steps.insert(steps);
                    current.remove(i);
                } else {
                    current[i] = next_node;
                    i += 1;
                }
            }   

            if current.is_empty() {
                break;
            }    
        }

        let mut iter = ghost_steps.iter();
        steps = *iter.next().unwrap();

        for ghost in ghost_steps {
            steps = lcm(steps, ghost);
        }
        return Ok(steps)
    }
    
   
    Err(GhostError::from("Something went wrong"))
}

fn get_out(input_file: &Path) -> Result<u64, GhostError> {
    let mut steps: u64 = 0;
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    if let Ok(mut lines) = read_lines(input_file) {
        let directions: Vec<char> = lines.next().unwrap().unwrap()
            .chars()
            .collect();
        lines.next();

        for line in lines {
            if let Ok(line) = line {
                let (mut node, rest) = line.split_once('=').unwrap();
                node = node.trim();
                let choices: (&str, &str) = rest[2..(rest.len()-1)].split_once(", ").unwrap();
                graph.insert(node.to_owned(), (choices.0.to_owned(), choices.1.to_owned()));
            }
        }

        let mut current = "AAA";
        for direction in directions.iter().cycle() {
            let node = graph.get(current).unwrap();
            current = if *direction == 'L' {&node.0} else {&node.1};
            steps += 1;

            if current == "ZZZ" {
                return Ok(steps);
            }    
        }
    }
   
    Err(GhostError::from("Something went wrong"))
}

fn exit_with_usage() {
    println!("Usage: day8 INPUT_FILE");
    println!("Part2: day8 INPUT_FILE part2");
    process::exit(1);
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct GhostError {
    message: String,
}

impl GhostError {
    fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl From<&str> for GhostError {
    fn from(message: &str) -> Self {
        Self::new(String::from(message))
    }
}

impl fmt::Display for GhostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let steps = part2(Path::new("tests/part2.txt")).unwrap();
        assert_eq!(steps, 6);
    }

    #[test]
    fn test_example1() {
        let steps = get_out(Path::new("tests/input1.txt")).unwrap();
        assert_eq!(steps, 2);
    }

    #[test]
    fn test_example2() {
        let steps = get_out(Path::new("tests/input2.txt")).unwrap();
        assert_eq!(steps, 6);
    }
}
