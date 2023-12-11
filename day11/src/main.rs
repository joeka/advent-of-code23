use std::path::Path;
use aoc::{get_args, Part, exit_with_error};
use aoc::errors::AOCError;

fn main() {
    let options = get_args();

    let result: Result<u64, AOCError> = match options.part {
        Part::One => part1(&options.input),
        Part::Two => part2(&options.input)
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => exit_with_error(error)
    }
}


fn part1(_: &Path) -> Result<u64, AOCError> {
    Err(AOCError::from("Not implemented"))
}

fn part2(_: &Path) -> Result<u64, AOCError> {
    Err(AOCError::from("Not implemented"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let value = part1(Path::new("tests/input.txt")).unwrap();
        assert_eq!(value, 42);
    }

}
