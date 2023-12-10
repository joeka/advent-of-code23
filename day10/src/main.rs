use std::io::BufRead;
use std::{env, fmt, fs::File, io, path::Path, process};

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

    let result: Result<u64, NavigationError> = match args.len() {
        2 => part1(&input_path),
        3 => part2(&input_path),
        _ => Err(NavigationError::from("Invalid number of arguments."))
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

type Coordinate = (usize, usize);
type Node = (Coordinate, Coordinate);

fn part1(input_file: &Path) -> Result<u64, NavigationError> {
    let mut start: Coordinate = (0, 0);
    let mut grid: Vec<Vec<Option<Node>>> = Vec::new();

    if let Ok(lines) = read_lines(input_file) {
        for (y, line) in lines.enumerate() {
            let mut current_line: Vec<Option<Node>> = Vec::new();
            for (x, c) in line.unwrap().chars().enumerate() {
                let node = create_node(&c, x, y);
                if c == 'S' {
                    start = (x, y);
                }
                current_line.push(node);
            }
            grid.push(current_line);
        }
    }

    let start_node = find_starting_directions(&grid, &start);
    let count = count_loop(&start, &start_node.0, &grid);
    Ok(count / 2)
}

fn count_loop(start: &Coordinate, next: &Coordinate, grid: &Vec<Vec<Option<Node>>>) -> u64 {
    let mut count: u64 = 1;
    let mut prev = start;
    let mut current = next;
    while let Some(node) = &grid[current.1][current.0] {
        let tmp = current;
        current = if node.0 == *prev {&node.1} else {&node.0};
        prev = tmp;
        count += 1
    }
    
    count
}

fn find_starting_directions(grid: &Vec<Vec<Option<Node>>>, start: &Coordinate) -> Node {
    let mut possible_coordinates: Vec<Coordinate> = Vec::new();
    let (x, y) = start;

    let y_min = if *y == 0 {0} else {y - 1};
    let y_max = if *y == grid.len() - 1 {*y} else {y + 1};
    let x_min = if *x == 0 {0} else {x - 1};
    let x_max = if *x == grid[0].len() - 1 {*x} else {x + 1};

    for y_s in y_min..=y_max {
        let line = grid.get(y_s).unwrap();
        for x_s in x_min..=x_max {
            if let Some(node) = line.get(x_s).unwrap() {
                if node.0 == *start || node.1 == *start {
                    possible_coordinates.push((x_s, y_s));
                    if possible_coordinates.len() == 2 {
                        return (possible_coordinates[0], possible_coordinates[1]);
                    }
                }
            }
        }
    }
    panic!("Couldn't figure out start directions");
}

fn create_node(c: &char, x: usize, y: usize) -> Option<Node> {
    match c {
        '|' => if y > 0 {Some(((x, y - 1), (x, y + 1)))} else {None}, // is a vertical pipe connecting north and south
        '-' => if x > 0 {Some(((x - 1, y), (x + 1, y)))} else {None}, // is a horizontal pipe connecting east and west
        'L' => if y > 0 {Some(((x, y - 1), (x + 1, y)))} else {None}, // is a 90-degree bend connecting north and east
        'J' => if y > 0 && x > 0 {Some(((x, y - 1), (x - 1, y)))} else {None}, // is a 90-degree bend connecting north and west
        '7' => if x > 0 {Some(((x - 1, y), (x, y + 1)))} else {None}, // is a 90-degree bend connecting south and west
        'F' => Some(((x + 1, y), (x, y + 1))), // is a 90-degree bend connecting south and east
        '.' => None, // is ground; there is no pipe in this tile
        'S' => None, // is the starting position of the animal
        _ => panic!("invalid character {c}")
    }
}

fn part2(_: &Path) -> Result<u64, NavigationError> {
    Err(NavigationError::from("Not implemented"))
}

fn exit_with_usage() {
    println!("Usage: day10 INPUT_FILE");
    println!("Part2: day10 INPUT_FILE part2");
    process::exit(1);
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct NavigationError {
    message: String,
}

impl NavigationError {
    fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl From<&str> for NavigationError {
    fn from(message: &str) -> Self {
        Self::new(String::from(message))
    }
}

impl fmt::Display for NavigationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let distance = part1(Path::new("tests/example1.txt")).unwrap();
        assert_eq!(distance, 4);
    }

    #[test]
    fn test_example2() {
        let distance = part1(Path::new("tests/example2.txt")).unwrap();
        assert_eq!(distance, 8);
    }
}
