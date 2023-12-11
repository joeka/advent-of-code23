use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use aoc::{Part, get_args, exit_with_error, get_input_buffer};
use aoc::errors::AOCError;
use num::integer::lcm;

fn main() {
    let options = get_args();

    let result: Result<u64, AOCError> = match options.part {
        Part::One => get_out(&options.input),
        Part::Two => part2(&options.input)
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => exit_with_error(error)
    }
}

fn part2(input_file: &PathBuf) -> Result<u64, AOCError> {
    let mut steps: u64 = 0;
    let mut current: Vec<String> = Vec::new();
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let mut lines = get_input_buffer(input_file);

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
    Ok(steps)
}

fn get_out(input_file: &PathBuf) -> Result<u64, AOCError> {
    let mut steps: u64 = 0;
    let mut graph: HashMap<String, (String, String)> = HashMap::new();

    let mut lines = get_input_buffer(input_file);

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
   
    Err(AOCError::from("Something went wrong"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let steps = part2(&PathBuf::from("tests/part2.txt")).unwrap();
        assert_eq!(steps, 6);
    }

    #[test]
    fn test_example1() {
        let steps = get_out(&PathBuf::from("tests/input1.txt")).unwrap();
        assert_eq!(steps, 2);
    }

    #[test]
    fn test_example2() {
        let steps = get_out(&PathBuf::from("tests/input2.txt")).unwrap();
        assert_eq!(steps, 6);
    }
}
