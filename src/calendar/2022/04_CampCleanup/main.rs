use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{ fs::File, io::{BufRead, BufReader} };

#[derive(Debug)]
enum Error { IoError, InvalidRange(i32, i32), ParsingError, RegexError(regex::Error) }

struct Range { 
    min: i32,
    max: i32
}

impl Range {
    fn new(min: i32, max: i32) -> Result<Range, Error> {
        if min > 0 && max > 0 && max >= min {
            Ok(Range { min, max })
        }
        else {
            Err(Error::InvalidRange(min, max))
        }
    }

    fn is_contained_or_contains(&self, other: &Range) -> bool {
        (other.min <= self.min && self.max <= other.max) ||
        (self.min <= other.min && other.max <= self.max) 
    }

    fn overlaps_with(&self, other: &Range) -> bool {
        (self.min <= other.min && other.min <= self.max) ||
        (other.min <= self.min && self.min <= other.max)
    }
}

fn parse_line(line: &str) -> Result<(Range, Range), Error> {
    lazy_static! { 
        static ref REG: Result<Regex, regex::Error> = Regex::new(r"(\d+)\-(\d+),(\d+)\-(\d+)");
    }

    let unwrapped_regex = REG.as_ref().map_err(|e| Error::RegexError(e.clone()))?;
    let captures = unwrapped_regex.captures(line).ok_or(Error::ParsingError)?;
    
    let (min1, max1, min2, max2) = captures.iter()
        .skip(1)
        .take(4)
        .map(|id| id.and_then(|regex_match| regex_match.as_str().parse::<i32>().ok()))
        .flatten()
        .collect_tuple()
        .ok_or(Error::ParsingError)?;

    let range1 = Range::new(min1, max1)?;
    let range2 = Range::new(min2, max2)?;
    Ok((range1, range2))
}

fn parse_input(input_path: &str) -> Result<(i32, i32), Error> {
    let input_file = File::open(input_path).unwrap();
    let input_lines: Vec<_> = BufReader::new(input_file).lines()
        .try_collect()
        .map_err(|_| Error::IoError)?;
    
    let range_pairs: Vec<(Range, Range)> = input_lines.into_iter()
        .map(|line| parse_line(line.as_str()))
        .try_collect()?;

    let contained_ranges = range_pairs.iter()
        .filter(|(range1, range2)| range1.is_contained_or_contains(range2))
        .count() as i32;

    let overlapping_ranges = range_pairs.iter()
        .filter(|(range1, range2)| range1.overlaps_with(range2))
        .count() as i32;

    Ok((contained_ranges, overlapping_ranges))
}

fn main() {
    let result = parse_input("inputs/2022/04/CampCleanup.txt");
    match result {
        Ok((contained, overlapping)) => {
            println!("Contained ranges: {contained}");
            println!("Overlapping ranges: {overlapping}");
        },
        Err(e) => {
            println!("{e:?}");
        }
    }
}