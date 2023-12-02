use std::cmp::max;
use std::io::BufRead;
use std::{env, fmt, fs::File, io, path::Path, process};

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
        _ => Err(GameError::from("Invalid number of arguments"))
    };
    match result {
        Ok(sum) => println!("{sum}"),
        Err(error) => {
            eprintln!("{}", error);
            exit_with_usage();
        }
    }
}

fn part2(input_file: &Path) -> Result<u32, GameError> {
    sum_of_minimum_power(input_file)
}

fn part1(input_file: &Path, args: &Vec<String>)  -> Result<u32, GameError> {
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

fn sum_of_minimum_power(input_file: &Path) -> Result<u32, GameError> {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(GameError::from("Could not read the input file!")),
            };
            let game = match parse_game(&line) {
                Ok(game) => game,
                Err(error) => return Err(error),
            };
            sum += game.required().power();
        }
    }
    Ok(sum)
}

fn sum_of_possible_games(input_file: &Path, bag: &Cubes) -> Result<u32, GameError> {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(GameError::from("Could not read the input file!")),
            };
            let game = match parse_game(&line) {
                Ok(game) => game,
                Err(error) => return Err(error),
            };
            if game_possible(&game, &bag) {
                sum += game.id;
            }
        }
    }
    Ok(sum)
}

fn game_possible(game: &Game, bag: &Cubes) -> bool {
    for draw in &game.draws {
        if draw.red > bag.red || draw.green > bag.green || draw.blue > bag.blue {
            return false;
        }
    }
    true
}

fn parse_game(line: &str) -> Result<Game, GameError> {
    let parts: Vec<&str> = line.split(':').collect();
    let game_id_part = match parts[0].split(' ').last() {
        Some(part) => part,
        None => return Err(GameError::new(format!("Could not parse: {line}"))),
    };
    let game_id = match parse_game_id(game_id_part) {
        Ok(id) => id,
        Err(error) => return Err(error),
    };
    let draws: Result<Vec<Cubes>, GameError> = parts[1].split(';')
        .map(parse_draw)
        .collect();
    match draws {
        Ok(draws) => Ok(Game { id: game_id, draws }),
        Err(error) => Err(error),
    }
}

fn parse_draw(part: &str) -> Result<Cubes, GameError> {
    let mut draw = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };
    for cube_part in part.split(',').map(str::trim) {
        let parts: Vec<&str> = cube_part.split(' ').collect();
        let value = match parts[0].parse::<u32>() {
            Ok(value) => value,
            Err(_) => return Err(GameError::new(format!("'{}' is not a number.", parts[0])))
        };
        match parts[1] {
            "red" => draw.red = value,
            "green" => draw.green = value,
            "blue" => draw.blue = value,
            _ => return Err(GameError::new(format!("'{}' is not a valid cube color.", parts[1])))
        }
    }
    Ok(draw)
}

fn parse_game_id(part: &str) -> Result<u32, GameError> {
    let id_part = match part.split(' ').last() {
        Some(part) => part,
        None => return Err(GameError::new(format!("Can't parse game id from '{part}'"))),
    };
    match id_part.parse::<u32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(GameError::new(format!("Can't parse game id: '{id_part}' is not a number."))),
    }
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct GameError {
    message: String
}

impl GameError {
    fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl From<&str> for GameError {
    fn from(message: &str) -> Self {
        Self::new(String::from(message))
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
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
    fn test_parse_draw() {
        let draw = parse_draw("8 green, 6 blue, 20 red").unwrap();
        assert_eq!(draw.red, 20);
        assert_eq!(draw.green, 8);
        assert_eq!(draw.blue, 6);
    }

    #[test]
    fn test_parse_draw_with_zero() {
        let draw = parse_draw("8 green, 6 blue").unwrap();
        assert_eq!(draw.red, 0);
        assert_eq!(draw.green, 8);
        assert_eq!(draw.blue, 6);
    }

    #[test]
    fn test_parse_game() {
        let game = parse_game("Game 5: 1 red, 2 blue, 3 green; 2 blue, 2 green").unwrap();
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

        assert!(game_possible(&game, &Cubes { red: 5, green: 5, blue: 5}));
        assert!(!game_possible(&game, &Cubes { red: 3, green: 3, blue: 3}));
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
