use std::cmp::max;
use std::io::BufRead;
use std::str::FromStr;
use std::{env, fs::File, io, path::Path, process};

use aoc::errors::AOCError;

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Cubes {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        for cube_part in s.split(',').map(str::trim) {
            let (number, color) = cube_part.split_once(' ')
                .ok_or(AOCError::new(format!("Can't parse '{s}' into Cubes.")))?;
            let value = match number.parse::<u32>() {
                Ok(value) => value,
                Err(_) => return Err(AOCError::new(format!("'{}' is not a number.", number)))
            };
            match color {
                "red" => cubes.red = value,
                "green" => cubes.green = value,
                "blue" => cubes.blue = value,
                _ => return Err(AOCError::new(format!("'{}' is not a valid cube color.", color)))
            }
        }
        Ok(cubes)
    }
}

struct Game {
    id: u32,
    draws: Vec<Cubes>,
}

impl Game {
    fn required(&self) -> Cubes {
        let mut required = Cubes { red: 0, green: 0, blue: 0 };
        for draw in &self.draws {
            required.red = max(required.red, draw.red);
            required.green = max(required.green, draw.green);
            required.blue = max(required.blue, draw.blue);
        }
        required
    }

    fn possible(&self, bag: &Cubes) -> bool {
        for draw in &self.draws {
            if draw.red > bag.red || draw.green > bag.green || draw.blue > bag.blue {
                return false;
            }
        }
        true
    }
}

impl FromStr for Game {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id_part, draws_part) = s.split_once(':')
            .ok_or(AOCError::new(format!("Could not parse: {s}")))?;
        let game_id = parse_game_id(game_id_part)?;
        let draws: Result<Vec<Cubes>, AOCError> = draws_part.split(';')
            .map(Cubes::from_str)
            .collect();
        match draws {
            Ok(draws) => Ok(Game { id: game_id, draws }),
            Err(error) => Err(error),
        }
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
        2 => part2(input_path),
        5 => part1(input_path, &args),
        _ => Err(AOCError::from("Invalid number of arguments"))
    };
    match result {
        Ok(sum) => println!("{sum}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

fn part2(input_file: &Path) -> Result<u32, AOCError> {
    sum_of_minimum_power(input_file)
}

fn part1(input_file: &Path, args: &Vec<String>)  -> Result<u32, AOCError> {
    let mut bag = Cubes { red: 0, green: 0, blue: 0};
    match args[2].parse::<u32>() {
        Ok(red) => bag.red = red,
        Err(_) => exit_with_usage()
    };
    match args[3].parse::<u32>() {
        Ok(green) => bag.green = green,
        Err(_) => exit_with_usage()
    };
    match args[4].parse::<u32>() {
        Ok(blue) => bag.blue = blue,
        Err(_) => exit_with_usage()
    };
    sum_of_possible_games(input_file, &bag)
}

fn exit_with_usage() {
    println!("Usage: day2 INPUT_FILE RED GREEN BLUE");
    println!("Part2: day2 INPUT_FILE");
    process::exit(1);
}

fn sum_of_minimum_power(input_file: &Path) -> Result<u32, AOCError> {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            let game = game_from_line(line)?;
            sum += game.required().power();
        }
    }
    Ok(sum)
}

fn sum_of_possible_games(input_file: &Path, bag: &Cubes) -> Result<u32, AOCError> {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            let game = game_from_line(line)?;
            if game.possible(&bag) {
                sum += game.id;
            }
        }
    }
    Ok(sum)
}

fn game_from_line(line: Result<String, io::Error>) -> Result<Game, AOCError> {
    let line = match line {
        Ok(line) => line,
        Err(_) => return Err(AOCError::from("Could not read the input file!")),
    };
    Game::from_str(&line)
}

fn parse_game_id(part: &str) -> Result<u32, AOCError> {
    let id_part = match part.split_once(' ') {
        Some(part) => part.1,
        None => return Err(AOCError::new(format!("Can't parse game id from '{part}'"))),
    };
    match id_part.parse::<u32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(AOCError::new(format!("Can't parse game id: '{id_part}' is not a number."))),
    }
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubes_power() {
        let cubes = Cubes { red: 2, green: 3, blue: 4 };
        assert_eq!(cubes.power(), 24);
    }

    #[test]
    fn test_game_required() {
        let game = Game { id: 1, draws: vec![
            Cubes { red: 1, green: 5, blue: 4 },
            Cubes { red: 2, green: 3, blue: 4 }
        ]};
        assert_eq!(game.required(), Cubes { red: 2, green: 5, blue: 4 });
    }

    #[test]
    fn test_sum_of_minimum_power() {
        let sum = sum_of_minimum_power(Path::new("tests/input.txt")).unwrap();
        assert_eq!(sum, 2286);
    }

    #[test]
    fn test_parse_game_id() {
        let game_id = parse_game_id("Game 42").unwrap();
        assert_eq!(game_id, 42);
    }

    #[test]
    fn test_cubes_from_str() {
        let cubes = Cubes::from_str("8 green, 6 blue, 20 red").unwrap();
        assert_eq!(cubes.red, 20);
        assert_eq!(cubes.green, 8);
        assert_eq!(cubes.blue, 6);
    }

    #[test]
    fn test_cubes_from_str_with_zero() {
        let draw = Cubes::from_str("8 green, 6 blue").unwrap();
        assert_eq!(draw.red, 0);
        assert_eq!(draw.green, 8);
        assert_eq!(draw.blue, 6);
    }

    #[test]
    fn test_game_from_str() {
        let game = Game::from_str("Game 5: 1 red, 2 blue, 3 green; 2 blue, 2 green").unwrap();
        assert_eq!(game.id, 5);
        assert_eq!(game.draws.len(), 2);
        assert_eq!(game.draws[0], Cubes { red: 1, green: 3, blue: 2});
        assert_eq!(game.draws[1], Cubes { red: 0, green: 2, blue: 2});
    }

    #[test]
    fn test_game_possible() {
        let game = Game { id: 1, draws: vec![
            Cubes {red: 1, green: 2, blue: 3},
            Cubes {red: 2, green: 3, blue: 4}
        ]};

        assert!(game.possible(&Cubes { red: 5, green: 5, blue: 5}));
        assert!(!game.possible(&Cubes { red: 3, green: 3, blue: 3}));
    }

    #[test]
    fn test_sum_of_possible_games() {
        let result = sum_of_possible_games(
            Path::new("./tests/input.txt"),
            &Cubes { red: 12, green: 13, blue: 14 }
        );
        assert_eq!(result.unwrap(), 8);
    }


    impl PartialEq for Cubes {
        fn eq(&self, other: &Self) -> bool {
            self.red == other.red && self.green == other.green && self.blue == other.blue
        }
    }
}
