use std::path::PathBuf;

use aoc::errors::AOCError;
use aoc::{get_args, Part, exit_with_error, get_input_buffer};

static RADIX: u32 = 10;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five
}

struct Hand {
    value: u64,
    bet: u64
}

impl Hand {
    fn new(cards: &str, bet: u64, with_joker: bool) -> Self {
        let cards: Vec<char> = cards.chars().collect::<Vec<char>>();
        let hand_type = get_hand_type(&cards, with_joker);
        Self {
            value: calculate_value(cards, hand_type, with_joker),
            bet: bet
        }
    }
}

fn calculate_value(cards: Vec<char>, hand_type: HandType, with_joker: bool) -> u64 {
    let mut value: u64 = (hand_type as u64) << (5 * 8);
    for (i, card) in cards.iter().enumerate() {
        value |= get_card_value(card, with_joker) << (8 * (4 - i))
    }
    value
}

fn get_card_value(c: &char, with_joker: bool) -> u64 {
    match c {
        'A' => 15,
        'K' => 14,
        'Q' => 13,
        'J' => if with_joker {1} else {12},
        'T' => 11,
        _ => c.to_digit(RADIX).expect("invalid card symbol") as u64
    }
}

fn get_hand_type(cards: &Vec<char>, with_joker: bool) -> HandType {
    let mut sorted = cards.clone();
    sorted.sort();

    let mut hand_type: HandType = HandType::HighCard;
    let mut previous: char = '?';
    let mut row = 0;
    let mut jokers: i32 = 0;
    for c in sorted {
        if previous == c {
            row += 1;
        } else {
            hand_type = match row {
                4 => HandType::Four,
                3 => HandType::Three,
                2 => if hand_type == HandType::Pair {HandType::TwoPairs} else {HandType::Pair}
                _ => hand_type
            };
            if with_joker && c == 'J' {
                jokers += 1;
                previous = '?';
                row = 0;
            } else {
                row = 1;
                previous = c;
            }
        }
    }
    hand_type = match row {
        5 => HandType::Five,
        4 => HandType::Four,
        3 => if hand_type == HandType::Pair {HandType::FullHouse} else {HandType::Three}
        2 => match hand_type {
            HandType::Three => HandType::FullHouse,
            HandType::Pair => HandType::TwoPairs,
            _ => HandType::Pair
        },
        _ => hand_type
    };
    if with_joker && jokers > 0 {
        match jokers {
            j if j > 3 => HandType::Five,
            3 => if hand_type == HandType::Pair {HandType::Five} else {HandType::Four},
            2 => match hand_type {
                HandType::Three => HandType::Five,
                HandType::Pair => HandType::Four,
                _ => HandType::Three
            },
            1 => match hand_type {
                HandType::Four => HandType::Five,
                HandType::Three => HandType::Four,
                HandType::TwoPairs => HandType::FullHouse,
                HandType::Pair => HandType::Three,
                _ => HandType::Pair
            },
            _ => hand_type
        }
    } else {
        hand_type
    }
}

fn main() {
    let options = get_args();

    let result: Result<u64, AOCError> = match options.part {
        Part::One => total_winnings(&options.input, false),
        Part::Two => total_winnings(&options.input, true)
    };

    match result {
        Ok(result) => println!("{result}"),
        Err(error) => exit_with_error(error)
    }
}

fn total_winnings(input_file: &PathBuf, with_joker: bool) -> Result<u64, AOCError> {
    let mut hands: Vec<Hand> = Vec::new();
    let lines = get_input_buffer(input_file);
    for line in lines {
        if let Ok(line) = line {
            let (cards, bet) = line.split_once(' ').unwrap();
            hands.push(Hand::new(cards, bet.parse::<u64>().unwrap(), with_joker))
        }
    } 

    hands.sort_by_key(|hand| hand.value);

    let mut result: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += (i as u64 + 1) * hand.bet;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_problems() {
        assert_eq!(get_hand_type(&vec!['J', '9', '9', 'T', 'T'], true), HandType::FullHouse);
    }

    #[test]
    fn test_get_hand_type_part2() {
        assert_eq!(get_hand_type(&"32T3K".chars().collect(), true), HandType::Pair);
        assert_eq!(get_hand_type(&"T55J5".chars().collect(), true), HandType::Four);
        assert_eq!(get_hand_type(&"KK677".chars().collect(), true), HandType::TwoPairs);
        assert_eq!(get_hand_type(&"KTJJT".chars().collect(), true), HandType::Four);
        assert_eq!(get_hand_type(&"QQQJA".chars().collect(), true), HandType::Four);
    }

    #[test]
    fn test_total_winnings_part2() {
        let location = total_winnings(&PathBuf::from("tests/input.txt"), true).unwrap();
        assert_eq!(location, 5905);
    }

    #[test]
    fn test_get_hand_type() {
        assert_eq!(get_hand_type(&"AAAAA".chars().collect(), false), HandType::Five);
        assert_eq!(get_hand_type(&"AA8AA".chars().collect(), false), HandType::Four);
        assert_eq!(get_hand_type(&"23332".chars().collect(), false), HandType::FullHouse);
        assert_eq!(get_hand_type(&"TTT98".chars().collect(), false), HandType::Three);
        assert_eq!(get_hand_type(&"23432".chars().collect(), false), HandType::TwoPairs);
        assert_eq!(get_hand_type(&"A23A4".chars().collect(), false), HandType::Pair);
        assert_eq!(get_hand_type(&"23456".chars().collect(), false), HandType::HighCard);
    }

    #[test]
    fn test_total_winnings() {
        let location = total_winnings(&PathBuf::from("tests/input.txt"), false).unwrap();
        assert_eq!(location, 6440);
    }
}
