use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader}
};

#[derive(PartialEq, Debug)]
enum Error { FileDecoding, NoInputFile, Parsing, Regex(regex::Error) }

#[derive(PartialEq, Debug, Clone, Copy)]
enum LeftHandCypher { A, B, C }
#[derive(PartialEq, Debug, Clone, Copy)]
enum RightHandCypher { X, Y, Z }

#[derive(PartialEq, Debug, Clone, Copy)]
enum Shape { Rock, Paper, Scissors }
#[derive(PartialEq, Debug, Clone, Copy)]
enum Outcome { Win, Loss, Draw }

impl LeftHandCypher {
    fn from_input(input: &str) -> Option<LeftHandCypher> {
        match input.chars().nth(0) {
            Some('A') => Some(LeftHandCypher::A),
            Some('B') => Some(LeftHandCypher::B),
            Some('C') => Some(LeftHandCypher::C),
            _ => None
        }
    }
}

impl RightHandCypher {
    fn from_input(input: &str) -> Option<RightHandCypher> {
        match input.chars().nth(0) {
            Some('X') => Some(RightHandCypher::X),
            Some('Y') => Some(RightHandCypher::Y),
            Some('Z') => Some(RightHandCypher::Z),
            _ => None
        }
    }
}

impl Shape {
    fn get_score(self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }
}

impl From<LeftHandCypher> for Shape {
    fn from(left_hand: LeftHandCypher) -> Self {
        match left_hand {
            LeftHandCypher::A => Shape::Rock,
            LeftHandCypher::B => Shape::Paper,
            LeftHandCypher::C => Shape::Scissors,
        }
    }
}

impl From<RightHandCypher> for Shape {
    fn from(right_hand: RightHandCypher) -> Self {
        match right_hand {
            RightHandCypher::X => Shape::Rock,
            RightHandCypher::Y => Shape::Paper,
            RightHandCypher::Z => Shape::Scissors,
        }
    }
}

impl Outcome {
    fn get_score(self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0
        }
    }
}

impl From<RightHandCypher> for Outcome {
    fn from(right_hand: RightHandCypher) -> Self {
        match right_hand {
            RightHandCypher::X => Outcome::Loss,
            RightHandCypher::Y => Outcome::Draw,
            RightHandCypher::Z => Outcome::Win,
        }
    }
}

fn parse_cypher(input_line: &String) -> Result<(LeftHandCypher, RightHandCypher), Error> {
    let regex = Regex::new(r"(?P<left_hand>A|B|C) (?P<right_hand>X|Y|Z)").map_err(|e| Error::Regex(e))?;
    
    regex.captures(&input_line).and_then(|capture| {
        let left_hand = capture.name("left_hand").and_then(|group| LeftHandCypher::from_input(group.as_str()));
        let right_hand = capture.name("right_hand").and_then(|group| RightHandCypher::from_input(group.as_str()));

        match (left_hand, right_hand) {
            (Some(opponent), Some(own)) => Some((opponent, own)),
            _ => None
        }
    })
    .ok_or(Error::Parsing)
}

fn get_score(own: Shape, opponent: Shape) -> i32 {
    let outcome = match own {
        Shape::Rock => match opponent {
            Shape::Rock => Outcome::Draw,
            Shape::Paper => Outcome::Loss,
            Shape::Scissors => Outcome::Win
        },
        Shape::Paper => match opponent {
            Shape::Rock => Outcome::Win,
            Shape::Paper => Outcome::Draw,
            Shape::Scissors => Outcome::Loss
        },
        Shape::Scissors => match opponent {
            Shape::Rock => Outcome::Loss,
            Shape::Paper => Outcome::Win,
            Shape::Scissors => Outcome::Draw
        }
    };

    own.get_score() + outcome.get_score()
}

fn deduce_own_from_other_outcome(other: Shape, outcome: Outcome) -> Shape {
    match other {
        Shape::Rock => match outcome {
            Outcome::Loss => Shape::Scissors,
            Outcome::Draw => Shape::Rock,
            Outcome::Win => Shape::Paper
        },
        Shape::Paper => match outcome {
            Outcome::Loss => Shape::Rock,
            Outcome::Draw => Shape::Paper,
            Outcome::Win => Shape::Scissors
        },
        Shape::Scissors => match outcome {
            Outcome::Loss => Shape::Paper,
            Outcome::Draw => Shape::Scissors,
            Outcome::Win => Shape::Rock
        }
    }
}

fn parse_file(file_path: &str) -> Result<(i32, i32), Error> {
    let file = File::open(file_path).map_err(|_| Error::NoInputFile)?;
    let input_lines = BufReader::new(file).lines().collect::<Result<Vec<String>, _>>().map_err(|_| Error::FileDecoding)?;
    let input_cyphers = input_lines.into_iter().map(|line| parse_cypher(&line)).collect::<Result<Vec<(LeftHandCypher, RightHandCypher)>, _>>()?;

    let first_interpretation = input_cyphers.iter()
        .map(|(left_hand, right_hand)| (Shape::from(*left_hand), Shape::from(*right_hand)) )
        .map(|(other, own)| get_score(own, other))
        .sum();

    let second_interpretation = input_cyphers.iter()
        .map(|(left_hand, right_hand)| (Shape::from(*left_hand), Outcome::from(*right_hand)) )
        .map(|(other, outcome)| get_score(deduce_own_from_other_outcome(other, outcome), other))
        .sum();

    Ok((first_interpretation, second_interpretation))
}

fn main() {
    let result = parse_file("inputs/2022/02/RockPaperScissors.txt");

    match result {
        Ok((first_interpretation, second_interpretation)) => {
            println!("Total Score 1: {first_interpretation}");
            println!("Total Score 2: {second_interpretation}");
        },
        Err(err) => {
            println!("{err:#?}")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_left_hand_cypher() {
        let (left_hand, _) = parse_cypher(&String::from("A X")).unwrap();
        assert_eq!(left_hand, LeftHandCypher::A);

        let (left_hand, _) = parse_cypher(&String::from("B X")).unwrap();
        assert_eq!(left_hand, LeftHandCypher::B);

        let (left_hand, _) = parse_cypher(&String::from("C X")).unwrap();
        assert_eq!(left_hand, LeftHandCypher::C);
    }

    #[test]
    fn parse_right_hand_cypher() {
        let (_, right_hand) = parse_cypher(&String::from("A X")).unwrap();
        assert_eq!(right_hand, RightHandCypher::X);

        let (_, right_hand) = parse_cypher(&String::from("A Y")).unwrap();
        assert_eq!(right_hand, RightHandCypher::Y);

        let (_, right_hand) = parse_cypher(&String::from("A Z")).unwrap();
        assert_eq!(right_hand, RightHandCypher::Z);
    }

    #[test]
    fn parse_invalid_line() {
        let result = parse_cypher(&String::from("D X")).unwrap_err();
        assert_eq!(result, Error::Parsing);

        let result = parse_cypher(&String::from("A W")).unwrap_err();
        assert_eq!(result, Error::Parsing);

        let result = parse_cypher(&String::from("A  X")).unwrap_err();
        assert_eq!(result, Error::Parsing);

        let result = parse_cypher(&String::from("abcdefg")).unwrap_err();
        assert_eq!(result, Error::Parsing);

        let result = parse_cypher(&String::from("a x")).unwrap_err();
        assert_eq!(result, Error::Parsing);
    }

    #[test]
    fn compute_score() {
        assert_eq!(get_score(Shape::Rock, Shape::Rock), 4); // 1 + Draw
        assert_eq!(get_score(Shape::Rock, Shape::Paper), 1); // 1 + Loss
        assert_eq!(get_score(Shape::Rock, Shape::Scissors), 7); // 1 + Win

        assert_eq!(get_score(Shape::Paper, Shape::Rock), 8); // 2 + Win
        assert_eq!(get_score(Shape::Paper, Shape::Paper), 5); // 2 + Draw
        assert_eq!(get_score(Shape::Paper, Shape::Scissors), 2); // 2 + Loss
        
        assert_eq!(get_score(Shape::Scissors, Shape::Rock), 3); // 3 + Loss
        assert_eq!(get_score(Shape::Scissors, Shape::Paper), 9); // 3 + Win
        assert_eq!(get_score(Shape::Scissors, Shape::Scissors), 6); // 3 + Draw
    }

    #[test]
    fn test_deduce_own_from_other_outcome() {
        assert_eq!(deduce_own_from_other_outcome(Shape::Rock, Outcome::Loss), Shape::Scissors);
        assert_eq!(deduce_own_from_other_outcome(Shape::Rock, Outcome::Draw), Shape::Rock);
        assert_eq!(deduce_own_from_other_outcome(Shape::Rock, Outcome::Win), Shape::Paper);

        assert_eq!(deduce_own_from_other_outcome(Shape::Paper, Outcome::Loss), Shape::Rock);
        assert_eq!(deduce_own_from_other_outcome(Shape::Paper, Outcome::Draw), Shape::Paper);
        assert_eq!(deduce_own_from_other_outcome(Shape::Paper, Outcome::Win), Shape::Scissors);
        
        assert_eq!(deduce_own_from_other_outcome(Shape::Scissors, Outcome::Loss), Shape::Paper);
        assert_eq!(deduce_own_from_other_outcome(Shape::Scissors, Outcome::Draw), Shape::Scissors);
        assert_eq!(deduce_own_from_other_outcome(Shape::Scissors, Outcome::Win), Shape::Rock);
    }
}