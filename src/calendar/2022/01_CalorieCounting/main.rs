use std::{
    fs::File,
    io::{ BufReader, BufRead},
};

use itertools::Itertools;

fn parse_file(file_path: &str) -> Result<(i32, i32), String> {
    let input_file: File = File::open(file_path).map_err(|e| e.to_string())?;

    let top_three: Vec<i32> = BufReader::new(input_file)
        .lines()
        .filter_map(|line| line.ok())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, load)| {
            if is_empty { None }
            else { Some(load) }
        })
        .map(|load| load.filter_map(|line| line.parse::<i32>().ok()).sum())
        .sorted()
        .rev()
        .take(3)
        .collect_vec();

    let top_carrier: i32 = *top_three.get(0).ok_or(String::from("No carrier could be found"))?;
    let top_three_sum: i32 = if top_three.len() == 3 { 
        Ok(top_three.iter().sum()) 
    } 
    else {
        Err("Less than three carriers were found.")
    }?;

    Ok((top_carrier, top_three_sum))
}

pub fn main() {
    let results = parse_file("inputs/2022/01/CalorieCounting.txt");

    match results {
        Ok((result_1, result_2)) => {
            println!("Result 1: {result_1}");
            println!("Result 2: {result_2}");
        },
        Err(err) => println!("{err}"),
    }
}