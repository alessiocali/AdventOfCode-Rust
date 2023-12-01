use std::fs::File;
use std::io::{ BufReader, BufRead };
use regex::Regex;

#[derive(Copy, Clone)]
enum SearchType {
    DigitsOnly,
    DigitsAndLiterals
}

fn find_first_digit_in_range<Iter: Iterator<Item = usize>>(line: &str, search_type: SearchType, indices: Iter) -> Option<u32> {
    let regex = match search_type {
        SearchType::DigitsOnly => Regex::new(r"\d").unwrap(),
        SearchType::DigitsAndLiterals => Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap()
    };
    
    for idx in indices {
        if let Some(digit_match) = regex.find_at(line, idx) {
            let digit = match digit_match.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                digit => digit.parse::<u32>().unwrap()
            };

            return Some(digit);
        }
    }

    None
}

fn get_first_last_value(line: &str, search_type: SearchType) -> Option<(u32, u32)> {
    let first = find_first_digit_in_range(line, search_type, 0..line.len());
    let last = find_first_digit_in_range(line, search_type, (0..line.len()).rev());

    match (first, last) {
        (Some(first), Some(last)) => Some((first, last)),
        _ => None
    }
}

fn combine(first: u32, second: u32) -> u32 {
    format!("{first}{second}").parse::<u32>().unwrap()
}

fn solve<T, S>(range: T, search_type: SearchType) -> u32 where T: Iterator<Item = S>, S: AsRef<str>  {
    range
        .filter_map(|l| get_first_last_value(l.as_ref(), search_type))
        .map(|(first, last)| combine(first, last))
        .sum::<u32>()
}

fn main () {
    let input = File::open("inputs/2023/01/input.txt").unwrap();
    let lines = BufReader::new(input).lines().filter_map(|lr| lr.ok()).collect::<Vec<_>>();
    let result_1 = solve(lines.iter(), SearchType::DigitsOnly);
    let result_2 = solve(lines.iter(), SearchType::DigitsAndLiterals);
    println!("Solution 1: {result_1}");
    println!("Solution 2: {result_2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_number() {
        let result = get_first_last_value("abcdefg", SearchType::DigitsOnly);
        assert!(result.is_none());
    }

    #[test]
    fn test_two_numbers() {
        let result = get_first_last_value("12", SearchType::DigitsOnly).unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 2);
    }

    #[test]
    fn test_two_and_letters() { 
        let result = get_first_last_value("abc1defg2hilmn", SearchType::DigitsOnly).unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 2);
    }

    #[test]
    fn test_single_number() {
        let result = get_first_last_value("abcde1fghi", SearchType::DigitsOnly).unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 1);
    }

    #[test]
    fn test_literals() {
        let result = get_first_last_value("onetwo", SearchType::DigitsAndLiterals).unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 2);
    }

    #[test]
    fn test_overlapping_literals() {
        let result = get_first_last_value("eighthree", SearchType::DigitsAndLiterals).unwrap();
        assert_eq!(result.0, 8);
        assert_eq!(result.1, 3);
    }

    #[test]
    fn test_mixed_digit_literals() {
        let result = get_first_last_value("one2", SearchType::DigitsAndLiterals).unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 2);
    }

    #[test]
    fn test_combine_one_digit() {
        let result = combine(1, 2);
        assert_eq!(result, 12);
    }
}