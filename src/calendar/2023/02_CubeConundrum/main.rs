use lazy_static::lazy_static;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

lazy_static! {
    static ref REG_GAME: Regex = Regex::new(r"^Game (?<game_id>\d+): (?<game_string>.*)$").unwrap();
    static ref REG_CUBE_SET: Regex = Regex::new(r"(?<count>\d+) (?<color>red|green|blue)").unwrap();
}

#[derive(Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32
}

impl CubeSet {
    fn is_empty(&self) -> bool {
        return self.red == 0 && self.green == 0 && self.blue == 0;
    }
}

struct Game {
    id: u32,
    sets: Vec<CubeSet>
}

#[derive(thiserror::Error, Debug)]
enum ParsingError {
    #[error("Failed to parse game line: {0}")]
    Game(String),
    #[error("Failed to parse game id: {0}")]
    Id(String),
    #[error("Failed to parse cube set: {0}")]
    CubeSet(String)
}

fn parse_game_line(line: &str) -> Result<Game, ParsingError> {
    let parse_result = REG_GAME.captures(line).ok_or(ParsingError::Game(line.to_string()))?;
    let game_id: u32 = parse_result.name("game_id").and_then(|id_capture| id_capture.as_str().parse().ok()).ok_or(ParsingError::Id(line.to_string()))?;
    let game_sets_string = parse_result.name("game_string").map_or("", |str_match| str_match.as_str());

    let mut cube_sets: Vec<CubeSet> = vec![];

    for set_line in game_sets_string.split(";") {
        let mut cube_set = CubeSet::default();

        for cube_set_handful_match in REG_CUBE_SET.captures_iter(set_line) {
            let make_set_error = || ParsingError::CubeSet(cube_set_handful_match.get(0).unwrap().as_str().to_string());
            let count: u32 = cube_set_handful_match.name("count").and_then(|count| count.as_str().parse().ok()).ok_or(make_set_error())?;
            let color: &str = cube_set_handful_match.name("color").ok_or(make_set_error())?.as_str();
            match color {
                "red" => cube_set.red = count,
                "green" => cube_set.green = count,
                "blue" => cube_set.blue = count,
                _ => { return Err(make_set_error()); }
            }
        }

        if !cube_set.is_empty() {
            cube_sets.push(cube_set);
        }
    }

    Ok(Game { id: game_id, sets: cube_sets })
}

fn main() {
    let file = File::open("inputs/2023/02/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|lr| lr.ok())
        .collect::<Vec<_>>();

    let games = lines
        .iter()
        .filter_map(|line| match parse_game_line(&line) {
            Ok(game) => Some(game),
            Err(err) => { println!("{err:?}"); None }
        })
        .collect::<Vec<_>>();

    let game_1_solution = games
        .iter()
        .filter(|game| game.sets.iter().all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14))
        .map(|game| game.id)
        .sum::<u32>();

    let game_2_solution = games
        .iter()
        .map(|game| CubeSet {
            red: game.sets.iter().map(|set| set.red).max().unwrap_or_default(),
            green: game.sets.iter().map(|set| set.green).max().unwrap_or_default(),
            blue: game.sets.iter().map(|set| set.blue).max().unwrap_or_default()
        })
        .map(|minimal_set| minimal_set.red * minimal_set.green * minimal_set.blue)
        .sum::<u32>();

    println!("Solution 1 : {game_1_solution}");
    println!("Solution 2 : {game_2_solution}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game_id() {
        let game = parse_game_line("Game 123: ---").unwrap();
        assert_eq!(game.id, 123);
    }

    #[test]
    fn test_cube_set() {
        let game = parse_game_line("Game 1: 1 red, 2 green, 3 blue").unwrap();
        assert_eq!(game.sets.len(), 1);
        assert_eq!(game.sets[0].red, 1);
        assert_eq!(game.sets[0].green, 2);
        assert_eq!(game.sets[0].blue, 3);
    }

    #[test]
    fn test_multiple_cube_sets() {
        let game = parse_game_line("Game 1: 1 red; 2 green; 3 blue").unwrap();
        assert_eq!(game.sets.len(), 3);
        assert_eq!(game.sets[0].red, 1);
        assert_eq!(game.sets[1].green, 2);
        assert_eq!(game.sets[2].blue, 3);

    }
}