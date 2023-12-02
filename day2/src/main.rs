use std::io::BufRead;
use std::{env, fmt, fs::File, io, path::Path, process};

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    draws: Vec<Cubes>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Usage: {} INPUT_FILE RED GREEN BLUE", args[0]);
        process::exit(1);
    }
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

    match sum_of_possible_games(Path::new(&args[1]), &bag) {
        Ok(sum) => println!("{sum}"),
        Err(_) => {
            exit_with_usage();
        }
    }
}

fn exit_with_usage() {
    println!("Usage: day2 INPUT_FILE RED GREEN BLUE");
    process::exit(1);
}

fn sum_of_possible_games(input_file: &Path, bag: &Cubes) -> Result<u32, GameError> {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(GameError),
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
        None => return Err(GameError),
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
    part.split(',').map(str::trim).for_each(|s| {
        let parts: Vec<&str> = s.split(' ').collect();
        let value = parts[0]
            .parse::<u32>()
            .expect("value should represent a number");
        match parts[1] {
            "red" => draw.red = value,
            "green" => draw.green = value,
            "blue" => draw.blue = value,
            _ => panic!("invalid value"),
        }
    });
    Ok(draw)
}

fn parse_game_id(part: &str) -> Result<u32, GameError> {
    let id_part = match part.split(' ').last() {
        Some(part) => part,
        None => return Err(GameError),
    };
    match id_part.parse::<u32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(GameError),
    }
}

fn read_lines(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
struct GameError;

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "couldn't solve the game")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
