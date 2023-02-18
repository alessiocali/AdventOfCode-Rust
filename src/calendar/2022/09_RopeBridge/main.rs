mod error;
mod geometry;

use advent_of_code::clamp;
use error::Error;
use geometry::{ Direction, Path, Point };
use regex::Regex;
use std::{ collections::HashSet, fs::File, io::{ BufRead, BufReader } };

#[derive(Clone)]
struct Rope {
    pub knots: Vec<Point>
}

impl Rope {
    fn new(knots_count: usize) -> Rope {
        assert!(knots_count >= 2);
        Rope { knots: vec![Point { x: 0, y: 0 }; knots_count]  }
    }

    fn tail<'a>(&'a self) -> &'a Point {
        self.knots.last().unwrap()
    }
}

fn main() {
    match read_input("inputs/2022/09/RopeBridge.txt") {
        Ok(path) => {
            let solution_1 = solve_problem(2, &path);
            let solution_2 = solve_problem(10, &path);
            println!("Solution 1: {solution_1}");
            println!("Solution 2: {solution_2}");
        },
        Err(err) => {
            println!("{err:?}");
        }
    }
}

fn read_input(path: &str) -> Result<Path, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_lines(reader.lines())
}

fn parse_lines<IterType, IterError>(iterator: IterType) -> Result<Path, Error> 
where IterType: Iterator<Item = Result<String, IterError>>
    , Error: From<IterError>
{
    let result: Result<Vec<_>, _> = iterator.map(|input_line| input_line.map_err(Error::from).and_then(parse_line)).collect();
    Ok(result?.into_iter().flatten().collect())
}

fn parse_line(line: String) -> Result<Vec<Direction>, Error> {
    lazy_static::lazy_static! {
        static ref DIRECTION: Result<Regex, regex::Error> = Regex::new(r"(?P<direction>L|R|U|D) (?P<amount>\d+)");
    }

    let direction_regex = DIRECTION.as_ref()?.to_owned();
    
    let captures = direction_regex.captures(&line).ok_or(Error::LineParsingError(line.clone()))?;
    let direction = captures.name("direction").ok_or(Error::LineParsingError(line.clone()))?.as_str();
    let amount = captures.name("amount").ok_or(Error::LineParsingError(line.clone()))?.as_str();

    let direction = Direction::try_from(direction)?;
    let amount = amount.parse::<usize>().map_err(|_| Error::LineParsingError(line.clone()))?;

    Ok(vec![direction; amount])
}

fn solve_problem(rope_size: usize, path: &Path) -> usize {
    follow_path(&mut Rope::new(rope_size), path).len()
}

fn follow_path(rope: &mut Rope, path: &Path) -> HashSet<Point> {
    let mut visited: HashSet<Point> = HashSet::new();

    visited.insert(rope.tail().clone());
    for direction in path.iter() {
        advance(rope, direction);
        visited.insert(rope.tail().clone());
    }

    visited
}

fn advance(rope: &mut Rope, direction: &Direction) {
    let mut iter = rope.knots.iter_mut();
    let mut current = iter.next().unwrap();

    // Advance head
    *current = *current + direction.value();
    for next in iter {
        let diff = *current - *next;
        
        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            let normalized_diff = Point { 
                x: clamp(diff.x, -1, 1),
                y: clamp(diff.y, -1, 1) 
            };
            *next = *next + normalized_diff;
        }

        current = next;
    } 
}