use std::path::PathBuf;
use std::usize;

use aoc::{get_args, Part, exit_with_error, get_input_buffer};
use aoc::errors::AOCError;

fn main() {
    let options = get_args();

    let result: Result<usize, AOCError> = match options.part {
        Part::One => galaxy_distances(&options.input, 2),
        Part::Two => galaxy_distances(&options.input, 1_000_000)
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => exit_with_error(error)
    }
}

fn galaxy_distances(input_file: &PathBuf, expanse: usize) -> Result<usize, AOCError> {
    let mut distance: usize = 0;

    let mut columns: Vec<Vec<Coordinate>> = Vec::new();

    let mut y: usize = 0;
    for line in get_input_buffer(input_file) {
        if let Ok(line) = line {
            let mut x: usize = 0;
            let mut empty_line = true;
            for c in line.chars() {
                if columns.len() <= x {
                    columns.push(Vec::new());
                }
                if c == '#' {
                    columns[x].push(Coordinate{ x, y });
                    empty_line = false;
                }
                x += 1;
            }
            y += if empty_line {expanse} else {1};
        }
    }
    let mut offset = 0;
    for column in columns.iter_mut() {
        if column.is_empty() {
            offset += expanse - 1;
        } else if offset > 0 {
            for galaxy in column.iter_mut() {
                galaxy.x += offset;
            }
        }
    }
    for (i, a) in columns.iter().flatten().enumerate() {
        for b in columns.iter().flatten().skip(1 + i) {
            let dist = a.dist(b);
            distance += dist;
        }
    }

    Ok(distance)
}

#[derive(Clone, PartialEq, Debug)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    pub fn dist(&self, other: &Coordinate) -> usize {
        if self == other {
            0
        } else {
            let x_diff = if self.x > other.x {
                self.x - other.x
            } else {
                other.x - self.x
            };
            let y_diff = if self.y > other.y {
                self.y - other.y
            } else {
                other.y - self.y
            };
            x_diff + y_diff
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let value = galaxy_distances(&PathBuf::from("tests/input.txt"), 2).unwrap();
        assert_eq!(value, 374);
    }

    #[test]
    fn test_100() {
        let value = galaxy_distances(&PathBuf::from("tests/input.txt"), 100).unwrap();
        assert_eq!(value, 8410);
    }
}
