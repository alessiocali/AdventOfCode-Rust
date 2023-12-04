use std::collections::HashSet;
use std::fs::File;
use std::io::{ BufReader, BufRead };

fn get_winning_numbers_count(line: &str) -> u32 {
    let mut line_split = line.split(":");
    let _game_id = line_split.next().unwrap();
    let mut number_string_split = line_split.next().unwrap().split("|");
    let winning_numbers_string = number_string_split.next().unwrap().trim();
    let your_numbers_string = number_string_split.next().unwrap().trim();

    let winning_numbers = winning_numbers_string
        .split(" ")
        .filter_map(|number_string| number_string.parse::<u32>().ok())
        .collect::<HashSet<_>>();

    your_numbers_string
        .split(" ")
        .filter_map(|number_string| number_string.parse::<u32>().ok())
        .filter(|number| winning_numbers.contains(&number))
        .count() as u32
}

fn get_score_from_win_count(win_count: u32) -> u32 {
    if win_count > 0u32 { 1u32 << (win_count - 1) } 
    else { 0 }
}

fn get_total_cards_count(winning_numbers_counts: &[u32]) -> Vec<u32> {
    let mut card_counts = vec![1u32; winning_numbers_counts.len()];
    
    for (idx, winning_count) in winning_numbers_counts.iter().enumerate() {
        let my_count = card_counts[idx];
        let next_idx = idx + 1;
        for clone_card_idx in next_idx..(next_idx + *winning_count as usize) {
            if let Some(clone_card_count) = card_counts.get_mut(clone_card_idx) {
                *clone_card_count += my_count;
            }
        }
    };

    card_counts
}

fn main() {
    let file = File::open("inputs/2023/04/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|line_result| line_result.ok())
        .collect::<Vec<_>>();

    let winning_numbers_counts = lines.iter().map(|line| get_winning_numbers_count(&line)).collect::<Vec<_>>();
    let solution_1 = winning_numbers_counts.iter().map(|winning_numbers_count| get_score_from_win_count(*winning_numbers_count)).sum::<u32>();
    let solution_2 = get_total_cards_count(&winning_numbers_counts).iter().sum::<u32>();

    println!("Solution 1: {solution_1}");
    println!("Solution 2: {solution_2}");
}