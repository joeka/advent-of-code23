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
type Node = (Coordinate, Coordinate, char);


fn part2(input_file: &Path) -> Result<u64, NavigationError> {
    let (
        start,
        start_node,
        grid) = parse_grid(input_file);
    Ok(calculate_area_in_loop((start.0, start.1, start_node.2), &start_node.0, &grid))
}

fn part1(input_file: &Path) -> Result<u64, NavigationError> {
    let (
        start,
        start_node,
        grid) = parse_grid(input_file);
    let count = count_loop(&start, &start_node.0, &grid);
    Ok(count / 2)
}

fn parse_grid(input_file: &Path) -> (Coordinate, Node, Vec<Vec<Option<Node>>>) {
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
    let starting_directions = find_starting_directions(&grid, &start);
    let start_node = define_start_node(&start, starting_directions);
    
    (start, start_node, grid)
}

type PathNode = (usize, usize, char);
enum State {
    Outside,
    Inside,
    WallTop,
    WallBottom
}
fn calculate_area_in_loop(start: PathNode, next: &Coordinate, grid: &Vec<Vec<Option<Node>>>) -> u64 {
    let mut path: Vec<(usize, usize, char)> = vec![start];
    let mut prev = start;
    let mut current = next;
    while let Some(node) = &grid[current.1][current.0] {
        path.push((current.0, current.1, node.2));
        let tmp = (current.0, current.1, node.2);
        current = if node.0 == (prev.0, prev.1) {&node.1} else {&node.0};
        prev = tmp;
    }

    path.sort_by_key(|coordinate| (coordinate.1, coordinate.0));

    let mut state = State::Outside;
    let mut count: usize = 0;
    let mut current_row: usize = 0;
    let mut left: usize = 0;

    for (x, y, c) in path {
        if current_row != y {
            current_row = y;
            state = match c {
                '|' => {
                    left = x;
                    State::Inside
                },
                'L' => State::WallBottom,
                'F' => State::WallTop,
                _ => panic!("Couldn't parse path in graph")
            };
            continue
        }

        match state {
            State::Outside => {
                state = match c {
                    '|' => {
                        left = x;
                        State::Inside
                    },
                    'L' => State::WallBottom,
                    'F' => State::WallTop,
                    _ => panic!("Couldn't parse path in graph")
                };
            },
            State::Inside => {
                count += x - left - 1;
                state = match c {
                    '|' => State::Outside,
                    'L' => State::WallTop,
                    'F' => State::WallBottom,
                    _ => panic!("Couldn't parse path in graph")
                };
            },
            State::WallTop => {
                state = match c {
                    'J' => {
                        left = x;
                        State::Inside
                    },
                    '7' => State::Outside,
                    '-' => state,
                    _ => panic!("Couldn't parse path in graph")
                }
            },
            State::WallBottom => {
                state = match c {
                    'J' => State::Outside,
                    '7' => {
                        left = x;
                        State::Inside
                    },
                    '-' => state,
                    _ => panic!("Couldn't parse path in graph")
                }
            }
        }
    }
    count as u64
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

fn define_start_node(start: &Coordinate, starting_directions: (Coordinate, Coordinate)) -> Node {
    let (x, y) = *start;
    let start_symbol = if starting_directions.0.0 == x && x == starting_directions.1.0 {
        '|'
    } else if starting_directions.0.1 == y && y == starting_directions.1.1 {
        '-'
    } else if starting_directions.0.0 < x || starting_directions.1.0 < x {
        if starting_directions.0.1 < y || starting_directions.1.1 < y {
            'J'
        } else {
            '7'
        }
    } else {
        if starting_directions.0.1 < y || starting_directions.1.1 < y {
            'L'
        } else {
            'F'
        }
    };

    (starting_directions.0, starting_directions.1, start_symbol)
}

fn find_starting_directions(grid: &Vec<Vec<Option<Node>>>, start: &Coordinate) -> (Coordinate, Coordinate) {
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
        '|' => if y > 0 {Some(((x, y - 1), (x, y + 1), *c))} else {None}, // is a vertical pipe connecting north and south
        '-' => if x > 0 {Some(((x - 1, y), (x + 1, y), *c))} else {None}, // is a horizontal pipe connecting east and west
        'L' => if y > 0 {Some(((x, y - 1), (x + 1, y), *c))} else {None}, // is a 90-degree bend connecting north and east
        'J' => if y > 0 && x > 0 {Some(((x, y - 1), (x - 1, y), *c))} else {None}, // is a 90-degree bend connecting north and west
        '7' => if x > 0 {Some(((x - 1, y), (x, y + 1), *c))} else {None}, // is a 90-degree bend connecting south and west
        'F' => Some(((x + 1, y), (x, y + 1), *c)), // is a 90-degree bend connecting south and east
        '.' => None, // is ground; there is no pipe in this tile
        'S' => None, // is the starting position of the animal
        _ => panic!("invalid character {c}")
    }
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
    fn test_part2_1() {
        let area = part2(Path::new("tests/part2_1.txt")).unwrap();
        assert_eq!(area, 4);
    }

    #[test]
    fn test_part2_2() {
        let area = part2(Path::new("tests/part2_2.txt")).unwrap();
        assert_eq!(area, 8);
    }

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
